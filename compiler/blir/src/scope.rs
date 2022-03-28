use std::{sync::{Arc, Weak}, collections::HashMap, cell::RefCell};

use crate::{Visibility, SymbolWrapper, Symbol, value::ValueKind, typ::Type};


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScopeRelation {
	None,
	SameLibrary,
	SameFile,
	SameContainer,

}

impl ScopeRelation {
	pub fn can_access(self, visibility: Visibility) -> bool {
		match visibility {
			Visibility::Public => true,
			Visibility::Internal => self != ScopeRelation::None,
			Visibility::Fileprivate => (self == ScopeRelation::SameFile || self == ScopeRelation::SameContainer),
			Visibility::Private => self == ScopeRelation::SameContainer,
		}
	}
}

#[derive(Clone)]
pub struct ScopeRef {
	inner: Arc<RefCell<Scope>>,
}

impl ScopeRef {
	pub fn new(parent: Option<&ScopeRef>, relation: ScopeRelation, lookup_parent_instance: bool, is_function: bool) -> ScopeRef {
		let scope = Scope {
			parent: parent.map(|parent| Arc::downgrade(&parent.inner)),
			symbols: HashMap::new(),
			instance_symbols: HashMap::new(),
			scope_types: HashMap::new(),
			imports: Vec::new(),
			lookup_parent_instance,
			relation,
			counter: 1,
			is_function,
		};

		ScopeRef { inner: Arc::new(RefCell::new(scope)) }
	}

	pub fn add_symbol(&self, name: String, vis: Visibility, sym: Symbol) -> Option<SymbolWrapper> {
		self.inner.borrow_mut().add_symbol(name, vis, sym)
	}

	pub fn add_instance_symbol(&self, name: String, vis: Visibility, sym: Symbol) -> Option<SymbolWrapper> {
		self.inner.borrow_mut().add_instance_symbol(name, vis, sym)
	}

	pub fn import(&self, scope: ScopeRef) {
		self.inner.borrow_mut().import(scope);
	}

	pub fn lookup_symbol(&self, name: &str) -> Option<SymbolWrapper> {
		self.inner.borrow().lookup_symbol(name)
	}

	pub fn lookup_instance_member(&self, name: &str) -> Option<SymbolWrapper> {
		self.inner.borrow().lookup_instance_member(name)
	}

	pub fn lookup_static_member(&self, name: &str) -> Option<SymbolWrapper> {
		self.inner.borrow().lookup_static_member(name)
	}

	pub fn define_variable(&self, name: &str, typ: Type) -> String {
		self.inner.borrow_mut().define_variable(name, typ)
	}

	pub fn define_scope_type(&self, name: &str, ty: Type) {
		self.inner
			.borrow_mut()
			.define_scope_type(name, ty)
	}

	pub fn scope_type(&self, name: &str) -> Option<Type> {
		self.inner
			.borrow()
			.scope_type(name)
	}
}

struct Scope {
	parent: Option<Weak<RefCell<Scope>>>,
	symbols: HashMap<String, SymbolWrapper>,
	imports: Vec<ScopeRef>,
	instance_symbols: HashMap<String, SymbolWrapper>,
	scope_types: HashMap<String, Type>,
	lookup_parent_instance: bool,
	relation: ScopeRelation,
	counter: u64,
	is_function: bool
}

impl Scope {
	fn add_symbol(&mut self, name: String, vis: Visibility, sym: Symbol) -> Option<SymbolWrapper> {
		let wrapper = SymbolWrapper::new(sym, vis);

		self.symbols.insert(name, wrapper)
	}

	fn add_instance_symbol(&mut self, name: String, vis: Visibility, sym: Symbol) -> Option<SymbolWrapper> {
		let wrapper = SymbolWrapper::new(sym, vis);

		self.instance_symbols.insert(name, wrapper)
	}

	fn import(&mut self, scope: ScopeRef) {
		self.imports.push(scope);
	}

	fn lookup_symbol(&self, name: &str) -> Option<SymbolWrapper> {
		if let Some(sym) = self.symbols.get(name) {
			return Some(sym.clone())
		}

		if let Some(sym) = self.parent()
			.map(|parent| {
				if self.lookup_parent_instance {
					if let Some(sym) = parent.borrow().lookup_instance_symbol(name) {
						return sym.filter(self.relation)
					}
				} else {
					if let Some(sym) = parent.borrow().lookup_symbol(name) {
						return sym.filter(self.relation)
					}
				}
				None
			}) {
				return sym
			}


		// THEN lookup imports

		self.imports
			.iter()
			.find_map(|scope| scope.lookup_symbol(name)
				.and_then(|sym| sym.filter(self.relation))
			)
	}

	fn lookup_instance_symbol(&self, name: &str) -> Option<SymbolWrapper> {
		if let Some(sym) = self.instance_symbols.get(name) {
			return Some(sym.clone())
		}

		self.lookup_symbol(name)
	}

	fn lookup_static_member(&self, name: &str) -> Option<SymbolWrapper> {
		if let Some(sym) = self.symbols.get(name) {
			return Some(sym.clone())
		}

		None
	}

	fn lookup_instance_member(&self, name: &str) -> Option<SymbolWrapper> {
		if let Some(sym) = self.instance_symbols.get(name) {
			return Some(sym.clone())
		}

		None
	}

	fn parent(&self) -> Option<Arc<RefCell<Scope>>> {
		self.parent
			.as_ref()
			.and_then(|parent| parent.upgrade())
	}

	fn next_index(&mut self) -> u64 {
		if self.is_function {
			let idx = self.counter;

			self.counter += 1;

			idx
		} else {
			self.parent
				.as_ref()
				.map(|parent| parent.upgrade().unwrap().borrow_mut().next_index())
				.unwrap_or(0)
		}
	}


	fn define_variable(&mut self, name: &str, typ: Type) -> String {
		let idx = self.next_index();

		let mangled_name = format!("var{idx}_{name}");

		let sym = Symbol::Value(ValueKind::LocalVariable(mangled_name.clone()).anon(typ));

		self.add_symbol(name.to_string(),Visibility::Public, sym);

		mangled_name
	}

	fn define_scope_type(&mut self, name: &str, ty: Type) {
		self.scope_types
			.insert(name.to_string(), ty);
	}

	fn scope_type(&self, name: &str) -> Option<Type> {
		if let Some(ty) = self.scope_types.get(name) {
			return Some(ty.clone())
		} else {
			return self.parent().unwrap().borrow().scope_type(name)
		}
	}
}