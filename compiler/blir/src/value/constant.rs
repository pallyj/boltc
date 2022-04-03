use std::{cell::{Ref, RefCell, RefMut}, sync::Arc, ops::Deref, fmt::Debug};

use crate::{Visibility, value::Value};

use super::Type;

pub struct ConstantInner {
	pub visibility: Visibility,

	pub name: String,

	pub typ: Type,

	pub value: Value
}

pub struct Constant {
	constant: RefCell<ConstantInner>,
}

impl Constant {
	pub fn new(visibility: Visibility, name: String, typ: Type, value: Value) -> ConstantRef {
		let constant_inner = ConstantInner {
			visibility,
			name,
			typ,
			value
		};

		ConstantRef { constant: Arc::new(Constant { constant: RefCell::new(constant_inner) }) }
	}

	pub fn borrow(&self) -> Ref<ConstantInner> {
		self.constant.borrow()
	}

	pub fn borrow_mut(&self) -> RefMut<ConstantInner> {
		self.constant.borrow_mut()
	}
}

#[derive(Clone)]
pub struct ConstantRef {
	constant: Arc<Constant>,
}

impl Deref for ConstantRef {
    type Target = Constant;

    fn deref(&self) -> &Self::Target {
        self.constant.deref()
    }
}

impl Debug for ConstantRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{visibility} static let {name}: {typ:?} = {value:?}", visibility = self.borrow().visibility, name = self.borrow().name, typ = self.borrow().typ, value = self.borrow().value)
    }
}