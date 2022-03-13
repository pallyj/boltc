use std::{sync::Arc, cell::{RefCell, Ref, RefMut}};

use crate::{Visibility, typ::{Type, StructRef}, value::Span};

use super::{CodeBlock, FuncParam};

pub struct Method {
	pub visibility: Visibility,
	pub is_static: bool,
	pub name: String,
	pub link_name: String,
	pub params: Vec<FuncParam>,
	pub return_type: Type,
	pub code: CodeBlock,
	pub span: Span,
	// Todo: Make this a weak
	self_type: StructRef,
}

impl Method {
	pub fn new(self_type: StructRef, is_static: bool, visibility: Visibility, name: String, params: Vec<FuncParam>, return_type: Type, code: CodeBlock, span: Span) -> MethodRef {
		let func = Method {
			visibility,
			is_static,
			link_name: name.clone(),
			name,
			params,
			return_type,
			code,
			span,
			self_type,
		};

		MethodRef {
			func: Arc::new(RefCell::new(func))
		}
	}

	pub fn reciever(&self) -> &StructRef {
		&self.self_type
	}
}

#[derive(Clone)]
pub struct MethodRef {
	func: Arc<RefCell<Method>>
}

impl MethodRef {
	pub fn borrow(&self) -> Ref<Method> {
		self.func.borrow()
	}

	pub fn borrow_mut(&mut self) -> RefMut<Method> {
		self.func.borrow_mut()
	}
}