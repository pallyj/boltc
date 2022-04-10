use blir::{attributes::AttributeFactory,
           code::{CodeBlock, ExternFunctionRef, FunctionRef, MethodRef, Statement, StatementKind},
           scope::ScopeRef,
           typ::{StructRef, Type, TypeKind},
           value::{ConstantRef, IfBranch, IfValue, Value, ValueKind, VarRef},
           BlirContext, Library, Symbol};
use errors::{debugger::Debugger, error::ErrorCode};

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
    debugger: &'a mut Debugger<'l>
}

impl<'a, 'l> TypeResolvePass<'a, 'l> {
    pub fn new(factory: &'a AttributeFactory, context: &'a mut BlirContext, debugger: &'a mut Debugger<'l>) -> Self {
        Self {
            factory,
            context,
            debugger
        }
    }

    pub fn run_pass(mut self, library: &mut Library) {
        // Run the pass on each extern function
        for extern_func in &library.extern_functions {
            // Resolve types in the extern function
            self.resolve_extern_func_types(extern_func);

            // Apply function attributes

        }

        // Resolve types in each struct
        for r#struct in &library.structs {
            self.resolve_struct_types(r#struct);
        }

        // Resolve code in each struct
        for r#struct in &library.structs {

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

            // Go through the functions code?
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
    
        /*for constant in &r#struct.constants {
            walk_constant(constant, scope, debugger);
        }
    
        for variable in &r#struct.instance_vars {
            walk_variable(variable, scope, debugger);
        }
    
        for method in &r#struct.methods {
            walk_method(method, debugger);
    
            let mut borrow = method.borrow_mut();
            let link_name = borrow.mangle();
    
            borrow.info.set_link_name(link_name.clone());
            let attributes = borrow.attributes.clone();
            factory.apply_func_attributes(&attributes, &mut borrow.info, context, debugger)
        }*/
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
}



pub fn run_pass(library: &mut Library, factory: &AttributeFactory, context: &mut BlirContext, debugger: &mut Debugger) {
    let scope = library.scope();

    for func in &library.extern_functions {
        walk_extern_function(func, debugger);
        let mut borrow = func.borrow_mut();
        let attributes = borrow.attributes.clone();
        factory.apply_func_attributes(&attributes, &mut borrow.info, context, debugger)
    }

    for r#struct in &library.structs {
        walk_struct(r#struct, scope, factory, context, debugger);
    }

    for func in &library.functions {
        walk_function(func, debugger);
        let mut borrow = func.borrow_mut();
        let link_name = borrow.mangle();

        borrow.info.set_link_name(link_name.clone());
        let attributes = borrow.attributes.clone();
        factory.apply_func_attributes(&attributes, &mut borrow.info, context, debugger)
    }

    for func in &library.functions {
        walk_function_code(func, debugger);
    }
}

fn walk_struct(r#struct: &StructRef, _scope: &ScopeRef, factory: &AttributeFactory, context: &mut BlirContext, debugger: &mut Debugger) {
    factory.apply_struct_attributes(r#struct, context, debugger);

    let r#struct = r#struct.borrow();
    let scope = r#struct.scope();

    for substruct in &r#struct.substructs {
        walk_struct(substruct, scope, factory, context, debugger);
    }

    for constant in &r#struct.constants {
        walk_constant(constant, scope, debugger);
    }

    for variable in &r#struct.instance_vars {
        walk_variable(variable, scope, debugger);
    }

    for method in &r#struct.methods {
        walk_method(method, debugger);

        let mut borrow = method.borrow_mut();
        let link_name = borrow.mangle();

        borrow.info.set_link_name(link_name.clone());
        let attributes = borrow.attributes.clone();
        factory.apply_func_attributes(&attributes, &mut borrow.info, context, debugger)
    }

    for method in &r#struct.methods {
        walk_method_code(method, debugger);
    }
}

fn walk_variable(var: &VarRef, scope: &ScopeRef, debugger: &mut Debugger) {
    walk_type(&mut (var.borrow_mut().typ), scope, debugger);
    if let Some(value) = &mut var.borrow_mut().default_value {
        walk_value(value, scope, debugger)
    }
}

fn walk_constant(var: &ConstantRef, scope: &ScopeRef, debugger: &mut Debugger) {
    walk_type(&mut (var.borrow_mut().typ), scope, debugger);
    walk_value(&mut var.borrow_mut().value, scope, debugger);
}

fn walk_method(method: &MethodRef, debugger: &mut Debugger) {
    let mut method = method.borrow_mut();

    let scope = method.scope().clone();

    walk_type(method.info.return_type_mut(), &scope, debugger);

    method.info
          .params_mut()
          .iter_mut()
          .for_each(|param| walk_type(&mut param.typ, &scope, debugger));

    method.add_params();
}

fn walk_method_code(method: &MethodRef, debugger: &mut Debugger) {
    let mut method = method.borrow_mut();

    let scope = method.scope().clone();

    walk_code_block(&mut method.code, &scope, debugger);
}

fn walk_function(function: &FunctionRef, debugger: &mut Debugger) {
    let mut function = function.borrow_mut();

    let scope = function.scope().clone();

    walk_type(function.info.return_type_mut(), &scope, debugger);

    function.info
            .params_mut()
            .iter_mut()
            .for_each(|param| walk_type(&mut param.typ, &scope, debugger));

    function.add_params();
}

fn walk_extern_function(function: &ExternFunctionRef, debugger: &mut Debugger) {
    let mut function = function.borrow_mut();

    let scope = function.parent.clone();

    walk_type(function.info.return_type_mut(), &scope, debugger);

    function.info
            .params_mut()
            .iter_mut()
            .for_each(|param| walk_type(&mut param.typ, &scope, debugger));
}

fn walk_function_code(function: &FunctionRef, debugger: &mut Debugger) {
    let mut function = function.borrow_mut();

    let scope = function.scope().clone();

    walk_code_block(&mut function.code, &scope, debugger);
}

fn walk_code_block(code: &mut CodeBlock, scope: &ScopeRef, debugger: &mut Debugger) {
    for smt in code.statements_mut() {
        walk_statement(smt, scope, debugger);
    }
}

fn walk_statement(smt: &mut Statement, scope: &ScopeRef, debugger: &mut Debugger) {
    match &mut smt.kind {
        StatementKind::Bind { name, typ, value } => {
            walk_type(typ, scope, debugger);

            *name = scope.define_variable(name, typ.clone());

            if let Some(value) = value {
                walk_value(value, scope, debugger)
            }
        }

        StatementKind::Eval { value, escaped: _ } => {
            walk_value(value, scope, debugger);
        }

        StatementKind::Return { value } => {
            if let Some(value) = value {
                walk_value(value, scope, debugger);
            }
        }
    }
}

fn walk_value(value: &mut Value, scope: &ScopeRef, debugger: &mut Debugger) {
    match &mut value.kind {
        ValueKind::Named(name) => {
            let Some(sym) = scope.lookup_symbol(name).map(|sym| sym.resolve()) else {
				debugger.throw_single(ErrorCode::SymbolNotFound { name: name.clone() }, &value.span );
				return
			};

            match sym {
                Symbol::Type(ty) => {
                    value.set_kind(ValueKind::Metatype(ty.clone()));
                    value.typ.set_kind(TypeKind::Metatype(Box::new(ty)));
                }

                Symbol::Value(res_val) => {
                    value.set_kind(res_val.kind);
                    value.typ = res_val.typ;
                }

                Symbol::Function(function) => {
                    value.set_type(function.take_typ());
                    value.set_kind(ValueKind::StaticFunc(function));
                }

                Symbol::ExternFunction(function) => {
                    value.set_type(function.take_typ());
                    value.set_kind(ValueKind::ExternFunc(function));
                }

                Symbol::StaticMethod(function) => {
                    value.set_type(function.take_typ());
                    value.set_kind(ValueKind::StaticMethod(function));
                }

                Symbol::InstanceVariable(instance) => {
                    value.set_type(instance.borrow().typ.clone());
                    let self_type = scope.scope_type("self")
                                         .expect("Compiler Error: Expected self type when looking up instance variable");
                    let myself = ValueKind::SelfVal.anon(self_type);
                    value.set_kind(ValueKind::InstanceVariable { reciever: Box::new(myself),
                                                                 var:      instance, })
                }

                Symbol::InstanceMethod(method) => {
                    value.set_type(method.take_typ());
                    let self_type = scope.scope_type("self")
                                         .expect("Compiler Error: Expected self type when looking up instance variable");
                    let myself = ValueKind::SelfVal.anon(self_type);
                    value.set_kind(ValueKind::InstanceMethod { reciever: Box::new(myself),
                                                               method })
                }

                Symbol::Constant(constant) => {
                    let constant_value = constant.borrow().value.clone();

                    value.set_kind(constant_value.kind);
                    value.typ = constant_value.typ;
                }
            }
        }

        ValueKind::FuncCall { function, args } => {
            walk_value(function.as_mut(), scope, debugger);

            if let ValueKind::Metatype(t) = &mut function.kind {
                let t = std::mem::replace(t, TypeKind::Void);

                function.set_kind(ValueKind::Init(t.anon()));
            }

            args.args
                .iter_mut()
                .for_each(|arg| walk_value(arg, scope, debugger));

            if let TypeKind::Function { return_type,
                                        params: _,
                                        labels: _, } = function.typ.kind()
            {
                let return_type = return_type.as_ref().clone();

                value.set_type(return_type);
            } else if let TypeKind::Method { return_type, .. } = function.typ.kind() {
                let return_type = return_type.as_ref().clone();

                value.set_type(return_type);
            }
        }

        ValueKind::Member { parent, member: _ } => walk_value(parent.as_mut(), scope, debugger),

        ValueKind::If(if_value) => walk_if_value(if_value, scope, debugger),

        _ => {}
    }
}

fn walk_if_value(if_value: &mut IfValue, scope: &ScopeRef, debugger: &mut Debugger) {
    walk_value(&mut if_value.condition, scope, debugger);

    walk_code_block(&mut if_value.positive, scope, debugger);

    if let Some(negative_block) = &mut if_value.negative {
        match negative_block {
            IfBranch::CodeBlock(codeblock) => walk_code_block(codeblock, scope, debugger),
            IfBranch::Else(else_if_value) => walk_if_value(else_if_value, scope, debugger),
        }
    }
}

fn walk_type(typ: &mut Type, scope: &ScopeRef, debugger: &mut Debugger) {
    match &mut typ.kind {
        TypeKind::Named(symbol_name) => {
            let Some(resolved_sym) = scope.lookup_symbol(symbol_name) else {
				debugger.throw_single(ErrorCode::TypeNotFound { name: symbol_name.clone() }, &typ.span);
				return
			};

            let Symbol::Type(resolved_typ) = resolved_sym.resolve() else {
				debugger.throw_single(ErrorCode::SymNotAType { name: symbol_name.clone() }, &typ.span);
				return
			};

            typ.set_kind(resolved_typ);
        }

        TypeKind::Member { parent, member } => {
            walk_type(parent.as_mut(), scope, debugger);

            let Some(sym) = parent.lookup_static_item(member.as_str()) else {
				debugger.throw_single(ErrorCode::MemberNotFound { name: member.clone() }, &typ.span);
				return;
			};

            let Symbol::Type(tk) = sym else {
				debugger.throw_single(ErrorCode::MemberNotATy { name: member.clone() }, &typ.span);
				return;
			};

            typ.set_kind(tk);
        }

        TypeKind::Function { return_type,
                             params,
                             labels: _, } => {
            walk_type(return_type.as_mut(), scope, debugger);

            params.iter_mut()
                  .for_each(|param| walk_type(param, scope, debugger));
        }

        _ => {}
    }
}
