use core::borrow;
use std::{collections::{HashMap}};

use blir::{attributes::AttributeFactory,
           code::{CodeBlock, ExternFunctionRef, FunctionRef, MethodRef, Statement, StatementKind},
           scope::{ScopeRef, ScopeRelation, ScopeType},
           typ::{StructRef, Type, TypeKind, EnumRef},
           value::{Closure, ClosureParam, ConstantRef, IfBranch, IfValue, Value, ValueKind, VarRef, GlobalVarRef},
           BlirContext, Library, Symbol, Visibility, pattern::{Pattern, PatternKind}};
use errors::{Span, DiagnosticReporter, IntoDiagnostic, Diagnostic, DiagnosticLevel, CodeLocation};
use parser::operators::{OperatorFactory, OperatorFix};

use crate::init_default::add_default_initializer;

/// Resolves types in recently lowered BLIR code
///
/// First, the parameters and return type of external functions are resolved
///
/// Next, types in each struct are resolved. This consists of a number of steps
///
/// - Types in substructs are resolved
/// - Types of fields are resolved
/// - Types of methods are resolved
/// - Attributes are applied to methods
/// - Attributes are applied to structs
///
/// Next, types in functions are resolved and attributes are applied
///
/// Finally, code in each function and method is resolved by the following process
///
/// - Each statement is stepped over
/// - Explicit types are resolved
/// - Each named value is resolved
///     - If the named value is a function, it will be *partially* resolved
///     - The value will be resolved to the set of functions which match
///           the labels and number of parameters
pub struct TypeResolvePass<'a, 'l> {
    factory:          &'a AttributeFactory,
    context:          &'a mut BlirContext,
    debugger:         &'a mut DiagnosticReporter<'l>,
    operator_factory: &'a OperatorFactory,
}

impl<'a, 'l> TypeResolvePass<'a, 'l> {
    pub fn new(factory: &'a AttributeFactory, operator_factory: &'a OperatorFactory, context: &'a mut BlirContext, debugger: &'a mut DiagnosticReporter<'l>) -> Self {
        Self { factory,
               context,
               debugger,
               operator_factory }
    }

    pub fn run_pass(&mut self, library: &mut Library) {
        // Run the pass on each extern function
        for extern_func in &library.extern_functions {
            // Resolve types in the extern function
            self.resolve_extern_func_types(extern_func);

            // Apply function attributes
        }

        for constant in &library.constants {
            self.resolve_constant(constant, library.scope());
        }

        for global in &library.globals {
            self.resolve_global(global, library.scope());
        }

        // Resolve types in each struct
        for r#struct in &library.structs {
            self.resolve_struct_types(r#struct);
        }

        for r#enum in &library.enums {
            self.resolve_enum_types(r#enum);
        }

        for r#struct in &library.structs {
            // Create a default initializer
            add_default_initializer(r#struct);
        }

        // Resolve code in each struct
        for r#struct in &library.structs {
            self.resolve_struct_code(r#struct);
        }

        for r#struct in &library.enums {
            self.resolve_enum_code(r#struct);
        }

        // Run the pass on each function
        for func in &library.functions {
            // Resolve types in the function
            self.resolve_func_types(func);

            // Set link name
            let mangled_name = func.borrow().mangle();
            func.borrow_mut().info.set_link_name(mangled_name);

            // Apply function attributes
            let mut func = func.borrow_mut();
            let attributes = func.attributes.clone();
            self.factory
                .apply_func_attributes(&attributes, &mut func.info, self.context, self.debugger)
        }

        for func in &library.functions {
            // Go through the function's code
            self.resolve_function_code(func);
        }
    }

    fn resolve_extern_func_types(&mut self, extern_func: &ExternFunctionRef) {
        let mut borrowed_func = extern_func.borrow_mut();
        let parent_scope = borrowed_func.parent.clone();

        // Resolve the return type
        self.resolve_type(borrowed_func.info.return_type_mut(), &parent_scope);

        // Resolve each parameter's type
        for param in borrowed_func.info.params_mut() {
            self.resolve_type(&mut param.typ, &parent_scope);
        }
    }

    fn resolve_enum_types(&mut self, r#enum: &EnumRef) {
        let scope = r#enum.scope();
        let mut tag_counter = 0;

        let mut used_tags: HashMap<usize, Vec<errors::Span>> = HashMap::new();

        for variant in r#enum.variants().iter() {
            if variant.has_tag() {
                tag_counter = variant.tag();
            }

            variant.set_tag(tag_counter);

            if used_tags.contains_key(&tag_counter) {
                used_tags.get_mut(&tag_counter)
                         .unwrap()
                         .push(variant.span());
            } else {
                let _ = used_tags.insert(tag_counter, vec![variant.span()]);
            }

            tag_counter += 1;

            variant.associated_types_mut()
                   .iter_mut()
                   .for_each(|typ| self.resolve_type(typ, &scope))
        }

        for (tag, spans) in used_tags {
            if spans.len() > 1 {
                self.debugger.throw_diagnostic(Error::TagAlreadyUsed(tag, spans));
            }
        }

        self.resolve_type(&mut *r#enum.repr_type_mut(), &scope);

        let ty = r#enum.repr_type().clone().kind;

        match ty {
            TypeKind::Integer { .. } => {}
            TypeKind::Struct(struct_ref) if struct_ref.integer_repr() => {
                let integer_type = struct_ref.borrow().instance_vars[0].borrow().typ.clone();

                r#enum.repr_type_mut().set_kind(integer_type.kind);
            }
            _ => {
                self.debugger.throw_diagnostic(Error::WrongEnumType(r#enum.repr_type().clone(), r#enum.repr_type().span().unwrap_or_else(Span::empty)));
            }
        }

        for substruct in r#enum.substructs().iter() {
            self.resolve_struct_types(substruct);
        }

        for subenum in r#enum.subenums().iter() {
            self.resolve_enum_types(subenum);
        }

        for method in r#enum.methods().iter() {
            // Resolve method types
            self.resolve_method_types(method);

            // Set link name
            let mangled_name = method.borrow().mangle();
            method.borrow_mut().info.set_link_name(mangled_name);

            // Apply function attributes
            let mut method = method.borrow_mut();
            let attributes = method.attributes.clone();
            self.factory
                .apply_func_attributes(&attributes, &mut method.info, self.context, self.debugger)
        }
    }

    fn resolve_struct_types(&mut self, r#struct: &StructRef) {
        let borrowed_struct = r#struct.borrow();
        let scope = borrowed_struct.scope();

        for substruct in &borrowed_struct.substructs {
            self.resolve_struct_types(substruct);
        }

        for subenum in &borrowed_struct.subenums {
            self.resolve_enum_types(subenum);
        }

        for constant in &borrowed_struct.constants {
            self.resolve_constant(constant, scope);
        }

        for variable in &borrowed_struct.instance_vars {
            self.resolve_variable(variable, scope);
        }

        for global in &borrowed_struct.globals {
            self.resolve_global(global, scope);
        }

        for method in &borrowed_struct.methods {
            // Resolve method types
            self.resolve_method_types(method);

            // Set link name
            let mangled_name = method.borrow().mangle();
            method.borrow_mut().info.set_link_name(mangled_name);

            // Apply function attributes
            let mut method = method.borrow_mut();
            let attributes = method.attributes.clone();
            self.factory
                .apply_func_attributes(&attributes, &mut method.info, self.context, self.debugger)
        }

        drop(borrowed_struct);

        self.factory.apply_struct_attributes(r#struct, self.context, self.debugger);
    }

    fn resolve_struct_code(&mut self, r#struct: &StructRef) {
        let r#struct = r#struct.borrow();

        for substruct in &r#struct.substructs {
            self.resolve_struct_code(substruct);
        }

        for subenum in &r#struct.subenums {
            self.resolve_enum_code(subenum);
        }

        for method in &r#struct.methods {
            self.resolve_method_code(method);
        }
    }

    fn resolve_enum_code(&mut self, r#enum: &EnumRef) {
        for substruct in r#enum.substructs().iter() {
            self.resolve_struct_code(substruct);
        }

        for subenum in r#enum.subenums().iter() {
            self.resolve_enum_code(subenum);
        }

        for method in r#enum.methods().iter() {
            self.resolve_method_code(method);
        }
    }

    fn resolve_variable(&mut self, var: &VarRef, scope: &ScopeRef) {
        self.resolve_type(&mut var.borrow_mut().typ, scope);

        if let Some(default_value) = &mut var.borrow_mut().default_value {
            self.resolve_value(default_value, scope);
        }
    }

    fn resolve_constant(&mut self, var: &ConstantRef, scope: &ScopeRef) {
        self.resolve_type(&mut var.borrow_mut().typ, scope);
        self.resolve_value(&mut var.borrow_mut().value, scope);
    }

    fn resolve_global(&mut self, var: &GlobalVarRef, scope: &ScopeRef) {
        self.resolve_type(&mut var.ty_mut(), scope);
        self.resolve_value(&mut var.default_value_mut(), scope);
    }

    fn resolve_func_types(&mut self, function: &FunctionRef) {
        let mut borrowed_func = function.borrow_mut();
        let parent_scope = borrowed_func.scope().clone();

        // Resolve the return type
        self.resolve_type(borrowed_func.info.return_type_mut(), &parent_scope);

        // Resolve each parameter's type
        for param in borrowed_func.info.params_mut() {
            self.resolve_type(&mut param.typ, &parent_scope);
        }

        // Add the functions parameters to its scope
        borrowed_func.add_params();
    }

    fn resolve_method_types(&mut self, method: &MethodRef) {
        let mut borrowed_func = method.borrow_mut();
        let parent_scope = borrowed_func.scope().clone();

        // Resolve the return type
        self.resolve_type(borrowed_func.info.return_type_mut(), &parent_scope);

        // Resolve each parameter's type
        for param in borrowed_func.info.params_mut() {
            self.resolve_type(&mut param.typ, &parent_scope);
        }

        // Add the functions parameters to its scope
        borrowed_func.add_params();

        if borrowed_func.is_operator {
            let Some(operator) = self.operator_factory.get_op(borrowed_func.info.name()) else {
                let operator_name = borrowed_func.info.name().clone();
                self.debugger.throw_diagnostic(Error::OperatorNotFound(operator_name, borrowed_func.span));
                return;
            };

            let nop = if operator.fix() == OperatorFix::Infix {
                2
            } else {
                1
            };

            if borrowed_func.info.params().len() != nop {
                let operator_name = borrowed_func.info.name().clone();
                self.debugger.throw_diagnostic(Error::OperatorExpectedParams(operator_name, nop - 1, borrowed_func.span.clone()));
            }
        }
    }

    fn resolve_function_code(&mut self, func: &FunctionRef) {
        let mut borrowed_func = func.borrow_mut();
        let func_scope = borrowed_func.scope().clone();

        self.resolve_code_block(&mut borrowed_func.code, &func_scope)
    }

    fn resolve_method_code(&mut self, method: &MethodRef) {
        let mut borrowed_method = method.borrow_mut();
        let method_scope = borrowed_method.scope().clone();

        self.resolve_code_block(&mut borrowed_method.code, &method_scope)
    }

    fn resolve_type(&mut self, typ: &mut Type, scope: &ScopeRef) {
        match typ.kind_mut() {
            TypeKind::Named(symbol_name) => {
                let Some(resolved_symbol) = scope.lookup_symbol(symbol_name) else {
                    // Throw an error and return
                    self.debugger.throw_diagnostic(Error::SymbolNotFound(symbol_name.clone(), typ.span().unwrap_or_else(Span::empty)));
                    return;
                };

                match resolved_symbol.resolve() {
                    Symbol::Type(resolved_type) => {
                        typ.set_kind(resolved_type);
                    }

                    _ => {
                        // Do something with `other_symbol`
                        self.debugger.throw_diagnostic(Error::SymbolNotAType(symbol_name.clone(), typ.span().unwrap_or_else(Span::empty)));
                    }
                };
            }

            TypeKind::Member { parent, member } => {
                self.resolve_type(parent, scope);

                // TODO: Add more detail to the error messages
                match parent.lookup_static_item(member) {
                    Some(Symbol::Type(resolved_type)) => {
                        typ.set_kind(resolved_type);
                    }

                    Some(_) => {
                        // Do something with `other_symbol`
                        self.debugger.throw_diagnostic(Error::MemberNotATy { parent_ty: parent.as_ref().clone(),
                                                                             member: member.clone(),
                                                                             span: typ.span().unwrap_or_else(Span::empty) });
                    }

                    None => {
                        self.debugger.throw_diagnostic(Error::MemberNotFound { parent_ty: parent.as_ref().clone(),
                                                                               member: member.clone(),
                                                                               span: typ.span().unwrap_or_else(Span::empty) });
                    }
                };
            }

            TypeKind::Function { return_type, params, .. } => {
                self.resolve_type(return_type, scope);

                for param in params {
                    self.resolve_type(param, scope);
                }
            }

            TypeKind::Tuple(tuple_items, _) => {
                for item in tuple_items {
                    self.resolve_type(item, scope);
                }
            }

            TypeKind::GenericOf { higher_kind, params } => {
                self.resolve_type(higher_kind, scope);

                for param in params.iter_mut() {
                    self.resolve_type(param, scope);
                }

                if let TypeKind::HKRawPointer = higher_kind.kind() {
                    if params.len() != 1 {
                        self.debugger.throw_diagnostic(Error::RPExpectsGenericParam {
                            count: params.len(),
                            span: higher_kind.span().unwrap_or_else(Span::empty)
                        });
                    }

                    let ptr = Box::new(params.remove(0));

                    typ.set_kind(TypeKind::RawPointer { pointer_type: ptr })
                }
            }

            TypeKind::RawPointer { pointer_type } => {
                self.resolve_type(pointer_type.as_mut(), scope);
            }

            TypeKind::GenericParam(param_name) => {
                let ty = scope.scope_type(&format!("generic[{}]", param_name)).unwrap();

                typ.set_kind(ty.kind);
            }

            TypeKind::Array { item, .. } => {
                self.resolve_type(item, scope);
            }

            _ => {
                // Do nothing
            }
        }
    }

    fn resolve_code_block(&mut self, code: &mut CodeBlock, scope: &ScopeRef) {
        for smt in code.statements_mut() {
            self.resolve_statement(smt, scope);
        }
    }

    fn resolve_statement(&mut self, smt: &mut Statement, scope: &ScopeRef) {
        match &mut smt.kind {
            StatementKind::Bind { pattern, typ, value } => {
                self.resolve_type(typ, scope);

                if let Some(value) = value {
                    self.resolve_value(value, scope)
                }

                self.define_pattern_in_scope(pattern, &scope);
            }

            StatementKind::Eval { value, .. } => {
                self.resolve_value(value, scope);
            }

            StatementKind::Return { value } => {
                if let Some(value) = value {
                    self.resolve_value(value, scope);
                }
            },
            
            StatementKind::Guard { condition, otherwise } => {
                self.resolve_value(condition.as_mut(), scope);

                self.resolve_code_block(otherwise, scope);
            }

            StatementKind::GuardLet { pattern, value, otherwise } => {
                self.resolve_value(value, scope);
                self.resolve_code_block(otherwise, scope);
                self.define_pattern_in_scope(pattern, scope);
            }

            StatementKind::Break(Some(value), _) => {
                self.resolve_value(value, scope)
            }
            StatementKind::Break(None, _) |
            StatementKind::Continue(_) => {}
            StatementKind::Panic => {}
        }
    }

    fn resolve_value(&mut self, value: &mut Value, scope: &ScopeRef) {
        match &mut value.kind {
            ValueKind::Named(name) => {
                let Some(sym) = scope.lookup_symbol(name).map(|sym| sym.resolve()) else {
                    self.debugger.throw_diagnostic(Error::SymbolNotFound(name.clone(), value.span.clone().unwrap_or_else(Span::empty)));
                    return
                };

                match sym {
                    Symbol::Type(ty) => {
                        value.set_kind(ValueKind::Metatype(ty.clone()));
                        if let TypeKind::Infer { .. } = value.typ.kind() {
                            value.typ.set_kind(TypeKind::Metatype(Box::new(ty.anon())));
                        }
                    }

                    Symbol::Value(res_val) => {
                        value.set_kind(res_val.kind);
                        value.set_type(res_val.typ);
                    }

                    Symbol::Function(function) => {
                        value.set_kind(ValueKind::Polymorphic(function));
                    }

                    Symbol::InstanceVariable(instance) => {
                        value.set_type(instance.borrow().typ.clone());
                        let Symbol::Value(myself) =
                            scope.lookup_symbol("self")
                                 .expect("Compiler Error: Expected self type when looking up instance variable")
                                 .resolve()
                        else { panic!("internal compiler error") };

                        value.set_kind(ValueKind::InstanceVariable { reciever: Box::new(myself),
                                                                     var:      instance, })
                    }

                    Symbol::Constant(constant) => {
                        let constant_value = constant.borrow().value.clone();

                        value.set_kind(constant_value.kind); 
                        value.set_type(constant_value.typ);
                    }

                    Symbol::Global(global) => {
                        value.set_type(global.ty().clone());
                        value.set_kind(ValueKind::GlobalVariable(global))
                    }
                            
                    Symbol::TupleField(..) => unreachable!(),
                    Symbol::EnumCase(..) => unreachable!(),
                }

                self.resolve_value(value, scope);
            }

            ValueKind::FuncCall { function, args } => {
                self.resolve_value(function, scope);

                if let ValueKind::Metatype(t) = &mut function.kind {
                    let init_type = std::mem::replace(t, TypeKind::Void).anon();

                    let initializer = init_type.lookup_instance_item("init", scope);

                    match initializer {
                        Some(Symbol::Function(monomorphizer)) => {
                            function.set_kind(ValueKind::Polymorphic(monomorphizer));
                        }

                        _ => {
                            // error: no initializer
                        }
                    }
                }

                for arg in &mut args.args {
                    self.resolve_value(arg, scope);
                }

                if let ValueKind::Polymorphic(polymorphics) = &mut function.kind {
                    let other = args.args
                        .iter()
                        .map(|arg| arg.name().cloned())
                        .collect();
                    polymorphics.filter_labels(&args.labels, &other);
                }

                if let TypeKind::Function { return_type, .. } | TypeKind::Method { return_type, .. } = function.typ.kind() {
                    let return_type = return_type.as_ref().clone();

                    value.set_type(return_type);
                }
            }

            ValueKind::Member { parent, member: _ } => self.resolve_value(parent, scope),

            ValueKind::If(if_value) => self.resolve_if_value(if_value, scope),

            ValueKind::Closure(closure) => {
                if closure.params.is_empty() {
                    return;
                }

                for param in &mut closure.params {
                    self.resolve_type(&mut param.typ, scope);
                }

                self.resolve_type(&mut value.typ, scope);
            }

            ValueKind::Tuple(fields) => {
                for field in fields {
                    self.resolve_value(field, scope)
                }
            }

            ValueKind::Match(match_value) => {
                self.resolve_value(&mut match_value.discriminant, scope);

                for branch in &mut match_value.branches {
                    // Create a new scope
                    let branch_scope = &Self::new_scope(scope);
                    self.define_pattern_in_scope(&mut branch.pattern, &branch_scope);
                    self.resolve_code_block(&mut branch.code, &branch_scope);
                }
            }

            ValueKind::Loop{ code: code_block, .. } => {
                self.resolve_code_block(code_block, scope)
            }

            ValueKind::Assign(left, right) => {
                self.resolve_value(left, scope);
                self.resolve_value(right, scope);
            }

            ValueKind::MonomorphizeFn { function, generics } => {
                let generic_scope = Self::new_scope(scope);

                for generic in generics.iter() {
                    scope.define_scope_type(&format!("generic[{}]", generic.0), generic.1.clone())
                }

                // Replace the type of this function using the generics
                self.resolve_value(function, &generic_scope);

                // Replace the types in the function using generics
                self.resolve_type(&mut value.typ, &generic_scope);

                let function_kind = std::mem::replace(&mut function.kind, ValueKind::Unit);
                value.set_kind(function_kind);
            }

            ValueKind::SequenceLiteral(seq) => {
                for item in seq {
                    self.resolve_value(item, scope);
                }
            }

            ValueKind::RepeatingLiteral { repeating, .. } => {
                self.resolve_value(repeating, scope);
            }

            _ => {}
        }
    }

    fn define_pattern_in_scope(&mut self, pattern: &mut Pattern, scope: &ScopeRef) {
        match &mut pattern.kind {
            PatternKind::Bind(name, varying) => {
                if !*varying {
                    if let Some(sym) = scope.lookup_symbol(name) {
                        let sym = sym.resolve();
                        if let Symbol::Constant(constant_ref) = sym {
                            let span = pattern.span.clone();
                            *pattern = PatternKind::Literal { value: constant_ref.borrow().value.clone() }.with_span(span);
                            return;
                        }
                    }
                }
                let mangled_bind_name = scope.define_variable(name, pattern.match_type.clone(), *varying);
                *name = mangled_bind_name;
            }
            PatternKind::Tuple { items, .. } |
            PatternKind::Variant { items, .. } => {
                items
                    .iter_mut()
                    .for_each(|pattern| self.define_pattern_in_scope(pattern, scope))
            }
            _ => {}
        }
    }

    pub fn resolve_closure(&mut self, closure: &mut Closure, closure_type: &mut Type, scope: &ScopeRef) {
        let closure_scope = ScopeRef::new(Some(scope),
                                          ScopeRelation::Scope,
                                          ScopeType::Code,
                                          false,
                                          true);

        self.add_closure_params(closure, closure_type);

        for param in &mut closure.params {
            let param_value = ValueKind::FunctionParam(param.name.clone(), false).anon(param.typ.clone());

            closure_scope.add_symbol(param.name.clone(),
                                     Visibility::Local,
                                     Symbol::Value(param_value));
        }

        self.resolve_code_block(&mut closure.code, &closure_scope);
    }

    fn add_closure_params(&mut self, closure: &mut Closure, closure_type: &mut Type) {
        if !closure.params.is_empty() {
            return;
        }

        let TypeKind::Function { params, .. } = closure_type.kind() else {
            // Error
            return;
        };

        if params.len() == 1 {
            let it_type = params.first().unwrap().clone();

            closure.params.push(ClosureParam { name: "it".to_string(),
                                               typ:  it_type, });

            return;
        }

        for (i, param) in params.iter().enumerate() {
            closure.params.push(ClosureParam { name: format!("$par{index}", index = i + 1),
                                               typ:  param.clone(), });
        }
    }

    fn resolve_if_value(&mut self, if_value: &mut IfValue, scope: &ScopeRef) {
        self.resolve_value(&mut if_value.condition, scope);

        self.resolve_code_block(&mut if_value.positive, &Self::new_scope(scope));

        if let Some(negative_block) = &mut if_value.negative {
            match negative_block {
                IfBranch::CodeBlock(codeblock) => self.resolve_code_block(codeblock, &Self::new_scope(scope)),
                IfBranch::Else(else_if_value) => self.resolve_if_value(else_if_value, scope),
                IfBranch::ElseLet(match_value) => {
                    self.resolve_value(&mut match_value.discriminant, scope);

                    for branch in &mut match_value.branches {
                        // Create a new scope
                        let branch_scope = &Self::new_scope(scope);
                        self.define_pattern_in_scope(&mut branch.pattern, &branch_scope);
                        self.resolve_code_block(&mut branch.code, &branch_scope);
                    }
                }
            }
        }
    }

    pub (crate) fn new_scope(outer: &ScopeRef) -> ScopeRef {
        ScopeRef::new(Some(outer), ScopeRelation::Code, ScopeType::Code, false, false)
    }
}


#[derive(Debug)]
enum Error {
    ClosureIsNotAFunc,
    RPExpectsGenericParam { count: usize, span: Span },
    SymbolNotFound(String, Span),
    SymbolNotAType(String, Span),
    ExpectedTypeFound(String),

    MemberNotFound { parent_ty: Type, member: String, span: Span },

    MemberNotATy { parent_ty: Type, member: String, span: Span },

    OperatorExpectedParams(String, usize, Span),

    OperatorNotFound(String, Span),

    WrongEnumType(Type, Span),
    TagAlreadyUsed(usize, Vec<Span>),

    PrivateSymbol {
        name: String,
        span: Span,
        suggestion: Option<Symbol>
    }
}

impl IntoDiagnostic for Error {
    fn into_diagnostic(self) -> errors::Diagnostic {
        match self {
            Self::RPExpectsGenericParam { count, span } => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "expects_generic_param",
                                format!("expected 1 generic parameter, found {count}"),
                                vec![CodeLocation::new(span, None)])
            }
            Error::ClosureIsNotAFunc => todo!(),
            Error::SymbolNotFound(name, span) => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "sym_not_found",
                                format!("symbol `{name}` not found"),
                                vec![ CodeLocation::new(span, None) ])
            }
            Error::SymbolNotAType(_, _) => todo!(),
            Error::ExpectedTypeFound(_) => todo!(),
            Error::MemberNotFound { parent_ty, member, span } => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "mem_not_found",
                                format!("{parent_ty} doesn't have member `{member}`"),
                                vec![ CodeLocation::new(span, None) ])
            }
            Error::MemberNotATy { parent_ty, member, span } => todo!(),
            Error::OperatorExpectedParams(_, _, _) => todo!(),
            Error::OperatorNotFound(_, _) => todo!(),
            Error::WrongEnumType(_, _) => todo!(),
            Error::TagAlreadyUsed(_, _) => todo!(),
            Error::PrivateSymbol { name, span, suggestion } => todo!(),
        }
    }
}