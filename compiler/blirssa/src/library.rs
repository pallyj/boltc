use std::{collections::{hash_map::Values, HashMap},
          fmt::Display};

use crate::{code::{ExternFunction, ExternFunctionRef, Function, FunctionRef},
            typ::{Struct, StructRef, Type, Enum, EnumRef}};

pub struct Library {
    name:             String,
    enums:            HashMap<String, EnumRef>,
    structs:          HashMap<String, StructRef>,
    functions:        HashMap<String, FunctionRef>,
    extern_functions: HashMap<String, ExternFunctionRef>,
}

impl Library {
    pub fn new(name: &str) -> Library {
        Library { name:             name.to_string(),
                  enums:            HashMap::new(),
                  structs:          HashMap::new(),
                  functions:        HashMap::new(),
                  extern_functions: HashMap::new(), }
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn add_struct(&mut self, name: &str, is_transparent: bool, is_packed: bool) {
        let r#struct = Struct::new(name.to_string(), is_transparent, is_packed);

        self.structs.insert(name.to_string(), r#struct);
    }

    pub fn add_enum(&mut self, name: &str, bits: u64) {
        let enum_def = Enum::new(name.to_string(), bits);

        self.enums.insert(name.to_string(), enum_def);
    }

    pub fn get_struct(&self, name: &str) -> Option<&StructRef> { self.structs.get(name) }

    pub fn get_enum(&self, name: &str) -> Option<&EnumRef> { self.enums.get(name) }

    pub fn structs(&self) -> Values<String, StructRef> { self.structs.values() }

    pub fn enums(&self) -> Values<String, EnumRef> { self.enums.values() }

    pub fn add_function(&mut self, name: &str, function_type: Type) {
        let function = Function::new(name, function_type);

        // Check if the function already exists

        self.functions.insert(name.to_string(), function);
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionRef> { self.functions.get(name) }

    pub fn add_extern_function(&mut self, name: &str, function_type: Type) {
        let function = ExternFunction::new(name, function_type);

        // Check if the function already exists

        self.extern_functions.insert(name.to_string(), function);
    }

    pub fn get_extern_function(&self, name: &str) -> Option<&ExternFunctionRef> { self.extern_functions.get(name) }

    pub fn functions(&self) -> Values<String, FunctionRef> { self.functions.values() }

    pub fn extern_functions(&self) -> Values<String, ExternFunctionRef> { self.extern_functions.values() }
}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r#struct in &self.structs {
            writeln!(f, "{}", r#struct.1)?;
        }
        for r#enum in &self.enums {
            writeln!(f, "{}", r#enum.1)?;
        }
        for func in &self.functions {
            writeln!(f, "{}", func.1)?;
        }

        Ok(())
    }
}
