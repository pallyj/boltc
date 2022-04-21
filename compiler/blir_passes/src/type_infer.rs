use blir::{code::{FunctionRef, MethodRef},
           scope::ScopeRef,
           typ::{StructRef, Type, TypeKind},
           value::{Closure, Value},
           BlirContext, Library};
use errors::debugger::Debugger;
use tyinfer::context::TypeInferContext;

pub struct TypeInferPass<'a, 'l> {
    context:  &'a mut BlirContext,
    debugger: &'a mut Debugger<'l>,
}

impl<'a, 'l> TypeInferPass<'a, 'l> {
    pub fn new(context: &'a mut BlirContext, debugger: &'a mut Debugger<'l>) -> Self { Self { context, debugger } }

    pub fn run_pass(&mut self, library: &mut Library) {
        for r#struct in library.structs.iter() {
            self.infer_struct(r#struct);
        }

        for func in library.functions.iter() {
            self.infer_func(func);
        }

        let scope = library.scope();

        for constant in &library.constants {
            let mut borrow_ref = constant.borrow_mut();
            let borrow = &mut *borrow_ref;
            self.infer_variable(&mut borrow.typ, &mut borrow.value, scope);
        }
    }

    fn infer_struct(&mut self, r#struct: &StructRef) {
        for r#struct in &r#struct.borrow().substructs {
            self.infer_struct(r#struct);
        }

        for method in &r#struct.borrow().methods {
            self.infer_method(method);
        }

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
    }
}
