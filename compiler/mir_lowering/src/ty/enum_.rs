use mir::ty::Enum;

use crate::MirLowerContext;

impl<'a, 'ctx> MirLowerContext<'a, 'ctx>
{
	pub fn create_enum(
		&self,
		enum_def: &Enum)
	{
		let enum_name = enum_def.name();

		self.context.opaque_struct_type(enum_name);
	}

	pub fn fill_enum_variants(
		&self,
		enum_def: &Enum)
	{
		let llvm_enum = self.module.get_struct_type(enum_def.name())
								   .expect("compiler error: couldn't find enum");

		let tag_type = self.lower_ty(enum_def.tag_type());

		// Find the maximum sized variant
		// todo: add data layout to mir
		let maximum_variant_size =
			enum_def.variants()
					.map(|(_, ty)| {
						let llvm_ty = self.lower_ty(ty.clone());
						self.layout.get_abi_size(&llvm_ty) as u32
					})
					.max()
					.unwrap_or(0);

		llvm_enum.set_body(&[
			tag_type, // tag type
			self.context.struct_type(&[], false).into(), // shared enum type
			self.context.i8_type().array_type(maximum_variant_size).into() // enum type
		], false);
	}
}