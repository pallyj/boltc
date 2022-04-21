use blir::{attributes::AttributeFactory,
           code::{CodeBlock, ExternFunctionRef, FunctionRef, MethodRef, Statement, StatementKind},
           scope::{ScopeRef, ScopeRelation, ScopeType},
           typ::{StructRef, Type, TypeKind},
           value::{ConstantRef, IfBranch, IfValue, Value, ValueKind, VarRef, Closure, ClosureParam},
           BlirContext, Library, Symbol, Visibility};
use errors::{debugger::Debugger, error::ErrorCode};
use parser::operators::{OperatorFactory, OperatorFix};

use crate::init_default::add_default_initializer;

/// 
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
    factory: &'a AttributeFactory,
    context: &'a mut BlirContext,
    debugger: &'a mut Debugger<'l>,
    operator_factory: &'a OperatorFactory,
}

impl<'a, 'l> TypeResolvePass<'a, 'l> {
    pub fn new(
        factory: &'a AttributeFactory,
        operator_factory: &'a OperatorFactory,
        context: &'a mut BlirContext,
        debugger: &'a mut Debugger<'l>) -> Self
    {
        Self {
            factory,
            context,
            debugger,
            operator_factory
        }
    }

    pub fn run_pass(mut self, library: &mut Library) {
        // Run the pass on each extern function
        for extern_func in &library.extern_functions {
            // Resolve types in the extern function
            self.resolve_extern_func_types(extern_func);

            // Apply function attributes
        }

        for constant in &library.constants {
            self.resolve_constant(constant, library.scope());
        }

        // Resolve types in each struct
        for r#struct in &library.structs {
            self.resolve_struct_types(r#struct);
        }

        for r#struct in &library.structs {
            // Create a default initializer
            add_default_initializer(&r#struct);
        }

        // Resolve code in each struct
        for r#struct in &library.structs {
            self.resolve_struct_code(r#struct);
        }

        // Run the pass on each function
        for func in &library.functions {
            // Resolve types in the function
            self.resolve_func_types(func);

            // Set link name
            let mangled_name = func.borrow().mangle();
            func.borrow_mut().info
                .set_link_name(mangled_name);

            // Apply function attributes
            let mut func = func.borrow_mut();
            let attributes = func.attributes.clone();
            self.factory.apply_func_attributes(&attributes, &mut func.info, self.context, self.debugger)
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
        self.resolve_type(&mut borrowed_func.info.return_type_mut(), &parent_scope);

        // Resolve each parameter's type
        for param in borrowed_func.info.params_mut() {
            self.resolve_type(&mut param.typ, &parent_scope);
        }
    }

    fn resolve_struct_types(&mut self, r#struct: &StructRef) {
        self.factory.apply_struct_attributes(r#struct, self.context, self.debugger);
    
        let r#struct = r#struct.borrow();
        let scope = r#struct.scope();
    
        for substruct in &r#struct.substructs {
            self.resolve_struct_types(substruct);
        }
    
        for constant in &r#struct.constants {
            self.resolve_constant(constant, scope);
        }
    
        for variable in &r#struct.instance_vars {
            self.resolve_variable(variable, scope);
        }

        for method in &r#struct.methods {
            // Resolve method types
            self.resolve_method_types(method);

            // Set link name
            let mangled_name = method.borrow().mangle();
            method.borrow_mut().info
                  .set_link_name(mangled_name);

            // Apply function attributes
            let mut method = method.borrow_mut();
            let attributes = method.attributes.clone();
            self.factory.apply_func_attributes(&attributes, &mut method.info, self.context, self.debugger)
        }
    }

    fn resolve_struct_code(&mut self, r#struct: &StructRef) {
        let r#struct = r#struct.borrow();
    
        for substruct in &r#struct.substructs {
            self.resolve_struct_code(substruct);
        }

        for method in &r#struct.methods {
            self.resolve_method_code(method);
        }
    }

    fn resolve_variable(&mut self, var: &VarRef, scope: &ScopeRef) {
        self.resolve_type(&mut var.borrow_mut().typ, scope);

        // Resolve value
    }
    
    fn resolve_constant(&mut self, var: &ConstantRef, scope: &ScopeRef) {
        self.resolve_type(&mut var.borrow_mut().typ, scope);
        // Resolve value
    }

    fn resolve_func_types(&mut self, function: &FunctionRef) {
        let mut borrowed_func = function.borrow_mut();
        let parent_scope = borrowed_func.scope().clone();

        // Resolve the return type
        self.resolve_type(&mut borrowed_func.info.return_type_mut(), &parent_scope);

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
        self.resolve_type(&mut borrowed_func.info.return_type_mut(), &parent_scope);

        // Resolve each parameter's type
        for param in borrowed_func.info.params_mut() {
            self.resolve_type(&mut param.typ, &parent_scope);
        }

        // Add the functions parameters to its scope
        borrowed_func.add_params();

        if borrowed_func.is_operator {
            let Some(operator) = self.operator_factory.get_op(borrowed_func.info.name()) else {
                self.debugger.throw(ErrorCode::OperatorDNE(borrowed_func.info.name().clone()), vec![borrowed_func.span]);
                return;
            };

            let nop = if operator.fix() == OperatorFix::Infix { 2 } else { 1 };

            if borrowed_func.info.params().len() != nop {
                self.debugger.throw(ErrorCode::OperatorExpectedParams(borrowed_func.info.name().clone(), nop - 1), vec![borrowed_func.span])
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
                    self.debugger.throw_single(ErrorCode::MemberNotFound { name: symbol_name.clone() }, &typ.span);
                    return;
                };


                match resolved_symbol.resolve() {
                    Symbol::Type(resolved_type) => {
                        typ.set_kind(resolved_type);
                    }

                    _ => {
                        // Do something with `other_symbol`
                        self.debugger.throw_single(ErrorCode::SymNotAType { name: symbol_name.clone() }, &typ.span);
                    }
                };
            }

            TypeKind::Member { parent, member } => {
                self.resolve_type(parent, scope);

                // TODO: Add more detail to the error messages
                match parent.lookup_static_item(&member) {
                    Some(Symbol::Type(resolved_type)) => {
                        typ.set_kind(resolved_type);
                    }

                    Some(_) => {
                        // Do something with `other_symbol`
                        self.debugger.throw_single(ErrorCode::MemberNotATy { name: member.clone() }, &typ.span);
                    }

                    None => {
                        self.debugger.throw_single(ErrorCode::MemberNotFound { name: member.clone() }, &typ.span);
                    }
                };
            }

            TypeKind::Function { return_type, params, .. } => {
                self.resolve_type(return_type, scope);

                for param in params {
                    self.resolve_type(param, scope);
                }
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
            StatementKind::Bind { name, typ, value } => {
                self.resolve_type(typ, scope);
    
                *name = scope.define_variable(name, typ.clone());
    
                if let Some(value) = value {
                    self.resolve_value(value, scope)
                }
            }
    
            StatementKind::Eval { value, .. } => {
                self.resolve_value(value, scope);
            }
    
            StatementKind::Return { value } => {
                if let Some(value) = value {
                    self.resolve_value(value, scope);
                }
            }
        }
    }
    
    fn resolve_value(&mut self, value: &mut Value, scope: &ScopeRef) {
        match &mut value.kind {
            ValueKind::Named(name) => {
                let Some(sym) = scope.lookup_symbol(name).map(|sym| sym.resolve()) else {
                    self.debugger.throw_single(ErrorCode::SymbolNotFound { name: name.clone() }, &value.span );
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
                        let self_type = scope.scope_type("self")
                                             .expect("Compiler Error: Expected self type when looking up instance variable");
                        let myself = ValueKind::SelfVal.anon(self_type);
                        value.set_kind(ValueKind::InstanceVariable { reciever: Box::new(myself),
                                                                     var:      instance, })
                    }
    
                    Symbol::Constant(constant) => {
                        let constant_value = constant.borrow().value.clone();
    
                        value.set_kind(constant_value.kind);
                        value.typ = constant_value.typ;
                    }

                }
            }
    
            ValueKind::FuncCall { function, args } => {
                self.resolve_value(function, scope);
    
                if let ValueKind::Metatype(t) = &mut function.kind {
                    let init_type = std::mem::replace(t, TypeKind::Void).anon();

                    let initializer = init_type.lookup_static_item("init");

                    match initializer {
                        Some(Symbol::Function(monomorphizer)) => {
                            function.set_kind(ValueKind::Polymorphic(monomorphizer));
                        }

                        _ => {
                            panic!("{init_type:?}");
                        }
                    }

                    //function.set_type(init_type.init_type().anon());
                    //function.set_kind(ValueKind::Init(init_type));
                }

                for arg in &mut args.args {
                    self.resolve_value(arg, scope);
                }

                if let ValueKind::Polymorphic(polymorphics) = &mut function.kind {
                    polymorphics.filter_labels(&args.labels);
                }
    
                if let TypeKind::Function { return_type, .. }
                     | TypeKind::Method { return_type, .. } = function.typ.kind() {
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
    
            _ => {}
        }
    }

    pub fn resolve_closure(
        &mut self,
        closure: &mut Closure,
        closure_type: &mut Type,
        scope: &ScopeRef)
    {
        let closure_scope = ScopeRef::new(
            Some(scope),
            ScopeRelation::Scope,
            ScopeType::Code,
            false,
            true
        );

        self.add_closure_params(closure, closure_type);

        for param in &mut closure.params {
            let param_value = ValueKind::FunctionParam(param.name.clone()).anon(param.typ.clone());

            closure_scope.add_symbol(param.name.clone(), Visibility::Local, Symbol::Value(param_value));
        }

        self.resolve_code_block(&mut closure.code, &closure_scope);
    }

    fn add_closure_params(
        &mut self,
        closure: &mut Closure,
        closure_type: &mut Type)
    {
        if !closure.params.is_empty() {
            return
        }

        let TypeKind::Function { params, .. } = closure_type.kind() else {
            // Error
            return;
        };

        if params.len() == 1 {
            let it_type = params.first().unwrap().clone();

            closure.params.push(ClosureParam {
                name: "it".to_string(),
                typ: it_type
            });

            return;
        }

        for (i, param) in params.iter().enumerate() {
            closure.params.push(ClosureParam {
                name: format!("${i}"),
                typ: param.clone()
            });
        }
    }
    
    fn resolve_if_value(&mut self, if_value: &mut IfValue, scope: &ScopeRef) {
        self.resolve_value(&mut if_value.condition, scope);
    
        self.resolve_code_block(&mut if_value.positive, scope);
    
        if let Some(negative_block) = &mut if_value.negative {
            match negative_block {
                IfBranch::CodeBlock(codeblock) => self.resolve_code_block(codeblock, scope),
                IfBranch::Else(else_if_value) => self.resolve_if_value(else_if_value, scope),
            }
        }
    }
}