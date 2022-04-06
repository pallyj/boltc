use blir::{code::{FunctionRef, ExternFunctionRef}, typ::TypeKind};

use crate::BlirLowerer;

impl BlirLowerer {
	pub (super) fn lower_func_signature(&mut self, func: FunctionRef) {
		let function_type = self.lower_type(&func.take_typ());

		self.ssa_library_mut().add_function(&func.borrow().info.link_name(), function_type);
	}

	pub (super) fn lower_extern_func_signature(&mut self, func: ExternFunctionRef) {
		let borrowed = func.borrow();
		let name = &borrowed.info.link_name();
		let function_type = self.lower_type(&func.take_typ());

		self.ssa_library_mut().add_extern_function(name, function_type);
	}

	pub (super) fn lower_func(&mut self, func: FunctionRef) {
		let function = self.ssa_library()
			.get_function(&func.borrow().info.link_name())
			.cloned()
			.unwrap();

		self.context.enter_function(&function);
		for (i, p) in func.borrow().info.params().iter().enumerate() {
			let arg_value = function.arg(i);
			self.context.define_var(&p.bind_name, arg_value);
		}

		let start_block = function.append_block("enter");
		self.builder().position_at_end(&start_block);

		let yield_value = self.lower_code_block(&func.borrow().code);
		if func.borrow().code.typ().kind() != &TypeKind::Divergent {
			self.builder().build_return(yield_value);
		}
	}
}