use std::collections::{HashMap, hash_map::{Values}};

use crate::{code::{FunctionRef, Function}, typ::{Type, StructRef, Struct}};

pub struct Library {
	name: String,
	structs: HashMap<String, StructRef>,
	functions: HashMap<String, FunctionRef>
}

impl Library {
	pub fn new(name: String) -> Library {
		Library {
			name,
			structs: HashMap::new(),
			functions: HashMap::new(),
		}		
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn add_struct(&mut self, name: &str, is_transparent: bool, is_packed: bool) {
		let r#struct = Struct::new(name.to_string(), is_transparent, is_packed);

		self.structs.insert(name.to_string(), r#struct);
	}

	pub fn get_struct(&mut self, name: &str) -> Option<&StructRef> {
		self.structs.get(name)
	}

	pub fn structs(&self) -> Values<String, StructRef> {
		self.structs.values()
	}

	pub fn add_function(&mut self, name: &str, function_type: Type) {
		let function = Function::new(name, function_type);

		// Check if the function already exists

		self.functions.insert(name.to_string(), function);
	}

	pub fn get_function(&self, name: &str) -> Option<&FunctionRef> {
		self.functions.get(name)
	}

	pub fn functions(&self) -> Values<String, FunctionRef> {
		self.functions.values()
	}
}