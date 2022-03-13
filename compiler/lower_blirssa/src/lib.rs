#![feature(let_else)]

use blirssa::{Library, typ::Type};
use func::lower_function;
use inkwell::{context::Context, module::{Module}, builder::Builder};
use struct_::lower_struct;
use typ::lower_function_type;

pub (crate) mod typ;
pub (crate) mod func;
pub (crate) mod value;
pub (crate) mod struct_;

pub fn lower_blirssa_library(library: Library) -> Result<(), String> {
    let context = Context::create();

    let module = context.create_module(library.name());
    let builder = context.create_builder();

    let module_context = ModuleContext { context: &context, module: &module, builder: &builder };

    //let error_context = ErrorContext::new();

    // Create a definition for each struct
    for r#struct in library.structs() {
        if r#struct.transparent_type().is_none() {
            context.opaque_struct_type(r#struct.name());
        }
    }

    // Fill in the fields for the structs
    for r#struct in library.structs() {
        lower_struct(r#struct, &module_context);
    }

    // Create a definition for each function
    for func in library.functions() {
        let Type::Function { return_type, pars } = func.typ() else {
            panic!("Error: Function created with non function type");
        };

        let Some(function_type) = lower_function_type(&*return_type, &pars, &module_context) else {
            return Err(format!("Error creating function type {}", func.typ()));
        };

        module.add_function(func.name(), function_type, None);
    }

    // Codegen each function definition
    for func in library.functions() {
        lower_function(func, &module_context)?;
    }

    // Handle the llvm code
    // Right now, spit it out on the console

    module.print_to_stderr();

    Ok(())
}

pub struct ModuleContext<'a, 'ctx> {
	pub (crate) context: &'ctx Context,
	pub (crate) module: &'a Module<'ctx>,
	pub (crate) builder: &'a Builder<'ctx>,
}