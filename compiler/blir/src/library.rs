use std::fmt::Debug;

use crate::{
	scope::{ScopeRelation, ScopeRef},
	typ::{StructRef, TypeKind},
	code::FunctionRef,
	Symbol,
	SymbolWrapper};

pub struct Library {
	name: String,

	scope: ScopeRef,

	pub functions: Vec<FunctionRef>,
	pub structs: Vec<StructRef>,
}

impl Library {
	pub fn new(name: &str) -> Library {
		Library {
			name: name.to_string(),
			scope: ScopeRef::new(None, ScopeRelation::None, false),
			functions: Vec::new(),
			structs: Vec::new(),
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn add_function(&mut self, func: FunctionRef) -> Option<SymbolWrapper> {
		// Add the function to the list of functions
		self.functions.push(func.clone());

		// Add the functions symbol, returning another symbol if it exists
		let (visibility, name, typ) = {
			let func_ref = func.borrow();

			(func_ref.visibility, func_ref.name.clone(), func_ref.typ())
		};

		let symbol = Symbol::Function(func);

		self.scope.add_symbol(name, visibility, symbol)
	}

	pub fn add_struct(&mut self, r#struct: StructRef) -> Option<SymbolWrapper> {
		// Add the substruct to the list of substructs
		self.structs.push(r#struct.clone());

		// Add the substructs symbol, returning another symbol if it exists
		let visibility = r#struct.visibility();
		let name = r#struct.name();

		let symbol = Symbol::Type(TypeKind::Struct(r#struct));
		self.scope.add_symbol(name, visibility, symbol)
	}

	pub fn scope(&self) -> &ScopeRef {
		&self.scope
	}
}

impl Debug for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for r#struct in &self.structs {
			writeln!(f, "{struct:?}")?;
		}

        for func in &self.functions {
			writeln!(f, "{func:?}")?;
		}

		Ok(())
    }
}