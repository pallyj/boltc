use blir::typ::{EnumRef, Type};
use itertools::Itertools;

use crate::BlirLowerer;

impl<'a> BlirLowerer<'a> {
	///
	/// 
	/// 
	pub (crate) fn lower_enum_definition(
		&mut self,
		enum_def: &EnumRef)
	{
		// Add the enum to the project
		let enum_name = enum_def.mangle();
		let repr_type = self.lower_ty(&enum_def.repr_type());

		self.builder.add_enum(&enum_name, repr_type);

		enum_def.set_link_name(enum_name);

		// Go through each child and lower its definition
		for substruct in enum_def.substructs().iter() {
			self.lower_struct_definition(substruct)
		}

		for subenum in enum_def.subenums().iter() {
			self.lower_enum_definition(subenum)
		}
	}

	///
	/// Adds variants to the enum and its children
	/// 
	pub (crate) fn lower_enum_signature(
		&mut self,
		enum_def: &EnumRef)
	{
		// Add the fields to the enum
		let variants =
			enum_def.variants().iter()
							 .map(|variant| {
								 let ty = variant.associated_types()
								 				 .iter()
												 .map(|ty| self.lower_ty(ty))
												 .collect_vec();
								 
								(variant.tag() as u64, mir::ty::Type::tuple(ty))
							 })
							 .collect_vec();

		self.builder.add_enum_variants(enum_def.link_name(), variants);

		// Add each method's signature to the function
		for method in enum_def.methods().iter() {
			self.lower_method_signature(method);
		}

		// Now lower each child struct
		for substruct in enum_def.substructs().iter() {
			self.lower_struct_signature(substruct);
		}

		// And each child enum
		for subenum in enum_def.subenums().iter() {
			self.lower_enum_signature(subenum)
		}
	}

	///
	/// 
	/// 
	pub (crate) fn lower_enum_code(
		&mut self,
		enum_def: &EnumRef)
	{
		// Lower each substruct
		for substruct in enum_def.substructs().iter() {
			self.lower_struct_code(substruct);
		}

		// Lower each subenum
		for subenum in enum_def.subenums().iter() {
			self.lower_enum_code(subenum)
		}

		// Lower each method
		for method in enum_def.methods().iter() {
			self.lower_method_code(method);
		}
	}
}