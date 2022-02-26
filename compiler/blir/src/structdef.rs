use std::{collections::HashMap, sync::{Arc, Mutex}};

use prelude::Source;

use crate::{typ::Type, Visibility, method::MethodDef, var::VariableDef};

#[derive(Clone)]
pub struct StructDef {
	// Attributes
	
	visibility: Visibility,

	name: String,

	implements: Vec<Type>,

	variables: HashMap<String, VariableDef>,

	methods: HashMap<String, MethodDef>,

	substructs: HashMap<String, Arc<Mutex<StructDef>>>
}

impl StructDef {
	pub fn new(visibility: Visibility, name: String, source: Source) -> Arc<Mutex<StructDef>> {
		Arc::new(Mutex::new(Self {
			visibility,
			name,
			implements: vec![],
			variables: HashMap::new(),
			methods: HashMap::new(),
			substructs: HashMap::new(),
		}))
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn add_substruct(&mut self, substruct: Arc<Mutex<StructDef>>) -> Result<(), ()> {
		let name = substruct.lock().unwrap().name().clone();

		if self.substructs.insert(name, substruct).is_some() {
			// Error
		}

		Ok(())
	}
}