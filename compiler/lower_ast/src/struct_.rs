use std::{collections::HashSet, sync::{Arc, Mutex}};

use prelude::*;
use bolt_ast::{
	Struct as AstStruct, StructItem
};
use blir::{
	StructDef as BlirStruct
};

///
///  Lowers the abstract syntax tree of a struct into Bolt IR
/// 
pub fn lower_struct(r#struct: WithSource<AstStruct>) -> Try<Arc<Mutex<BlirStruct>>, ()> {
	let (r#struct, source) = r#struct.unwrap();

	let visibility = lower_visibility(r#struct.visibility().clone());
	let name = r#struct.name().clone();

	let ir_struct = BlirStruct::new(visibility, name.unwrap_or("".to_string()), source);

	let mut ir_struct_view = ir_struct.lock().unwrap();

	// TODO: Move this into BlirStruct and add a symbol field
	let mut symbols = HashSet::new();

	for struct_item in r#struct.into_items() {
		let (struct_item, source) = struct_item.unwrap();

		match struct_item {
			StructItem::SubStruct(substruct) => {
				let ir_substruct = require!(lower_struct(substruct.with_source(source)));

				{
					let ir_substruct_view = ir_substruct.lock().unwrap();

					// Move this into add_substruct
					let substruct_name = ir_substruct_view.name().clone();
					symbols.insert(substruct_name);
				}

				if let Err(err) = ir_struct_view.add_substruct(ir_substruct) {
					//error_ctx.raise(err);
				}
			}
			_ => {
				// Do nothing yet, we haven't implemented these
			}
		}
	}

	drop(ir_struct_view);

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