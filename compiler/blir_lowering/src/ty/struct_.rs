use blir::typ::StructRef;
use itertools::Itertools;

use crate::BlirLowerer;

impl<'a> BlirLowerer<'a> {
	///
	/// 
	/// 
	pub (crate) fn lower_struct_definition(
		&mut self,
		struct_def: &StructRef)
	{
		// Add this struct to the project
		let mut borrowed_struct = struct_def.borrow_mut();

		let struct_name = borrowed_struct.mangle();
		let is_transparent = borrowed_struct.is_transparent;
		let is_packed = false;

		self.builder.add_struct(&struct_name, is_transparent, is_packed);

		borrowed_struct.link_name = struct_name;

		// Go through each child and lower its definition
		for substruct in &borrowed_struct.substructs {
			self.lower_struct_definition(substruct)
		}

		for subenum in &borrowed_struct.subenums {

		}
	}

	///
	/// Adds fields to the struct and its children
	/// 
	pub (crate) fn lower_struct_signature(
		&mut self,
		struct_def: &StructRef)
	{
		let borrowed_struct = struct_def.borrow();

		// Add the fields to the struct
		let fields =
			borrowed_struct.instance_vars.iter()
										 .map(|instance_var| {
										 	 let borrowed_instance = instance_var.borrow();
											 let ty = self.lower_ty(&borrowed_instance.typ);
 
										 	 (borrowed_instance.name.clone(), ty)
										 })
										 .collect_vec();
        // todo: struct names aren't mangled
		self.builder.add_struct_fields(&borrowed_struct.link_name, fields);

		// Add each method's signature to the function
		for method in &borrowed_struct.methods {
			self.lower_method_signature(method);
		}

		// Now lower each child struct
		for substruct in &borrowed_struct.substructs {
			self.lower_struct_signature(substruct);
		}

		// And each child enum
		for subenum in &borrowed_struct.subenums {

		}
	}

	pub (crate) fn lower_struct_code(
		&mut self,
		struct_def: &StructRef)
	{
		let borrowed_struct = struct_def.borrow();

		// Lower each substruct
		for substruct in &borrowed_struct.substructs {
			self.lower_struct_code(substruct);
		}

		// Lower each subenum
		for subenum in &borrowed_struct.subenums {

		}

		// Lower each method
		for method in &borrowed_struct.methods {
			self.lower_method_code(method);
		}
	}
}