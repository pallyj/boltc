use std::{cell::{RefCell, Ref, RefMut}, collections::HashMap, sync::Arc};

use crate::{Visibility, SymbolKey, Symbol, code::MethodRef};

use super::TypeKind;

pub struct Struct {
	pub visibility: Visibility,

	pub name: String,

	pub link_name: String,

	static_symbols: HashMap<SymbolKey, Symbol>,
	instance_symbols: HashMap<SymbolKey, Symbol>,

	substructs: Vec<StructRef>,
	methods: Vec<MethodRef>
}

impl Struct {
	pub fn new(visibility: Visibility, name: String) -> StructRef {
		let r#struct = Struct {
			visibility,
			link_name: name.clone(),
			name: name,
			static_symbols: HashMap::new(),
			instance_symbols: HashMap::new(),
			substructs: Vec::new(),
			methods: Vec::new(),
		};

		StructRef {
			r#struct: Arc::new(RefCell::new(r#struct)),
		}
	}

	pub fn add_substruct(&mut self, substruct: StructRef) -> Option<Symbol> {
		// Add the substruct to the list of substructs
		self.substructs.push(substruct.clone());

		// Add the substructs symbol, returning another symbol if it exists
		let (visibility, name) = {
			let substruct_ref = substruct.borrow();

			(substruct_ref.visibility, substruct_ref.name.clone())
		};

		let key = SymbolKey::new(name, visibility);
		let symbol = Symbol::Type(TypeKind::Struct(substruct));

		self.static_symbols.insert(key, symbol)
	}

	pub fn add_method(&mut self, method: MethodRef) -> Option<Symbol> {
		// Add the function to the list of functions
		self.methods.push(method.clone());

		// Add the functions symbol, returning another symbol if it exists
		let (visibility, name, is_static) = {
			let method_ref = method.borrow();

			(method_ref.visibility, method_ref.name.clone(), method_ref.is_static)
		};

		let key = SymbolKey::new(name, visibility);
		if is_static {
			let symbol = Symbol::StaticMethod(method);

			self.static_symbols.insert(key, symbol)
		} else {
			let symbol = Symbol::InstanceMethod(method);

			self.instance_symbols.insert(key, symbol)
		}
	}
}

#[derive(Clone)]
pub struct StructRef {
	r#struct: Arc<RefCell<Struct>>,
}

impl StructRef {
	pub fn borrow(&self) -> Ref<Struct> {
		self.r#struct.borrow()
	}

	pub fn borrow_mut(&self) -> RefMut<Struct> {
		self.r#struct.borrow_mut()
	}
}

impl PartialEq for StructRef {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.r#struct, &other.r#struct)
    }
}

impl Eq for StructRef {
    fn assert_receiver_is_total_eq(&self) {}
}