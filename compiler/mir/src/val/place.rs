use std::fmt::Display;

use errors::Span;

use crate::{instr::LocalId, ty::{Type, TypeKind}};

use super::{RValue, RValueKind, GlobalId};

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
	/// The value of a global variable
	/// 
	Global(GlobalId),

	///
	/// Gets a field of a struct
	/// 
	StructField(Box<Place>, String),

	///
	/// Casts an enum to a variant as a tuple
	/// 
	CastEnumVariant(Box<Place>, u64, String),

	///
	/// Gets the place of an indexed member of a tuple
	/// 
	TupleItem(Box<Place>, usize),

	///
	/// Gets the item at an index in an array
	/// 
	ArrayIndex(Box<Place>, RValue),

	///
	/// The dereference of a pointer value
	/// Does NOT move the value out of the pointer
	/// The value can be assigned, copied, moved, or referenced
	/// 
	Deref(RValue),

	///
	/// Gets the discriminant of an enum
	/// 
	Discriminant(Box<Place>),
}

///
/// A place combines a `PlaceKind` with a `Type` and a `Span`
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Place {
	kind: PlaceKind,
	is_mutable: bool,
	ty: Type,
	span: Span,
}

impl Place {
	///
	/// Creates a new `Place`
	/// 
	pub (crate) fn new(kind: PlaceKind, ty: Type, is_mutable: bool, span: Span) -> Place {
		Place { kind, ty, is_mutable, span }
	}

	///
	/// Moves or copies a value from the place
	/// 
	pub fn copy(&self, span: Span) -> RValue {
		RValue {
			kind: RValueKind::Copy(Box::new(self.clone())),
			ty: self.ty.clone(),
			span
		}
	}

	///
	/// Gets a reference to the place
	/// 
	pub fn get_ref(&self, span: Span) -> RValue {
		RValue {
			kind: RValueKind::Ref(Box::new(self.clone())),
			ty: self.ty.clone(),
			span
		}
	}

	///
	/// Creates a new place referring to a field of another place
	/// 
	pub fn field(&self, field_name: &str, field_ty: Type, span: Span) -> Place {
		Place { kind: PlaceKind::StructField(Box::new(self.clone()), field_name.to_string()), ty: field_ty, is_mutable: self.is_mutable, span }
	}

	///
	/// Casts an enum value to a variant. The new variant is a place, meaning it can be edited.
	/// 
	pub fn cast_variant(&self, variant: u64, variant_name: &str, associated_ty: Type, span: Span) -> Place {
		let is_mutable = self.is_mutable;
		Place { kind: PlaceKind::CastEnumVariant(Box::new(self.clone()), variant, variant_name.to_string()), ty: associated_ty, is_mutable, span }
	}

	///
	/// Retrieves an indexed item from a tuple
	/// 
	pub fn tuple_item(&self, index: usize, span: Span) -> Place {
		if let TypeKind::Tuple(tuple_items) = &self.ty.kind() {
			let ty = tuple_items[index].clone();
			Place { kind: PlaceKind::TupleItem(Box::new(self.clone()), index), ty, is_mutable: self.is_mutable, span }
		} else {
			panic!("Tried to index into a non-tuple value {:?}", self.ty())
		}
	}

	///
	/// Retrieves an indexed item from an array
	/// 
	pub fn array_index(&self, index: RValue, span: Span) -> Place {
		if let TypeKind::Array { item, .. } = &self.ty.kind() {
			Place { kind: PlaceKind::ArrayIndex(Box::new(self.clone()), index), ty: item.as_ref().clone(), is_mutable: self.is_mutable, span }
		} else {
			panic!("Tried to index into a non-array value {:?}", self.ty())
		}
	}

	///
	/// A place for a function param
	/// 
	pub fn function_param(n: usize, ty: Type, span: Span) -> Place {
		Place::new(PlaceKind::Local(LocalId::new(n)), ty, false, span)
	}

	///
	/// Get the discriminant of an enum value
	/// 
	pub fn discriminant(&self, discrim_ty: Type, span: Span) -> Self {
		let is_mutable = self.is_mutable;
		Place { kind: PlaceKind::Discriminant(Box::new(self.clone())), ty: discrim_ty, is_mutable, span }
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

	///
	/// 
	/// 
	pub fn is_mutable(&self) -> bool {
		self.is_mutable
	}

	///
	/// 
	/// 
	pub fn span(&self) -> Span {
		self.span
	}
}

impl Display for PlaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use PlaceKind::*;

        match self {
			Local(id) => write!(f, "{id}"),
			Global(id) => write!(f, "{id}"),
			StructField(place, field) => write!(f, "{place}.{field}"),
			CastEnumVariant(place, _, variant_name) => write!(f, "{place} as .{variant_name}"),
			TupleItem(place, index) => write!(f, "{place}.{index}"),
			Deref(pointer_value) => write!(f, "*{pointer_value}"),
			Discriminant(place) => write!(f, "discriminant {place}"),
			ArrayIndex(place, idx) => write!(f, "index {idx} of {place}"),
		}
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}