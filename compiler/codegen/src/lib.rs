#![feature(let_else)]

use std::sync::Arc;

use blir::Library;
use context::LibraryGenContext;
use func::generate_func;
use inkwell::context::Context;

mod typ;
mod func;
mod expr;
mod context;
mod integer;
mod smt;

pub fn codegen_library(lib: &Arc<Library>) {
	let context = Context::create();
	let module = context.create_module(lib.name());
	let builder = context.create_builder();

	let codegen_ctx = LibraryGenContext::new(&context, &module, &builder);

	for func in lib.funcs().iter() {
		generate_func(func, codegen_ctx);
	}

	module.print_to_stderr();
}