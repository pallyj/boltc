use std::collections::HashMap;

use itertools::Itertools;

use crate::Project;

use super::{Type, TypeKind};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumId {
	idx: usize,
}

///
/// Represents a tagged union data type
/// 
pub struct Enum {
	id: EnumId,
	name: String,
	repr_type: Type,
	variants: HashMap<u64, Type>
}

impl Enum {
	///
	/// Constructs a new enum without any variants
	/// 
	pub fn new(id: EnumId, name: &str, repr_type: Type) -> Enum {
		Enum {
			id,
			name: name.to_string(),
			repr_type,
			variants: HashMap::new()
		}
	}

	///
	/// The enum's id
	/// 
	pub fn id(&self) -> EnumId {
		self.id
	}

	///
	/// The enum's name
	/// 
	pub fn name(&self) -> &str {
		&self.name
	}

	///
	/// Inserts variants into an enum
	/// 
	pub fn insert_variants(&mut self, variants: Vec<(u64, Type)>) {
		for (key, value) in variants {
			self.variants.insert(key, value);
		}
	}

	///
	/// The type representing the enum's tag
	/// 
	pub fn tag_type(&self) -> Type {
		self.repr_type.clone()
	}

	///
	/// Returns the type of a variant of an enum
	/// 
	pub fn get_variant_type(&self, tag: u64) -> Option<Type> {
		self.variants.get(&tag).cloned()
	}

	pub (crate) fn write(
		&self,
		f: &mut std::fmt::Formatter,
		project: &Project) -> std::fmt::Result
	{
		write!(f, "enum {}: ", self.name)?;
		self.repr_type.write(f, project)?;
		writeln!(f, " {{")?;

		for (key, ty) in self.variants.iter().sorted_by_key(|(key, _)| *key).collect_vec() {
			write!(f, "\t{key}: ")?;
			ty.write(f, project)?;
			writeln!(f)?;
		}

		writeln!(f, "}}")
	}
}

impl EnumId {
	///
	/// Creates an EnumId from an index
	/// 
	pub (crate) fn new(index: usize) -> Self {
		EnumId { idx: index }
	}

	///
	/// 
	/// 
	pub (crate) fn unique_idx(self) -> usize {
		self.idx
	}

	///
	/// Gets a type representing this enum
	/// 
	pub fn type_of(self) -> Type {
		Type { kind: TypeKind::Enum { id: self } }
	}
}