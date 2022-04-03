use std::{sync::{Arc, Weak}, collections::HashMap, cell::RefCell};

use crate::{Visibility, SymbolWrapper, Symbol, value::ValueKind, typ::Type};


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScopeRelation {
	None,
	SameLibrary,
	SameFile,
	SameContainer,

}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ScopeType {
	Code,
	Container,
	File,
	Library
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
	pub fn new(parent: Option<&ScopeRef>, relation: ScopeRelation, ty: ScopeType, lookup_parent_instance: bool, is_function: bool) -> ScopeRef {
		let scope = Scope {
			parent: parent.map(|parent| Arc::downgrade(&parent.inner)),
			symbols: HashMap::new(),
			instance_symbols: HashMap::new(),
			scope_types: HashMap::new(),
			imports: Vec::new(),
			lookup_parent_instance,
			relation,
			ty,
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

	pub fn typ(&self) -> ScopeType {
		self.inner
			.borrow()
			.ty
	}

	pub fn parent(&self) -> Option<ScopeRef> {
		self.inner
			.borrow()
			.parent()
			.map(|inner| ScopeRef { inner })
	}

	pub fn relation_to(&self, other_scope: &ScopeRef) -> ScopeRelation {
		let mut self_iter = ScopeIter::new(self);
		let mut other_iter = ScopeIter::new(other_scope);

		// Check if other scope is a child of this scope's container
		let self_container = self_iter.next(ScopeType::Container);
		while let Some(other_container) = other_iter.next(ScopeType::Container) {
			if self_container
				.as_ref()
				.map(|self_container| self_container == &other_container)
				.unwrap_or(false) { return ScopeRelation::SameContainer }
		}

		// Check if this scope is a child of other_scope
		while let Some(self_container) = self_iter.next(ScopeType::Container) {
			if &self_container == other_scope { return ScopeRelation::SameContainer }
		}

		// Check if both scopes are in the same file
		let self_file = self_iter.next(ScopeType::File);
		let other_file = other_iter.next(ScopeType::File);

		if self_file.zip(other_file)
			.map(|(self_file, other_file)| self_file == other_file)
			.unwrap_or(false) { return ScopeRelation::SameFile };

		// Check if both scopes are in the same library
		let self_library = self_iter.next(ScopeType::Library);
		let other_library = other_iter.next(ScopeType::Library);

		if self_library.zip(other_library)
			.map(|(self_library, other_library)| self_library == other_library)
			.unwrap_or(false) { return ScopeRelation::SameLibrary };

		return ScopeRelation::None
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
	ty: ScopeType,
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
			return self.parent().and_then(|parent| parent.borrow().scope_type(name))
		}
	}
}

impl PartialEq for ScopeRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for ScopeRef {
    fn assert_receiver_is_total_eq(&self) {}
}

pub struct ScopeIter {
	scope: Option<ScopeRef>
}

impl ScopeIter {
	pub fn new(scope: &ScopeRef) -> Self {
		Self {
			scope: Some(scope.clone())
		}
	}

	pub fn next(&mut self, ty: ScopeType) -> Option<ScopeRef> {
		while let Some(scope) = self.scope.as_ref() {
			if scope.typ() == ty {
				let scope = self.scope.take().unwrap();
				self.scope = scope.parent();
				return Some(scope)
			} else {
				self.scope = scope.parent();
			}
		}

		return None
	}
}