mod struct_;

pub use struct_::{Struct, StructId};

use crate::Project;

/// 
/// A `Type` without the span
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
	Integer { bits: u32 },
	Float { bits: u32 },
	Struct { id: StructId },
	// Enum { id: EnumId },
	Pointer(Box<Type>),
	Tuple (Vec<Type>),
	Array {
		item: Box<Type>,
		count: usize,
	},
	Function {
		parameters: Vec<Type>,
		return_type: Box<Type>
	}
}


///
/// A `Type` incorporates a `TypeKind` and a `Span` where it comes from
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
	kind: TypeKind,
	// span: Span
}

impl Type {
	/// 
	/// An array of the type with count `n`
	/// 
	pub fn array(self, n: usize) -> Type {
		Type { kind: TypeKind::Array { item: Box::new(self),
									   count: n } }
	}

	///
	/// A function returning the type and parameters `parameters`
	/// 
	pub fn func(self, parameters: Vec<Type>) -> Type {
		Type { kind: TypeKind::Function { parameters,
										  return_type: Box::new(self) } }
	}

	///
	/// A tuple with the specified items
	/// 
	pub fn tuple(items: Vec<Type>) -> Type {
		Type { kind: TypeKind::Tuple(items) }
	}


	/// 
	/// An empty tuple
	/// 
	pub fn void() -> Type {
		Type { kind: TypeKind::Tuple(vec![]) }
	}


	/// 
	/// A pointer to the type
	/// 
	pub fn pointer(self) -> Type {
		Type { kind: TypeKind::Pointer(Box::new(self)) }
	}

	/// 
	/// An integer type 
	/// 
	pub fn int(bits: u32) -> Type {
		Type { kind: TypeKind::Integer { bits } }
	}

	///
	/// A float type
	/// 
	pub fn float(bits: u32) -> Type {
		Type { kind: TypeKind::Float { bits } }
	}

	///
	/// 
	/// 
	pub fn c_struct(id: StructId) -> Type {
		Type { kind: TypeKind::Struct { id } }
	}


	///
	/// What type this is
	/// 
	pub fn kind(&self) -> &TypeKind {
		&self.kind
	}

	///
	/// 
	/// 
	#[allow(unstable_name_collisions)]
	pub (crate) fn write(&self, f: &mut std::fmt::Formatter, project: &Project) -> std::fmt::Result {
		match self.kind() {
			TypeKind::Integer { bits } => write!(f, "i{bits}"),
			TypeKind::Float { bits } => write!(f, "f{bits}"),

			TypeKind::Struct { .. } => write!(f, "any struct"),

			TypeKind::Pointer(resolving) => {
				write!(f, "&")?;
				resolving.write(f, project)
			}
			TypeKind::Tuple(items) => {
				write!(f, "(")?;
				items.iter()
					 .map(|item| { item.write(f, project)?; write!(f, ", ")})
					 .collect::<std::fmt::Result>()?;
				write!(f, ")")
			}

														 
			TypeKind::Array { item, count } => {
				write!(f, "[")?;
				item.write(f, project)?;
				write!(f, "; {count}]")
			}

			TypeKind::Function { parameters, return_type } => {
				write!(f, "func (")?;

				parameters.iter()
						  .map(|item| { item.write(f, project)?; write!(f, ", ")})
					 	  .collect::<std::fmt::Result>()?;

				write!(f, ") -> ")?;
				return_type.write(f, project)
			}
		}
	}
}