use std::sync::Arc;

use blir::FuncDef;
use inkwell::{types::{BasicType, BasicMetadataTypeEnum}, module::Linkage, values::FunctionValue};

use crate::{context::{LibraryGenContext, FuncGenContext}, typ::generate_type, smt::generate_smt};

pub fn generate_func<'a, 'ctx: 'a>(func: &Arc<FuncDef>, context: LibraryGenContext<'a, 'ctx>) -> FunctionValue<'ctx> {
	let return_type = generate_type(&func.return_type(), context).unwrap();

	let param_types = func.params()
		.iter()
		.map(|p| generate_type(p.typ(), context))
		.map(|p| p.unwrap().into())
		.collect::<Vec<_>>();

	let fn_type = return_type.fn_type(&param_types, false);

	let function = context.module().add_function(func.name(), fn_type, None);

	let entry_block = context.context().append_basic_block(function, func.name());
	context.builder().position_at_end(entry_block);

	let func_ctx = FuncGenContext::new(context, function);

	for smt in func.code().statements() {
		generate_smt(&smt.0, &func_ctx);
	}

	// Codegen the function

	function
}