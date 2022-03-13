use std::collections::HashMap;

use crate::{typ::{StructRef, TypeKind}, code::FunctionRef, SymbolKey, Symbol};

pub struct Library {
	name: String,

	symbols: HashMap<SymbolKey, Symbol>,

	functions: Vec<FunctionRef>,
	structs: Vec<StructRef>,
}

impl Library {
	pub fn new(name: String) -> Library {
		Library {
			name,
			symbols: HashMap::new(),
			functions: Vec::new(),
			structs: Vec::new(),
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn add_function(&mut self, func: FunctionRef) -> Option<Symbol> {
		// Add the function to the list of functions
		self.functions.push(func.clone());

		// Add the functions symbol, returning another symbol if it exists
		let (visibility, name) = {
			let func_ref = func.borrow();

			(func_ref.visibility, func_ref.name.clone())
		};

		let key = SymbolKey::new(name, visibility);
		let symbol = Symbol::Function(func);

		self.symbols.insert(key, symbol)
	}

	pub fn add_struct(&mut self, r#struct: StructRef) -> Option<Symbol> {
		// Add the substruct to the list of substructs
		self.structs.push(r#struct.clone());

		// Add the substructs symbol, returning another symbol if it exists
		let (visibility, name) = {
			let substruct_ref = r#struct.borrow();

			(substruct_ref.visibility, substruct_ref.name.clone())
		};

		let key = SymbolKey::new(name, visibility);
		let symbol = Symbol::Type(TypeKind::Struct(r#struct));

		self.symbols.insert(key, symbol)
	}
}