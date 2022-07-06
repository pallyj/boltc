use std::fmt::Display;

use errors::Span;

use crate::{ty::Type, Project};

use super::{Place, PlaceKind};

///
/// A `GlobalId` is a way to refer to a `Global` in a project
/// 
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GlobalId {
	index: usize,
}

impl GlobalId {
	///
	/// Creates a new `Global`
	/// 
	pub (crate) fn new(index: usize) -> Self {
		Self {
			index
		}
	}

	///
	/// Gets the index of the `Global`
	/// 
	pub (crate) fn index(&self) -> usize {
		self.index
	}
}

///
/// A `Global` holds a static variable. The global can be referenced
/// anywhere in the project.
/// 
pub struct Global {
	///
	/// The id of the `Global`
	/// 
	id: GlobalId,

	///
	/// The name of the `Global`
	/// 
	name: String,

	///
	/// The type of the `Global`
	/// 
	ty: Type
}

impl Global {
	///
	/// Creates a new `Global`. For internal use only.
	/// 
	pub (crate) fn new(id: GlobalId, name: String, ty: Type) -> Self {
		Self {
			id,
			name,
			ty }
	}

	///
	/// Gets an id that can be used to refer to this global
	/// 
	pub fn id(&self) -> GlobalId {
		self.id
	}

	///
	/// Gets the name of the `Global`
	/// 
	pub fn name(&self) -> &str {
		&self.name
	}

	///
	/// Gets the type of the `Global`
	/// 
	pub fn ty(&self) -> Type {
		self.ty.clone()
	}

	///
	/// Gets a `Place` referring to this global
	/// 
	pub fn place(&self, span: Span) -> Place {
		Place::new(PlaceKind::Global(self.id),
				   self.ty.clone(),
				   true,
				   span)
	}

	///
	/// Writes a `Global` to a formatter. Needs a project reference
	/// 
	pub fn write(
		&self,
		f: &mut std::fmt::Formatter,
		lib: &Project) -> std::fmt::Result
	{
		write!(f, "global {}: ", self.name)?;
		self.ty.write(f, lib)
	}
}

impl Display for GlobalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.index)
    }
}