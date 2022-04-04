use std::fmt::Debug;

use mangle::{Mangled, MangleComponent};

use crate::{
	scope::{ScopeRelation, ScopeRef, ScopeType},
	typ::{StructRef, TypeKind},
	code::{FunctionRef, ExternFunctionRef},
	Symbol,
	SymbolWrapper};

pub struct Library {
	name: String,

	scope: ScopeRef,

	pub mangled: Mangled,

	pub functions: Vec<FunctionRef>,
	pub extern_functions: Vec<ExternFunctionRef>,
	pub structs: Vec<StructRef>,

	files: Vec<ScopeRef>,
}

impl Library {
	pub fn new(name: &str) -> Library {
		Library {
			name: name.to_string(),
			mangled: Mangled::new(MangleComponent::Library(name.to_string())),
			scope: ScopeRef::new(None, ScopeRelation::None, ScopeType::Library, false, false),
			functions: Vec::new(),
			extern_functions: Vec::new(),
			structs: Vec::new(),
			files: vec![],
		}
	}

	pub fn new_file(&mut self) -> ScopeRef {
		let scope = ScopeRef::new(Some(&self.scope), ScopeRelation::SameLibrary, ScopeType::File, false, false);

		self.files.push(scope.clone());

		scope
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn add_function(&mut self, func: FunctionRef) -> Option<SymbolWrapper> {
		// Add the function to the list of functions
		self.functions.push(func.clone());

		// Add the functions symbol, returning another symbol if it exists
		let (visibility, name) = {
			let func_ref = func.borrow();

			(func_ref.visibility, func_ref.name.clone())
		};

		let symbol = Symbol::Function(func);

		self.scope.add_symbol(name, visibility, symbol)
	}

	pub fn add_extern_function(&mut self, func: ExternFunctionRef) -> Option<SymbolWrapper> {
		// Add the function to the list of functions
		self.extern_functions.push(func.clone());

		// Add the functions symbol, returning another symbol if it exists
		let (visibility, name) = {
			let func_ref = func.borrow();

			(func_ref.visibility, func_ref.name.clone())
		};

		let symbol = Symbol::ExternFunction(func);

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

	pub fn mangled(&self) -> &Mangled {
		&self.mangled
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

		for func in &self.extern_functions {
			writeln!(f, "{func:?}")?;
		}

		Ok(())
    }
}