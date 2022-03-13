use std::{sync::Arc, cell::{RefCell, Ref, RefMut}};

use crate::{Visibility, typ::Type, value::Span};

use super::CodeBlock;

pub struct Function {
	pub visibility: Visibility,
	pub name: String,
	pub link_name: String,
	pub params: Vec<FuncParam>,
	pub return_type: Type,
	pub code: CodeBlock,
	pub span: Span
}

pub struct FuncParam {
	pub label: Option<String>,
	pub bind_name: String,
	pub typ: Type,
}

impl Function {
	pub fn new(visibility: Visibility, name: String, params: Vec<FuncParam>, return_type: Type, code: CodeBlock, span: Span) -> FunctionRef {
		let func = Function {
			visibility,
			link_name: name.clone(),
			name,
			params,
			return_type,
			code,
			span
		};

		FunctionRef {
			func: Arc::new(RefCell::new(func))
		}
	}
}

#[derive(Clone)]
pub struct FunctionRef {
	func: Arc<RefCell<Function>>
}

impl FunctionRef {
	pub fn borrow(&self) -> Ref<Function> {
		self.func.borrow()
	}

	pub fn borrow_mut(&mut self) -> RefMut<Function> {
		self.func.borrow_mut()
	}
}