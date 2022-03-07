use std::sync::Arc;

use crate::{sym::Symbol, Expr, TypeKind};

/// A trait which 
pub trait Scope {
	fn parent(&self) -> Option<Arc<dyn Scope>>;

	fn name(&self) -> &str;

	fn symbol(&self) -> mangle::symbol::Symbol;


	fn lookup_symbol(&self, name: &String) -> Option<Symbol>;

	fn define_expr(&self, name: String, value: Expr);

	fn scoped_type(&self, name: &str) -> Option<TypeKind>;

	fn take_index(&self) -> u64;
}

pub enum ScopeKind {
	Library,

	Enum,
	Class,
	Struct,
	Protocol,

	Function,
	Method,

	IfBlock,

}

// Levels
// - Lower ast into blir
// - Lookup declaration types
// - Check declaration types
// - Lookup func parameter types
// - Infer types in function bodies
// - Check types/members in function bodies
//
// - Lower blir into blirssa
// - Optimize
// - Lower blirssa into llvm
// - Optimize
// - Lower llvm into assembly