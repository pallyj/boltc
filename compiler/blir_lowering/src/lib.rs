#![feature(bool_to_option)]
#![feature(let_else)]

use std::collections::HashMap;

use mir::val::Place;

mod ty;
mod val;
mod code;

///
/// 
/// 
pub struct BlirLowerer<'a> {
    builder: mir::Builder<'a>,
    libraries: Vec<blir::Library>,
    // todo: move these to another struct
    function_ctx: HashMap<String, Place>,
}

impl<'a> BlirLowerer<'a> {
    ///
    /// 
    /// 
    pub fn new(project: &'a mut mir::Project, libraries: Vec<blir::Library>) -> Self {
        let builder = project.builder();

        Self { builder, libraries, function_ctx: HashMap::new() }
    }

    ///
    /// Lowers each library into the mir project.
    /// 
    /// Consumes the `BlirLowerer`. Since the project is passed
    /// as a reference, it is not returned.
    /// 
    /// # Example
    /// 
    ///     fn compile(liraries: Vec<blir::Library>) -> mir::Project {
    ///         let mut project = mir::Project::new("example");
    /// 
    ///         let lowerer = BlirLowerer::new(&mut project, libraries);
    ///         lowerer.lower()
    /// 
    ///         project
    ///     }
    /// 
    pub fn lower(mut self) {
        let libraries = std::mem::take(&mut self.libraries);

        // Create a definition for each type
        // These need to be created first so
        // they can be used before they're defined
        for library in &libraries {
            for struct_def in &library.structs {
                // Lower the struct definition
                self.lower_struct_definition(struct_def)
            }

            for enum_def in &library.enums {
                // Lower the enum definition
            }
        }

        // Lower the signature of each item.
        // Adds fields to structs, variants
        // to enums, and parameters to
        // functions and methods.
        for library in &libraries {
            for struct_def in &library.structs {
                // Lower the struct signature
                self.lower_struct_signature(struct_def)
            }

            for enum_def in &library.enums {
                // Lower the enum signature
            }

            for func_def in &library.extern_functions {
                // Lower the extern function
            }

            for func_def in &library.functions {
                // Lower the function signature
                self.lower_function_signature(func_def);
            }
        }

        // Finally, now that each type and
        // function is defined, lower the
        // code
        for library in &libraries {
            for struct_def in &library.structs {
                // Lower the struct's code
                self.lower_struct_code(struct_def);
            }

            for enum_def in &library.enums {
                // Lower the enum's code
            }

            for func_def in &library.functions {
                // Lower the function's code
                self.lower_function_code(func_def);
            }
        }

        // Lower each closure
    }
}