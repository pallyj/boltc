#![feature(let_else)]

use std::{sync::Arc, process::Command};

use blir::Library;
use context::{LibraryGenContext, TypeContainer};
use func::{generate_func, generate_extern_func};
use inkwell::{context::Context, passes::{PassManager}, types::{BasicTypeEnum, BasicType}};
use structdef::generate_struct;
use typ::generate_type;

mod typ;
mod func;
mod expr;
mod context;
mod integer;
mod smt;
mod structdef;

pub fn codegen_library(lib: &Arc<Library>) {
	let context = Context::create();
	let module = context.create_module(lib.name());
	let builder = context.create_builder();
	let types = TypeContainer::new();

	let fpm = PassManager::create(&module);

	fpm.add_instruction_combining_pass();
	fpm.add_reassociate_pass();
	fpm.add_gvn_pass();
	fpm.add_basic_alias_analysis_pass();
	fpm.add_tail_call_elimination_pass();

	let codegen_ctx = LibraryGenContext::new(&context, &module, &builder, &fpm, &types);

	for r#struct in lib.structs().iter() {
		let name = r#struct.link_name().clone();

		types.define_type(name.clone(), context.opaque_struct_type(&name).as_basic_type_enum());
	}

	for r#struct in lib.structs().iter() {
		generate_struct(r#struct, codegen_ctx);
	}

	for extern_func in lib.extern_funcs().iter() {
		generate_extern_func(extern_func, codegen_ctx);
	}

	for func in lib.funcs().iter() {
		let return_type = generate_type(&func.return_type(), codegen_ctx).unwrap();

		let param_types = func.params()
			.iter()
			.map(|p| generate_type(p.typ(), codegen_ctx))
			.map(|p| p.unwrap().into())
			.collect::<Vec<_>>();

		let fn_type = return_type.fn_type(&param_types, false);

		codegen_ctx.module().add_function(&func.link_name(), fn_type, None);
	}

	for func in lib.funcs().iter() {
		generate_func(func, codegen_ctx);
	}

	let name = format!("{}.ll", lib.name());

	module.print_to_file(&name).unwrap();

	Command::new("llc")
		.args(["--filetype=obj", &name, "-o", &format!("{}.o", lib.name())])
		.spawn()
		.unwrap();
}