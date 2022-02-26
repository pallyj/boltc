use std::sync::{Arc, Mutex};

use crate::{typ::Type, Visibility, var::GlobalVariableDef, Expr};

#[derive(Clone)]
pub enum SymbolKind {
	/// A symbol that resolves to a type e.g. a struct
	Type(Type),

	/// A symbol that resolves to a global var
	GlobalVar(Arc<Mutex<GlobalVariableDef>>),

	/// A symbol that resolves to a function
	Function(Expr),

	Value(Expr),

}

#[derive(Clone)]
pub struct Symbol {
	visibility: Visibility,
	kind: SymbolKind,
}

impl Symbol {
	pub fn new(kind: SymbolKind, visibility: Visibility) -> Self {
		Self {
			kind,
			visibility
		}
	}

	pub fn kind(&self) -> &SymbolKind {
		&self.kind
	}
}