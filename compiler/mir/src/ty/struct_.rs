use std::{collections::HashMap};

use crate::Project;

use super::{Type, TypeKind};

///
/// An identifier used to find a struct in a project
/// 
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StructId {
	id: usize
}

///
/// A struct is a datatype consisting of types arranged one after another
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
	id: StructId,

	is_transparent: bool,
	is_packed: bool,

	name: String,

	fields: Vec<Type>,
	field_indices: HashMap<String, usize>,
}

impl StructId {
	///
	/// 
	/// 
	pub fn new(id: usize) -> Self {
		StructId { id }
	}

	pub fn ty(self) -> Type {
		Type::c_struct(self)
	}
}

impl Struct {
	///
	/// 
	/// 
	pub (crate) fn new(id: StructId, name: String, is_transparent: bool, is_packed: bool) -> Struct {
		Struct { id,
				 is_transparent,
				 is_packed,
				 name,
				 fields: vec![],
				 field_indices: HashMap::new() }
	}

	pub fn id(&self) -> StructId {
		self.id
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn field_index(&self, field_name: &str) -> Option<usize> {
		self.field_indices.get(field_name).copied()
	}

	pub fn field_type(&self, field_name: &str) -> Option<&Type> {
		let field_index = self.field_index(field_name)?;
		self.fields.get(field_index)
	}

	pub fn is_transparent(&self) -> bool {
		self.is_transparent
	}

	pub fn is_packed(&self) -> bool {
		self.is_packed
	}

	pub fn fields(&self) -> &Vec<Type> {
		&self.fields
	}

	///
	/// 
	/// 
	pub fn field_names(&self) -> impl Iterator<Item = &String> {
		self.field_indices.keys()
	}

	///
	/// 
	/// 
	pub fn insert_fields(&mut self, fields: Vec<(String, Type)>) {
		for field in fields {
			self.field_indices.insert(field.0, self.fields.len());
			self.fields.push(field.1);
		}
	}

	///
	/// Writes a struct to a formatter
	/// 
	pub (crate) fn write(&self, f: &mut std::fmt::Formatter, project: &Project) -> std::fmt::Result {
		if self.is_packed { writeln!(f, "@packed")?; }
		if self.is_transparent { writeln!(f, "@transparent")?; }

		writeln!(f, "struct {} {{", self.name())?;

		for field in self.field_indices.keys() {
			let ty = self.field_type(field).expect("");
			write!(f, "\t{}: ", field)?;
			ty.write(f, project)?;
			writeln!(f)?;
		}

		writeln!(f, "}}")
	}
}

impl StructId {
	///
	/// The unique index of the struct within the project
	/// 
	pub (crate) fn unique_idx(&self) -> usize {
		self.id
	}

	///
	/// The type of an instance of this struct
	/// 
	pub fn type_of(self) -> Type {
		Type { kind: TypeKind::Struct { id: self } }
	}
}