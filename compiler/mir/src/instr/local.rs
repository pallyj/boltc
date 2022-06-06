use std::fmt::Display;

use crate::{ty::Type, val::Place, Project};

///
/// The identifier for a local variable.
/// This is unique across a function
/// 
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocalId {
	id: usize,
}

///
/// A declaration of a local variable
/// The local variable doesn't have a value,
/// it is just a place where a value can be
/// stored.
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct Local {
	id:  LocalId,
	ty: Type
}

impl LocalId {
	///
	/// Creates a new local id
	/// 
	pub (crate) fn new(id: usize) -> LocalId {
		LocalId { id }
	}

	///
	/// The index of this local in the current function
	/// 
	pub (crate) fn local_idx(&self) -> usize {
		self.id
	}
}

impl Local {
	/// 
	/// Creates a new local with an id and a type
	/// 
	pub (crate) fn new(id: LocalId, ty: Type) -> Local {
		Local { id, ty }
	}

	///
	/// Gets the id of the local
	/// 
	pub fn id(&self) -> LocalId {
		self.id
	}

	///
	/// Gets the type of the local
	/// 
	pub fn ty(&self) -> &Type {
		&self.ty
	}

	///
	/// Returns a place for the local that can be used
	/// 
	pub fn place(&self) -> Place {
		todo!()
	}

	///
	/// 
	/// 
	#[allow(unstable_name_collisions)]
	pub (crate) fn write(&self, f: &mut std::fmt::Formatter, project: &Project) -> std::fmt::Result {
		write!(f, "let var {}: ", self.id)?;
		self.ty.write(f, project)
	}
}

impl Display for LocalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "_{}", self.id)
    }
}