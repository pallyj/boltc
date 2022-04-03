use std::{sync::Arc, cell::{RefCell, Ref, RefMut}, fmt::Debug, ops::Deref};

use errors::Span;
use mangle::{Mangled, MangleComponent};

use crate::{Visibility, typ::{Type, TypeKind}, scope::{ScopeRef, ScopeRelation, ScopeType}, value::ValueKind, Symbol};

use super::CodeBlock;

#[derive(Clone)]
pub struct FunctionInner {
	pub visibility: Visibility,
	pub name: String,
	pub link_name: String,
	pub params: Vec<FuncParam>,
	pub return_type: Type,
	pub code: CodeBlock,
	pub span: Span,
	scope: ScopeRef,
	parent_mangled: Mangled,
}

impl FunctionInner {
	pub fn add_params(&self) {
		self.scope.define_scope_type("return", self.return_type.clone());
		
		for p in self.params.iter() {
			let val = ValueKind::FunctionParam(p.bind_name.clone())
				.anon(p.typ.clone());
				
			self.scope.add_symbol(p.bind_name.clone(), Visibility::Public, Symbol::Value(val));
		}
	}

	pub fn typ(&self) -> Type {
		let params = self.params.iter().map(|param| param.typ.clone()).collect::<Vec<_>>();

		TypeKind::Function { return_type: Box::new(self.return_type.clone()), params, labels: vec![] }.anon()
	}

	pub fn scope(&self) -> &ScopeRef {
		&self.scope
	}

	pub fn mangled(&self) -> Mangled {
		self.parent_mangled
			.clone()
			.append(MangleComponent::Function(self.name.clone()))
	}
}

pub struct Function {
	inner: RefCell<FunctionInner>
}

#[derive(Clone)]
pub struct FuncParam {
	pub label: Option<String>,
	pub bind_name: String,
	pub typ: Type,
}

impl Debug for FuncParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.label {
			write!(f, "{label} {bind_name}: {ty:?}", bind_name = self.bind_name, ty = self.typ)
		} else {
			write!(f, "{bind_name}: {ty:?}", bind_name = self.bind_name, ty = self.typ)
		}
    }
}

impl Function {
	pub fn new(visibility: Visibility, name: String, params: Vec<FuncParam>, return_type: Type, code: CodeBlock, span: Span, parent: &ScopeRef, parent_mangled: Mangled) -> FunctionRef {
		let func = FunctionInner {
			visibility,
			link_name: name.clone(),
			name,
			params,
			return_type,
			code,
			span,
			scope: ScopeRef::new(Some(parent), ScopeRelation::SameFile, ScopeType::Code, false, true),
			parent_mangled,
		};
		

		FunctionRef {
			func: Arc::new(Function { inner: RefCell::new(func) })
		}
	}
}

#[derive(Clone)]
pub struct FunctionRef {
	func: Arc<Function>
}

impl Deref for FunctionRef {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        self.func.deref()
    }
}

impl FunctionRef {
	pub fn take_typ(&self) -> Type {
		unsafe { &*self.func.inner.as_ptr() }.typ()
	}
}

impl Function {
	pub fn borrow(&self) -> Ref<FunctionInner> {
		self.inner.borrow()
	}

	pub fn borrow_mut(&self) -> RefMut<FunctionInner> {
		self.inner.borrow_mut()
	}
}

impl Debug for FunctionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let func = self.borrow();

		let params = func.params.iter()
			.map(|param| format!("{param:?}"))
			.collect::<Vec<_>>()
			.join(", ");

        write!(f, "{} func {}({}): {:?} {:?}", func.visibility, func.name, params, func.return_type, func.code)
    }
}