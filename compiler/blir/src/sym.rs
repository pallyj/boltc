use crate::{
	code::{FunctionRef, MethodRef},
	scope::{ScopeRelation},
	value::{VarRef, Value},
	typ::{TypeKind},
	Visibility
};

#[derive(Debug, Clone)]
pub enum Symbol {
	Value(Value),

	Function(FunctionRef),
	StaticMethod(MethodRef),
	InstanceMethod(MethodRef),

	InstanceVariable(VarRef),

	Type(TypeKind),
}

#[derive(Clone)]
pub struct SymbolWrapper {
	sym: Symbol,
	vis: Visibility
}

impl SymbolWrapper {
	pub fn new(sym: Symbol, vis: Visibility) -> Self {
		Self {
			sym,
			vis
		}
	}
	pub fn filter(self, relation: ScopeRelation) -> Option<SymbolWrapper> {
		if relation.can_access(self.vis) {
			Some(self)
		} else {
			None
		}
	}
	pub fn resolve(self) -> Symbol {
		self.sym
	}
}