use blir::{typ::StructRef, code::MethodRef};
use blirssa::typ::StructField;

use crate::BlirLowerer;

impl BlirLowerer {
	pub (super) fn lower_struct_definition(&mut self, r#struct: StructRef) {
		let ssa_lib = self.ssa_library_mut();

		let is_packed = false;
		let is_transparent = r#struct.borrow().instance_vars.len() == 1;

		let link_name = r#struct.borrow().mangled().mangle();

		r#struct.borrow_mut().link_name = link_name;

		ssa_lib.add_struct(&r#struct.link_name(), is_transparent, is_packed);

		for substruct in r#struct.borrow().substructs.clone() {
			self.lower_struct_definition(substruct);
		}
	}

	pub (super) fn lower_struct_signatures(&mut self, r#struct: StructRef) {
		let borrowed_struct = r#struct.borrow();

		let self_struct = self.ssa_library().get_struct(&borrowed_struct.link_name).cloned().unwrap();

		for substruct in borrowed_struct.substructs.clone() {
			self.lower_struct_signatures(substruct);
		}

		for var in &borrowed_struct.instance_vars {
			let borrowed_var = var.borrow();

			let ty = self.lower_type(&borrowed_var.typ);
			let field = StructField::new(&borrowed_var.name, ty);

			self_struct.add_field(field);
		}
	}
	
	pub (super) fn lower_struct_code(&mut self, r#struct: StructRef) {
		let borrowed_struct = r#struct.borrow();

		for method in &borrowed_struct.methods {
			self.lower_method_signature(method);
		}

		for method in &borrowed_struct.methods {
			self.lower_method(method);
		}
	}

	fn lower_method_signature(&mut self, method: &MethodRef) {
		let function_type = self.lower_type(&method.take_typ());

		let link_name = method.borrow().mangled().mangle();
		method.borrow_mut().link_name = link_name;

		self.ssa_library_mut().add_function(&method.borrow().link_name, function_type);
	}

	pub (super) fn lower_method(&mut self, func: &MethodRef) {
		let method = self.ssa_library()
			.get_function(&func.borrow().link_name)
			.cloned()
			.unwrap();

		self.context.enter_function(&method);
		if !func.is_static() {
			self.context.define_var("self", method.arg(0));
		}
		let offset = if func.is_static() { 0 } else { 1 };
		for (i, p) in func.borrow().params.iter().enumerate() {
			let arg_value = method.arg(i + offset);
			self.context.define_var(&p.bind_name, arg_value);
		}

		let start_block = method.append_block("enter");
		self.builder().position_at_end(&start_block);

		let yield_value = self.lower_code_block(&func.borrow().code);
		self.builder().build_return(yield_value);
	}
}