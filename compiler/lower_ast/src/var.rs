use std::sync::Arc;

use prelude::*;
use bolt_ast::Var;
use blir::{VariableDef as BlirVar, Scope, Visibility};
use type_infer::type_infer_ctx;

use crate::{lower_type, lower_expr};

pub fn lower_var(var: WithSource<Var>, context: &Arc<dyn Scope>) -> Try<Arc<BlirVar>, ()> {
	let visibility = Visibility::Public;
	let name = var.value().name().clone();
	let typ = require!(var.value().typ()
		.map(|typ| lower_type(typ.clone()))
		.unwrap_or_else(|| Try::Some(type_infer_ctx())));
	let default_value = match var.value().default_value() {
		Some(default) => Some(require!(lower_expr(default.clone()))),
		None => None
	};

	Try::Some(BlirVar::new(true, visibility, name, typ, default_value, context))
}