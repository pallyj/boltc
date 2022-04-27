#![feature(let_else)]

use blirssa::{typ::Type, Library};
use enum_::lower_enum;
use func::lower_function;
use inkwell::{builder::Builder,
              context::Context,
              module::{Linkage, Module}};
use struct_::lower_struct;
use typ::lower_function_type;

pub(crate) mod func;
pub(crate) mod struct_;
pub(crate) mod typ;
pub(crate) mod value;
pub(crate) mod enum_;

pub fn lower_blirssa_library(library: Library, context: &Context) -> Result<Module, String> {
    let module = context.create_module(library.name());
    let builder = context.create_builder();

    let module_context = ModuleContext { context,
                                         module: &module,
                                         builder: &builder };

    // let error_context = ErrorContext::new();

    // Create a definition for each struct
    for r#struct in library.structs() {
        if r#struct.transparent_type().is_none() {
            context.opaque_struct_type(r#struct.name());
        }
    }

    // Create a definition for each enum
    for r#enum in library.enums() {
        context.opaque_struct_type(r#enum.name());
    }

    // Fill in the fields for the structs
    for r#struct in library.structs() {
        lower_struct(r#struct, &module_context);
    }

    // Fill in the fields for the structs
    for r#enum in library.enums() {
        lower_enum(r#enum, &module_context);
    }

    // Create a definition for each extern function
    for func in library.extern_functions() {
        let Type::Function { return_type, pars } = func.typ() else {
            panic!("Error: Function created with non function type");
        };

        let Some(function_type) = lower_function_type(&*return_type, &pars, &module_context) else {
            return Err(format!("Error creating function type {}", func.typ()));
        };

        module.add_function(func.name(), function_type, Some(Linkage::External));
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

    Ok(module)
}

pub struct ModuleContext<'a, 'ctx> {
    pub(crate) context: &'ctx Context,
    pub(crate) module:  &'a Module<'ctx>,
    pub(crate) builder: &'a Builder<'ctx>,
}
