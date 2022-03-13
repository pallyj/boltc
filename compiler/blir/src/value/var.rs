use std::{sync::Arc, cell::{RefCell, Ref, RefMut}};

use crate::{typ::Type, Visibility};

use super::Value;

pub struct Variable {
	pub visibility: Visibility,
	pub name: String,
	pub typ: Type,
	pub value: Option<Value>,
}

impl Variable {
	pub fn new(visibility: Visibility, name: String, typ: Type, value: Option<Value>) -> VariableRef {
		let var = Variable {
			visibility,
			name,
			typ,
			value
		};

		VariableRef {
			var: Arc::new(RefCell::new(var))
		}
	}
}

#[derive(Clone)]
pub struct VariableRef {
	var: Arc<RefCell<Variable>>,
}

impl VariableRef {
	pub fn borrow(&self) -> Ref<Variable> {
		self.var.borrow()
	}

	pub fn borrow_mut(&mut self) -> RefMut<Variable> {
		self.var.borrow_mut()
	}
}