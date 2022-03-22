use std::{sync::Arc, cell::{RefCell, Ref, RefMut}, ops::Deref, fmt::Debug};

use errors::Span;

use crate::{Visibility, typ::{Type, TypeKind}, scope::{ScopeRef, ScopeRelation}, value::ValueKind, Symbol};

use super::{CodeBlock, FuncParam};

pub struct Method {
	inner: RefCell<MethodInner>,
}

#[allow(dead_code)]
pub struct MethodInner {
	pub visibility: Visibility,
	pub is_static: bool,
	pub name: String,
	pub link_name: String,
	pub params: Vec<FuncParam>,
	pub return_type: Type,
	pub code: CodeBlock,
	pub span: Span,
	scope: ScopeRef,
	// Todo: Make this a weak
	self_type: Type,
}

impl MethodInner {
	pub fn scope(&self) -> &ScopeRef {
		&self.scope
	}

	pub fn add_params(&self) {
		let sym = Symbol::Value(ValueKind::SelfVal.anon(self.self_type.clone()));
		self.scope.add_symbol("self".to_string(), Visibility::Public, sym);

		for p in self.params.iter() {
			let val = ValueKind::FunctionParam(p.bind_name.clone())
				.anon(p.typ.clone());
				
			self.scope.add_symbol(p.bind_name.clone(), Visibility::Public, Symbol::Value(val));
		}
	}

	pub fn typ(&self) -> Type {
		if self.is_static {
			let params = self.params.iter().map(|param| param.typ.clone()).collect::<Vec<_>>();

			TypeKind::Function { return_type: Box::new(self.return_type.clone()), params, labels: vec![] }.anon()
		} else {
			let self_iter = std::iter::once(self.self_type.clone());
			let params_iter: Vec<Type> = self.params.iter().map(|param| param.typ.clone()).collect();

			let params = self_iter.chain(params_iter).collect();

			TypeKind::Function { return_type: Box::new(self.return_type.clone()), params, labels: vec![] }.anon()
		}
	}
}

impl Method {
	pub fn new(self_type: Type, is_static: bool, visibility: Visibility, name: String, params: Vec<FuncParam>, return_type: Type, code: CodeBlock, span: Span, parent: &ScopeRef) -> MethodRef {
		let func = MethodInner {
			visibility,
			is_static,
			link_name: name.clone(),
			name,
			params,
			return_type,
			code,
			span,
			scope: ScopeRef::new(Some(parent), ScopeRelation::SameContainer, !is_static),
			self_type,
		};

		MethodRef {
			func: Arc::new(Method { inner: RefCell::new(func) })
		}
	}

	pub fn is_static(&self) -> bool {
		self.inner.borrow().is_static
	}

	pub fn name(&self) -> String {
		self.inner.borrow().name.clone()
	}

	pub fn visibility(&self) -> Visibility {
		self.inner.borrow().visibility
	}

	pub fn borrow(&self) -> Ref<MethodInner> {
		self.inner.borrow()
	}

	pub fn borrow_mut(&self) -> RefMut<MethodInner> {
		self.inner.borrow_mut()
	}
}

#[derive(Clone)]
pub struct MethodRef {
	func: Arc<Method>
}

impl MethodRef {
	pub fn take_typ(&self) -> Type {
		unsafe { &*self.func.inner.as_ptr() }.typ()
	}
}

impl Deref for MethodRef {
    type Target = Method;

    fn deref(&self) -> &Self::Target {
        self.func.deref()
    }
}

impl Debug for MethodRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let method = self.borrow();

		let params = method.params.iter()
			.map(|param| format!("{param:?}"))
			.collect::<Vec<_>>()
			.join(", ");

        write!(f, "{} func {}({}): {:?} {:?}", method.visibility, method.name, params, method.return_type, method.code)
    }
}