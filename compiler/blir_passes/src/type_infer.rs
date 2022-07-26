use blir::{code::{FunctionRef, MethodRef},
           scope::ScopeRef,
           typ::{StructRef, Type, TypeKind, EnumRef},
           value::{Closure, Value},
           BlirContext, Library};
use errors::DiagnosticReporter;
use tyinfer::context::TypeInferContext;

pub struct TypeInferPass<'a, 'l> {
    context:  &'a mut BlirContext,
    debugger: &'a mut DiagnosticReporter<'l>,
}

impl<'a, 'l> TypeInferPass<'a, 'l> {
    pub fn new(context: &'a mut BlirContext, debugger: &'a mut DiagnosticReporter<'l>) -> Self { Self { context, debugger } }

    pub fn run_pass(&mut self, library: &mut Library) {
        let scope = library.scope();

        for global in &library.globals {
            let (mut val, mut ty) = global.value_and_ty_mut();
            self.infer_variable(&mut ty, &mut val, scope);
        }

        for constant in &library.constants {
            let mut borrow_ref = constant.borrow_mut();
            let borrow = &mut *borrow_ref;
            self.infer_variable(&mut borrow.typ, &mut borrow.value, scope);
        }

        for r#struct in library.structs.iter() {
            self.infer_struct(r#struct);
        }

        for r#enum in library.enums.iter() {
            self.infer_enum(r#enum);
        }

        for func in library.functions.iter() {
            self.infer_func(func);
        }
    }

    fn infer_struct(&mut self, r#struct: &StructRef) {
        let scope = r#struct.borrow().scope().clone();

        for constant in &r#struct.borrow().constants {
            let mut borrow_ref = constant.borrow_mut();
            let borrow = &mut *borrow_ref;
            self.infer_variable(&mut borrow.typ, &mut borrow.value, &scope);
        }

        for variable in &r#struct.borrow().instance_vars {
            let mut borrow_ref = variable.borrow_mut();
            let borrow = &mut *borrow_ref;
            if let Some(value) = &mut borrow.default_value {
                self.infer_variable(&mut borrow.typ, value, &scope);
            }
        }
        
        for global in &r#struct.borrow().globals {
            self.infer_variable(&mut global.ty_mut(), &mut global.default_value_mut(), &scope);
        }

        for r#struct in &r#struct.borrow().substructs {
            self.infer_struct(r#struct);
        }

        for r#enum in &r#struct.borrow().subenums {
            self.infer_enum(r#enum);
        }

        for method in &r#struct.borrow().methods {
            self.infer_method(method);
        }
    }

    fn infer_enum(&mut self, r#enum: &EnumRef) {
        for r#struct in r#enum.substructs().iter() {
            self.infer_struct(r#struct);
        }

        for r#enum in r#enum.subenums().iter() {
            self.infer_enum(r#enum);
        }

        for method in r#enum.methods().iter() {
            self.infer_method(method);
        }
    }

    fn infer_variable(&mut self, typ: &mut Type, value: &mut Value, scope: &ScopeRef) {
        let mut infer_context = TypeInferContext::new(self.debugger, self.context);

        infer_context.infer_variable(typ, value, scope);

        infer_context.replace().replace_variable(typ, value, scope);

        infer_context.infer_variable(typ, value, scope);

        infer_context.finish().replace_variable(typ, value, scope);
    }

    fn infer_func(&mut self, func: &FunctionRef) {
        let mut infer_context = TypeInferContext::new(self.debugger, self.context);

        let mut borrowed_function = func.borrow_mut();

        let function_scope = borrowed_function.scope().clone();
        let function_type = borrowed_function.info.return_type().clone();
        let function_block = &mut borrowed_function.code;

        // This is going to need to change
        // We do only one step from 0.6 on

        for _ in 0..3 {
            infer_context.replace()
                         .replace_codeblock(function_block, &function_scope);

            infer_context.infer_codeblock(function_block, &function_type, &function_scope);
        }

        infer_context.finish()
                     .replace_codeblock(function_block, &function_scope);

        infer_context.infer_codeblock(function_block, &function_type, &function_scope);

        infer_context.finish()
                     .replace_codeblock(function_block, &function_scope);

        infer_context.infer_codeblock(function_block, &function_type, &function_scope);

        infer_context.finish()
                     .replace_codeblock(function_block, &function_scope);

        /*infer_context.infer_codeblock(function_block, &function_type, &function_scope);

        infer_context.finish()
                    .replace_codeblock(function_block, &function_scope);

        infer_context.infer_codeblock(function_block, &function_type, &function_scope);

        infer_context.finish()
                    .replace_codeblock(function_block, &function_scope);*/
    }

    fn infer_method(&mut self, method: &MethodRef) {
        let mut infer_context = TypeInferContext::new(self.debugger, self.context);

        let mut borrowed_function = method.borrow_mut();

        let function_scope = borrowed_function.scope().clone();
        let function_type = borrowed_function.info.return_type().clone();
        let function_block = &mut borrowed_function.code;

        for _ in 0..2 {
            infer_context.replace()
                         .replace_codeblock(function_block, &function_scope);

            infer_context.infer_codeblock(function_block, &function_type, &function_scope);
        }

        infer_context.finish()
                     .replace_codeblock(function_block, &function_scope);

        infer_context.infer_codeblock(function_block, &function_type, &function_scope);

        infer_context.finish()
                     .replace_codeblock(function_block, &function_scope);

        infer_context.infer_codeblock(function_block, &function_type, &function_scope);

        infer_context.finish()
                     .replace_codeblock(function_block, &function_scope);
    }

    pub fn infer_closure(&mut self, func: &mut Closure, closure_type: &mut Type, scope: &ScopeRef) {
        let mut infer_context = TypeInferContext::new(self.debugger, self.context);

        let function_type = match closure_type.kind() {
            TypeKind::Function { return_type, .. } => return_type,
            _ => return,
        };

        let function_block = &mut func.code;

        for _ in 0..2 {
            infer_context.replace()
                         .replace_codeblock(function_block, scope);

            infer_context.infer_codeblock(function_block, function_type, scope);
        }

        infer_context.finish()
                     .replace_codeblock(function_block, scope);

        infer_context.infer_codeblock(function_block, function_type, scope);

        infer_context.finish()
                     .replace_codeblock(function_block, scope);

        infer_context.infer_codeblock(function_block, &function_type, scope);

        infer_context.finish()
                     .replace_codeblock(function_block, scope);
    }
}
