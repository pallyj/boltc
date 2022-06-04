use std::collections::HashMap;

use itertools::Itertools;

use crate::{ty::{Type, TypeKind}, Project};

use super::val::Value;

///
/// 
/// 
#[allow(dead_code)]
pub struct Var {
	ty: Type,
	value: Value
}

impl Var {
	///
	/// 
	/// 
	pub (crate) fn new(ty: Type, project: &Project) -> Var {
		let init_value = Self::init_value(&ty, project);

		Self { ty, value: init_value }
	}

	///
	fn init_value(ty: &Type, project: &Project) -> Value {
		use TypeKind::*;
		
		match ty.kind() {
			Struct { id } => {
				let struct_def = project.get_struct(*id).expect("struct doesn't exist");
				let mut fields = HashMap::new();

				for field_name in struct_def.field_names() {
					let field_ty = struct_def.field_type(field_name).expect("");

					fields.insert(field_name.to_string(), Self::init_value(field_ty, project));
				}

				Value::Struct(fields)
			}
			//Pointer(_) => todo!(),
			Tuple(tuple_items) => Value::Tuple(tuple_items.iter().map(|v| Self::init_value(v, project)).collect_vec()),
			//Array { item, count } => todo!(),
			_ => Value::Undef,
		}
	}

	///
	/// 
	/// 
	pub fn get(&self) -> &Value {
		&self.value
	}

	///
	/// 
	/// 
	pub fn get_mut(&mut self) -> &mut Value {
		&mut self.value
	}

	///
	/// 
	/// 
	pub fn set(&mut self, val: Value) {
		self.value = val;
	}
}