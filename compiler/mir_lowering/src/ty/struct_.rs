use itertools::Itertools;

use crate::MirLowerContext;

impl<'a, 'ctx> MirLowerContext<'a, 'ctx>
{
	pub fn create_struct(&self, struct_def: &mir::ty::Struct)
	{
		if struct_def.is_transparent() {
			return
		}

		let struct_name = struct_def.name();
		//let struct_id = struct_def.id();

		self.context.opaque_struct_type(struct_name);
	}

	pub fn fill_struct_fields(
		&self,
		struct_def: &mir::ty::Struct)
	{
		if struct_def.is_transparent() {
			return
		}

		let is_packed = struct_def.is_packed();
		let fields = struct_def.fields()
							   .iter()
							   .cloned()
							   .map(|field| self.lower_ty(field))
							   .collect_vec();

		let llvm_struct = self.module.get_struct_type(struct_def.name())
									 .expect("compiler error: couldn't find struct");

		llvm_struct.set_body(&fields, is_packed);
	}
}