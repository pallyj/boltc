use blir::{Library, BlirContext, code::{FunctionRef, MethodRef}, typ::StructRef};
use errors::debugger::Debugger;
use tyinfer::context::TypeInferContext;

pub struct TypeInferPass<'a, 'l> {
	context: &'a mut BlirContext,
    debugger: &'a mut Debugger<'l>
}

impl<'a, 'l> TypeInferPass<'a, 'l> {
	pub fn new(context: &'a mut BlirContext, debugger: &'a mut Debugger<'l>) -> Self {
        Self {
			context,
            debugger
        }
    }

	pub fn run_pass(&mut self, library: &mut Library) {
		for r#struct in library.structs.iter() {
			self.infer_struct(r#struct);
		}

		for func in library.functions.iter() {
			self.infer_func(func);
		}
	}

	fn infer_struct(&mut self, r#struct: &StructRef) {
		for method in &r#struct.borrow().methods {
			self.infer_method(method);
		}
	}

	fn infer_func(&mut self, func: &FunctionRef) {
		let mut infer_context = TypeInferContext::new(self.debugger, self.context);

		let mut borrowed_function = func.borrow_mut();

		let function_scope = borrowed_function.scope().clone();
		let function_type = borrowed_function.info.return_type().clone();
		let function_block = &mut borrowed_function.code;

		for _ in 0..2 {
			infer_context
				.replace()
				.replace_codeblock(function_block, &function_scope);
				
			infer_context
				.infer_codeblock(function_block, &function_type, &function_scope);
		}

		infer_context
			.finish()
			.replace_codeblock(function_block, &function_scope);

		infer_context
			.infer_codeblock(function_block, &function_type, &function_scope);

		infer_context
			.finish()
			.replace_codeblock(function_block, &function_scope);
	}

	fn infer_method(&mut self, method: &MethodRef) {
		let mut infer_context = TypeInferContext::new(self.debugger, self.context);

		let mut borrowed_function = method.borrow_mut();

		let function_scope = borrowed_function.scope().clone();
		let function_type = borrowed_function.info.return_type().clone();
		let function_block = &mut borrowed_function.code;

		for _ in 0..3 {
			infer_context
				.replace()
				.replace_codeblock(function_block, &function_scope);
				
			infer_context
				.infer_codeblock(function_block, &function_type, &function_scope);
		}

		infer_context
			.finish()
			.replace_codeblock(function_block, &function_scope);
	}
}