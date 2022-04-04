use std::{sync::{Arc, Weak}, cell::{RefCell, Ref, RefMut}, fmt::Debug, ops::Deref};

use errors::Span;

use crate::{Visibility, typ::{Type, TypeKind}, scope::ScopeRef};

use super::FuncParam;


#[derive(Clone)]
pub struct ExternFunctionInner {
	pub visibility: Visibility,
	pub name: String,
	pub link_name: String,
	pub params: Vec<FuncParam>,
	pub return_type: Type,
	pub span: Span,
	pub parent: ScopeRef,
}

impl ExternFunctionInner {
	pub fn typ(&self) -> Type {
		let params = self.params.iter().map(|param| param.typ.clone()).collect::<Vec<_>>();

		TypeKind::Function { return_type: Box::new(self.return_type.clone()), params, labels: vec![] }.anon()
	}
}

pub struct ExternFunction {
	inner: RefCell<ExternFunctionInner>
}

impl ExternFunction {
	pub fn new(visibility: Visibility, name: String, params: Vec<FuncParam>, return_type: Type, span: Span, parent: &ScopeRef) -> ExternFunctionRef {
		let func = ExternFunctionInner {
			visibility,
			link_name: name.clone(),
			name,
			params,
			return_type,
			span,
			parent: parent.clone()
		};
		

		ExternFunctionRef {
			func: Arc::new(ExternFunction { inner: RefCell::new(func) })
		}
	}
}

#[derive(Clone)]
pub struct ExternFunctionRef {
	func: Arc<ExternFunction>
}

impl Deref for ExternFunctionRef {
    type Target = ExternFunction;

    fn deref(&self) -> &Self::Target {
        self.func.deref()
    }
}

impl ExternFunctionRef {
	pub fn take_typ(&self) -> Type {
		unsafe { &*self.func.inner.as_ptr() }.typ()
	}
}

impl ExternFunction {
	pub fn borrow(&self) -> Ref<ExternFunctionInner> {
		self.inner.borrow()
	}

	pub fn borrow_mut(&self) -> RefMut<ExternFunctionInner> {
		self.inner.borrow_mut()
	}
}

impl Debug for ExternFunctionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let func = self.borrow();

		let params = func.params.iter()
			.map(|param| format!("{param:?}"))
			.collect::<Vec<_>>()
			.join(", ");

        write!(f, "{} func {}({}): {:?}", func.visibility, func.name, params, func.return_type)
    }
}