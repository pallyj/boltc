use std::fmt::Display;

use crate::{instr::LocalId, ty::{Type, TypeKind}};

use super::{RValue, RValueKind};

/// 
/// A place represents an value stored in memory. Like a lvalue in C,
/// a place can be copied, moved, referenced, or assigned to.
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum PlaceKind {
	///
	/// Fetches the value of a local
	/// 
	Local(LocalId),

	///
	/// Gets a field of a struct
	/// 
	StructField(Box<Place>, String),

	///
	/// Casts an enum to a variant as a tuple
	/// 
	CastEnumVariant(Box<Place>, String),

	///
	/// Gets the place of an indexed member of a tuple
	/// 
	TupleItem(Box<Place>, usize),

	///
	/// The dereference of a pointer value
	/// Does NOT move the value out of the pointer
	/// The value can be assigned, copied, moved, or referenced
	/// 
	Deref(RValue)
}

///
/// A place combines a `PlaceKind` with a `Type` and a `Span`
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Place {
	kind: PlaceKind,
	ty: Type
}

impl Place {
	///
	/// Creates a new `Place`
	/// 
	pub (crate) fn new(kind: PlaceKind, ty: Type) -> Place {
		Place { kind, ty }
	}

	///
	/// Moves or copies a value from the place
	/// 
	pub fn copy(&self) -> RValue {
		RValue {
			kind: RValueKind::Copy(Box::new(self.clone())),
			ty: self.ty.clone()
		}
	}

	///
	/// Gets a reference to the place
	/// 
	pub fn get_ref(&self) -> RValue {
		RValue {
			kind: RValueKind::Ref(Box::new(self.clone())),
			ty: self.ty.clone()
		}
	}

	///
	/// Creates a new place referring to a field of another place
	/// 
	pub fn field(&self, field_name: &str, field_ty: Type) -> Place {
		Place { kind: PlaceKind::StructField(Box::new(self.clone()), field_name.to_string()), ty: field_ty }
	}

	///
	/// Casts an enum value to a variant. The new variant is a place, meaning it can be edited.
	/// 
	pub fn cast_variant(self, variant: &str, associated_ty: Type) -> Place {
		Place { kind: PlaceKind::CastEnumVariant(Box::new(self), variant.to_string()), ty: associated_ty }
	}

	///
	/// Retrieves an indexed item from a tuple
	/// 
	pub fn tuple_item(&self, index: usize) -> Place {
		if let TypeKind::Tuple(tuple_items) = &self.ty.kind() {
			let ty = tuple_items[index].clone();
			Place { kind: PlaceKind::TupleItem(Box::new(self.clone()), index), ty }
		} else {
			panic!("Tried to index into a non-tuple value")
		}
	}

	///
	/// A place for a function param
	/// 
	pub fn function_param(n: usize, ty: Type) -> Place {
		Place::new(PlaceKind::Local(LocalId::new(n)), ty)
	}

	///
	/// 
	/// 
	pub fn kind(&self) -> &PlaceKind {
		&self.kind
	}

	///
	/// 
	/// 
	pub fn ty(&self) -> &Type {
		&self.ty
	}
}

impl Display for PlaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::Local(id) => write!(f, "{id}"),
			Self::StructField(place, field) => write!(f, "{place}.{field}"),
			Self::CastEnumVariant(place, variant) => write!(f, "{place} as .{variant}"),
			Self::TupleItem(place, index) => write!(f, "{place}.{index}"),
			Self::Deref(pointer_value) => write!(f, "*{pointer_value}"),
		}
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}