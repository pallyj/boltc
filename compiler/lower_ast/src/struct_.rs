use std::sync::Arc;

use prelude::*;
use bolt_ast::{
	Struct as AstStruct, StructItem
};
use blir::{
	StructDef as BlirStruct, Scope
};

use crate::{lower_method, lower_var};

///
///  Lowers the abstract syntax tree of a struct into Bolt IR
/// 
pub fn lower_struct(r#struct: WithSource<AstStruct>, scope: &Arc<dyn Scope>) -> Try<Arc<BlirStruct>, ()> {
	let (r#struct, source) = r#struct.unwrap();

	let visibility = lower_visibility(r#struct.visibility().clone());
	let name = r#struct.name().clone();

	let ir_struct = BlirStruct::new(scope, visibility, name.unwrap_or("".to_string()), source);
	let struct_scope: Arc<dyn Scope> = ir_struct.clone();

	for struct_item in r#struct.into_items() {
		let (struct_item, source) = struct_item.unwrap();

		match struct_item {
			StructItem::SubStruct(substruct) => {
				let ir_substruct = require!(lower_struct(substruct.with_source(source), &struct_scope));

				if let Err(_err) = ir_struct.add_substruct(ir_substruct) {
					//error_ctx.raise(err);
				}
			}
			StructItem::Method(method) => {
				let ir_method = require!(lower_method(method.with_source(source), &struct_scope));

				if let Err(_err) = ir_struct.add_method(ir_method) {
					// error_ctx.raise(err);
				}
			}
			StructItem::Variable(var) => {
				let ir_var = require!(lower_var(var.with_source(source), &struct_scope));

				if let Err(_err) = ir_struct.add_variable(ir_var) {
					// error_ctx.raise(err);
				}
			}
			_ => {
				// Do nothing yet, we haven't implemented these
			}
		}
	}


	return Try::Some(ir_struct)
}

pub fn lower_visibility(vis: Option<WithSource<bolt_ast::Visibility>>) -> blir::Visibility {
	let Some(vis) = vis else {
		return blir::Visibility::Internal;
	};

	match vis.value() {
		bolt_ast::Visibility::Public => blir::Visibility::Public,
		bolt_ast::Visibility::Internal => blir::Visibility::Internal,
		bolt_ast::Visibility::Fileprivate => blir::Visibility::Fileprivate,
		bolt_ast::Visibility::Private => blir::Visibility::Private
	}
}