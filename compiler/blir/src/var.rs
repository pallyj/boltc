use std::{sync::Weak, fmt::Debug};

use crate::{Visibility, typ::Type, library::Library, scope::Scope, expr::Expr};

#[derive(Clone)]
pub struct GlobalVariableDef {
	is_mutable: bool,

	visibility: Visibility,
	
	name: String,

	typ: Type,

	default_value: Expr,

	library: Weak<Library>
}

#[derive(Clone)]
pub struct VariableDef {
	is_mutable: bool,

	visibility: Visibility,

	name: String,

	typ: Type,

	default_value: Expr,

	container: Weak<dyn Scope>,
}