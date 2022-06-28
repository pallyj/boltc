use std::collections::HashMap;

use enum_::Enum;
use func::Function;
use html::render_docs;
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use struct_::Struct;

mod func;
mod visibility;
mod ty;
mod struct_;
mod enum_;
mod attribute;
mod html;

// todo: typealiases
// todo: attributes
#[derive(Serialize, Deserialize)]
pub struct Library {
    /// The library's name
    pub name:        String,

    /// Whether the library is visible in the documentation
    pub is_visibile: bool,

    /// Functions defined in another library
    pub extern_funcs:Vec<Function>,

    /// Functions defined in the library
    pub functions:   Vec<Function>,

    /// Structs defined in the library
    pub structs:     Vec<Struct>,

    /// Enums defined in the library
    pub enums:       Vec<Enum>
}

impl Library {
    pub fn compose(lib: blir::Library) -> Library {
        let name = lib.name().clone();
        let is_visibile = true;

        // Add typealiases and extern func

        let extern_funcs = lib.extern_functions.iter().filter_map(Function::compose_extern).collect_vec();
        let functions = lib.functions.iter().filter_map(Function::compose).collect_vec();
        let structs = lib.structs.iter().map(Struct::compose).collect_vec();
        let enums = lib.enums.iter().map(Enum::compose).collect_vec();


        Library {
            name,
            is_visibile,
            extern_funcs,
            functions,
            structs,
            enums
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Bundle {
    /// The primary library in the library
    primary: String,

    /// The list of libraries contained in the bundle
    libraries:  HashMap<String, Library>
}

impl Bundle {
    pub fn new(primary: String) -> Self {
        Self {
            primary,
            libraries: HashMap::new()
        }
    }

    pub fn add_library(&mut self, library: blir::Library) {
        let library_name = library.name().clone();
        let library = Library::compose(library);

        self.libraries.insert(library_name, library);
    }

    pub fn get_json_representation(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn save(self) {
        render_docs(self);
    }
}