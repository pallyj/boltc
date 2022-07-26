use inkwell::{module::Linkage, AddressSpace};
use mir::val::Global;

use crate::{MirLowerContext};

impl<'a, 'ctx> MirLowerContext<'a, 'ctx>
{
	pub fn create_global(
		&self,
		global: &Global)
	{
		// todo: use llvm global ctors
		let llvm_global_type = self.lower_ty(global.ty());

		let global = self.module.add_global(llvm_global_type,
							   				Some(AddressSpace::Generic),
							   				global.name());

		global.set_linkage(Linkage::Common);
		global.set_alignment(8);
		global.set_initializer(&llvm_global_type.const_zero());
	}

	/*pub fn lower_global_init(&self)
	{
		if let Some(init) = self.module.get_function(".init")
		{
			/*
			// in the future, use the c way
			let ctor_type = self.context.struct_type(&[
				self.context.i32_type().into(),
				self.context.void_type().fn_type(&[], false).ptr_type(AddressSpace::Generic).into(),
				self.context.i8_type().ptr_type(AddressSpace::Generic).into(),
			], false);

			let ctors = self.module.add_global(ctor_type.array_type(1), None, "llvm.global_ctors");

			let initializer = ctor_type.const_array(&[
				ctor_type.const_named_struct(&[
					self.context.i32_type().const_int(65535, false).into(),
					init.as_global_value().as_pointer_value().into(),
					self.context.i8_type().ptr_type(AddressSpace::Generic).const_null().into()
				])
			]);

			ctors.set_linkage(Linkage::Appending);
			ctors.set_section(".ctor");
			ctors.set_initializer(&initializer);*/
		}
	}*/
}