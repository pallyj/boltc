use std::sync::Arc;

use blir::StructDef;
use inkwell::types::{StructType, BasicTypeEnum};

use crate::{context::LibraryGenContext, func::generate_method, typ::generate_type};

pub fn generate_struct<'a, 'ctx>(r#struct: &Arc<StructDef>, context: LibraryGenContext<'a, 'ctx>) {
	let struct_type = context.types().get_type(r#struct.link_name().as_str()).into_struct_type();

	let body_types = r#struct
		.variables()
		.iter()
		.map(|var| generate_type(&var.typ(), context).unwrap())
		.collect::<Vec<_>>();
	
	struct_type.set_body(&body_types, false);

	context.types().define_type(r#struct.link_name().clone(), BasicTypeEnum::StructType(struct_type));

	for method in r#struct.methods().iter() {
		generate_method(&method, context);
	}
}

// Components
// Frontend
//  - Lexer
//  - Parser
// Mid end
//  - ast
//  - lower_ast
//  - blir
//  - lower_blir
//  - blirssa
// Backend
//  - codegen
//  - llvm
