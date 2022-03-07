use std::sync::{Arc};

use blir::{Library, FileScope, Scope};
use bolt_ast::{Decl};
use prelude::{WithSource, Try, require, HasSource};

use crate::{lower_struct, lower_func, lower_extern_func};


pub fn lower_file(decls: Vec<WithSource<Decl>>, library: Arc<Library>) -> Try<(), ()> {
	let file = FileScope::new(&library);

	let dyn_file: Arc<dyn Scope> = file.clone();

	for decl in decls {
		let (decl, source) = decl.unwrap();

		match decl {
			Decl::Import(import) => {
				file.import(&import.library())
			}

			Decl::Struct(s) => {
				let lowered_struct = require!(lower_struct(s.with_source(source), &dyn_file));

				library.define_struct(lowered_struct);
			}

			Decl::Func(f) => {
				if f.code().is_some() {
					let lowered_func = require!(lower_func(f.with_source(source), &dyn_file));
					library.define_function(lowered_func);
				} else {
					let lowered_func = require!(lower_extern_func(f.with_source(source), &dyn_file));
					library.define_extern_function(lowered_func);
				}
			}

			_ => {}
		}
	}

	library.add_file(file);

	Try::Some(())
	
}