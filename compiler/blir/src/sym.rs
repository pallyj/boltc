use crate::{typ::{TypeKind}, Visibility, intrinsics::{UnaryIntrinsicFn, BinaryIntrinsicFn}, code::{FunctionRef, MethodRef}};

use std::hash::Hash;

pub enum Symbol {
	UnaryIntrinsicFn(UnaryIntrinsicFn),
	BinaryIntrinsicFn(BinaryIntrinsicFn),

	Function(FunctionRef),
	StaticMethod(MethodRef),
	InstanceMethod(MethodRef),

	Type(TypeKind),
}

#[derive(Eq)]
pub struct SymbolKey {
	visibility: Visibility,
	name: String,
}

impl SymbolKey {
	pub fn new(name: String, visibility: Visibility) -> Self {
		Self {
			name,
			visibility,
		}
	}

	pub fn visibility(&self) -> Visibility {
		self.visibility
	}
}

impl Hash for SymbolKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for SymbolKey {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}