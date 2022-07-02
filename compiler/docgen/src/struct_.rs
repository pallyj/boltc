use blir::{typ::{StructRef}, value::VarRef};
use itertools::Itertools;
use serde::{Serialize, Deserialize};

use crate::{ty::Type, visibility::Visibility, func::Function, enum_::Enum, attribute::Attribute};

#[derive(Serialize, Deserialize)]
pub struct Struct {
	///
	/// Doc comments on the struct
	/// 
	pub meta: String,

	///
	/// Attributes on the struct
	/// 
	pub attributes: Vec<Attribute>,

	///
	/// The visibility of the struct
	/// 
	pub visibility: Visibility,

	///
	/// The struct's name
	/// 
	pub name: String,

	///
	/// Fields on the struct
	/// 
	pub fields: Vec<StructField>,

	///
	/// The struct's methods
	/// 
	pub methods: Vec<Function>,

	///
	/// Structs defined under this struct
	/// 
	pub substructs: Vec<Struct>,

	///
	/// Enums defined under this struct
	/// 
	pub subenums: Vec<Enum>,

	///
	/// Typealiases defined under this struct
	/// 
	pub typealiases: Vec<()>
}

#[derive(Serialize, Deserialize)]
pub struct StructField {
	///
	/// Meta describing the struct field
	/// 
	pub meta: String,

	///
	/// The visibility of the struct field
	/// 
	pub visibility: Visibility,

	///
	/// Whether the field can be mutated. The field can only be assigned
	/// if the field is variable and the struct is variable
	/// 
	pub is_variable: bool,

	///
	/// The name of the struct field
	/// 
	pub field_name: String,

	///
	/// The type of the field
	/// 
	pub field_type: Type,

	default_value: ()
}

impl Struct {
	pub fn compose(struct_ref: &StructRef) -> Self {
		let meta = struct_ref.meta();
		let attributes = Vec::new();
		let visibility = Visibility::compose(struct_ref.visibility());
		let name = struct_ref.name();
		let fields = struct_ref.borrow().instance_vars.iter().map(StructField::compose).collect_vec();
		let methods = struct_ref.borrow().methods.iter().filter_map(Function::compose_method).collect_vec();
		let substructs = struct_ref.borrow().substructs.iter().map(Struct::compose).collect_vec();
		let subenums = struct_ref.borrow().subenums.iter().map(Enum::compose).collect_vec();

		Struct {
			meta,
			attributes,
			visibility,
			name,
			fields,
			methods,
			substructs,
			subenums,
			typealiases: Vec::new(),
		}
	}

	pub fn hide_below(&mut self, visibility: crate::visibility::Visibility) {
        self.methods.retain(|func| func.visibility >= visibility);
		self.fields.retain(|func| func.visibility >= visibility);
        self.substructs.retain(|func| func.visibility >= visibility);
        self.subenums.retain(|func| func.visibility >= visibility);

        for each_struct in &mut self.substructs {
            each_struct.hide_below(visibility);
        }

        for each_enum in &mut self.subenums {
            each_enum.hide_below(visibility);
        }
    }
}

impl StructField {
	pub fn compose(var: &VarRef) -> StructField {
		let borrowed_var = var.borrow();
		
		let meta = borrowed_var.meta.clone();
		let visibility = Visibility::compose(borrowed_var.visibility);
		let is_variable = !borrowed_var.is_constant;
		let field_name = borrowed_var.name.clone();
		let field_type = Type::compose(&borrowed_var.typ);

		StructField {
			meta,
			visibility,
			is_variable,
			field_name,
			field_type,
			default_value: (),
		}
	}
}