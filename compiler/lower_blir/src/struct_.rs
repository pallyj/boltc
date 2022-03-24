use blir::{typ::StructRef, code::MethodRef};
use blirssa::typ::StructField;

use crate::BlirLowerer;

impl BlirLowerer {
	pub (super) fn lower_struct_definition(&mut self, r#struct: StructRef) {
		let ssa_lib = self.ssa_library_mut();

		let is_packed = false;
		let is_transparent = r#struct.borrow().instance_vars.len() == 1;

		// TODO: Use link name
		ssa_lib.add_struct(&r#struct.name(), is_transparent, is_packed);

		for substruct in r#struct.borrow().substructs.clone() {
			self.lower_struct_definition(substruct);
		}
	}

	pub (super) fn lower_struct_signatures(&mut self, r#struct: StructRef) {
		let borrowed_struct = r#struct.borrow();

		let self_struct = self.ssa_library().get_struct(&borrowed_struct.name).cloned().unwrap();

		for var in &borrowed_struct.instance_vars {
			let borrowed_var = var.borrow();

			let ty = self.lower_type(&borrowed_var.typ);
			let field = StructField::new(&borrowed_var.name, ty);

			self_struct.add_field(field);
		}

		for method in &borrowed_struct.methods {
			self.lower_method_signature(method);
		}


		for substruct in borrowed_struct.substructs.clone() {
			self.lower_struct_signatures(substruct);
		}
	}

	fn lower_method_signature(&mut self, method: &MethodRef) {
		let name = method.name();
		let function_type = self.lower_type(&method.take_typ());

		self.ssa_library_mut().add_function(&name, function_type);
	}
}