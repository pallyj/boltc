use std::{sync::{Weak, atomic::{AtomicUsize, Ordering}, Arc, Mutex, MutexGuard}, fmt::Debug};

use crate::{Visibility, typ::Type, library::Library, scope::Scope, expr::Expr};

#[derive(Clone)]
pub struct GlobalVariableDef {
	is_mutable: bool,

	visibility: Visibility,
	
	name: String,

	typ: Type,

	default_value: Expr,

	library: Weak<Library>
}

pub struct VariableDef {
	is_mutable: bool,

	visibility: Visibility,

	name: String,

	typ: Mutex<Type>,

	default_value: Option<Expr>,

	field_index: AtomicUsize,

	container: Weak<dyn Scope>,
}

impl VariableDef {
	pub fn new(is_mutable: bool, visibility: Visibility, name: String, typ: Type, default_value: Option<Expr>, container: &Arc<dyn Scope>) -> Arc<VariableDef> {
		Arc::new(VariableDef {
			is_mutable,
			visibility,
			name,
			typ: Mutex::new(typ),
			default_value,
			field_index: AtomicUsize::new(0),
			container: Arc::downgrade(container),
		})
	}

	pub fn is_mutable(&self) -> bool {
		self.is_mutable
	}

	pub fn visibility(&self) -> Visibility {
		self.visibility.clone()
	}

	pub fn name(&self) -> &String {
		return &self.name
	}

	pub fn typ(&self) -> MutexGuard<Type> {
		self.typ.lock().unwrap()
	}

	pub fn default_value(&self) -> Option<&Expr> {
		self.default_value.as_ref()
	}

	pub fn field_index(&self) -> usize {
		self.field_index.load(Ordering::Relaxed)
	}

	pub fn set_field_index(&self, idx: usize) {
		self.field_index.store(idx, Ordering::Relaxed);
	}

	pub fn parent(&self) -> Arc<dyn Scope> {
		self.container.upgrade().unwrap()
	}
}