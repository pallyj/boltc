#![feature(let_else)]

use std::{sync::Arc, path::Path, process::Command, os::unix::prelude::CommandExt};

use blir::Library;
use context::LibraryGenContext;
use func::generate_func;
use inkwell::{context::Context, execution_engine::JitFunction, OptimizationLevel, passes::{PassManager, PassManagerBuilder}};

mod typ;
mod func;
mod expr;
mod context;
mod integer;
mod smt;

type FactorialFunc = unsafe extern "C" fn(u64) -> u64;
type ReturnFunc = unsafe extern "C" fn() -> u64;

pub fn codegen_library(lib: &Arc<Library>) {
	let context = Context::create();
	let module = context.create_module(lib.name());
	let builder = context.create_builder();

	let mut fpm = PassManager::create(&module);

	fpm.add_instruction_combining_pass();
	fpm.add_reassociate_pass();
	fpm.add_gvn_pass();
	fpm.add_basic_alias_analysis_pass();
	fpm.add_tail_call_elimination_pass();

	let codegen_ctx = LibraryGenContext::new(&context, &module, &builder, &fpm);

	for func in lib.funcs().iter() {
		generate_func(func, codegen_ctx);
	}

	let name = format!("{}.ll", lib.name());

	module.print_to_file(&name);

	Command::new("llc")
		.args([&name, "-o", &format!("{}.o", lib.name())])
		.spawn()
		.unwrap();
}