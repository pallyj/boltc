use std::{sync::Arc, cell::{RefCell, Ref, RefMut}, ops::Deref, fmt::Debug};

use errors::Span;
use mangle::{Mangled, MangleComponent};

use crate::{Visibility, typ::{Type, TypeKind}, scope::{ScopeRef, ScopeRelation, ScopeType}, value::ValueKind, Symbol, attributes::Attributes};

use super::{CodeBlock, FuncParam, FunctionInfo};

pub struct Method {
	inner: RefCell<MethodInner>,
}

#[allow(dead_code)]
pub struct MethodInner {
	pub attributes: Attributes,
	pub visibility: Visibility,
	pub is_static: bool,
	pub info: FunctionInfo,
	pub code: CodeBlock,
	pub span: Span,
	scope: ScopeRef,
	self_type: Type,
	parent_mangled: Mangled,
}

impl MethodInner {
	pub fn scope(&self) -> &ScopeRef {
		&self.scope
	}

	pub fn add_params(&self) {
		let sym = Symbol::Value(ValueKind::SelfVal.anon(self.self_type.clone()));
		if !self.is_static {
			self.scope.add_symbol("self".to_string(), Visibility::Public, sym);
			self.scope.define_scope_type("self", self.self_type.clone());
		}

		self.scope.define_scope_type("return", self.info.return_type().clone());

		for p in self.info.params().iter() {
			let val = ValueKind::FunctionParam(p.bind_name.clone())
				.anon(p.typ.clone());
				
			self.scope.add_symbol(p.bind_name.clone(), Visibility::Public, Symbol::Value(val));
		}
	}

	pub fn typ(&self) -> Type {
		if self.is_static {
			let params = self.info.params().iter().map(|param| param.typ.clone()).collect::<Vec<_>>();

			TypeKind::Function { return_type: Box::new(self.info.return_type().clone()), params, labels: vec![] }.anon()
		} else {
			let params = self.info.params().iter().map(|param| param.typ.clone()).collect::<Vec<_>>();

			TypeKind::Method { reciever: Box::new(self.self_type.clone()), return_type: Box::new(self.info.return_type().clone()), params }.anon()
		}
	}

	pub fn mangled(&self) -> Mangled {
		self.parent_mangled
			.clone()
			.append(MangleComponent::Function(self.info.name().clone()))
	}
}

impl Method {
	pub fn new(attributes: Attributes, self_type: Type, is_static: bool, visibility: Visibility, name: String, params: Vec<FuncParam>, return_type: Type, code: CodeBlock, span: Span, parent: &ScopeRef, parent_mangled: Mangled) -> MethodRef {
		let func = MethodInner {
			attributes,
			visibility,
			is_static,
			info: FunctionInfo::new(name, params, return_type, true),
			code,
			span,
			scope: ScopeRef::new(Some(parent), ScopeRelation::SameContainer, ScopeType::Code, !is_static, true),
			self_type,
			parent_mangled
		};

		MethodRef {
			func: Arc::new(Method { inner: RefCell::new(func) })
		}
	}

	pub fn is_static(&self) -> bool {
		self.inner.borrow().is_static
	}

	pub fn name(&self) -> String {
		self.inner.borrow().info.name().clone()
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

		let params = method.info.params().iter()
			.map(|param| format!("{param:?}"))
			.collect::<Vec<_>>()
			.join(", ");

        write!(f, "{} func {}({}): {:?} {:?}", method.visibility, method.info.name(), params, method.info.return_type(), method.code)
    }
}