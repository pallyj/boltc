use std::{cell::{Ref, RefCell, RefMut},
          fmt::Debug,
          ops::Deref,
          sync::Arc};

use errors::Span;

use super::{FuncParam, FunctionInfo};
use crate::{attributes::Attributes,
            scope::ScopeRef,
            typ::{Type, TypeKind},
            Visibility};

#[derive(Clone)]
pub struct ExternFunctionInner {
    pub attributes: Attributes,
    pub visibility: Visibility,
    pub info:       FunctionInfo,
    pub span:       Span,
    pub parent:     ScopeRef,
}

impl ExternFunctionInner {
    pub fn typ(&self) -> Type {
        let params = self.info
                         .params()
                         .iter()
                         .map(|param| param.typ.clone())
                         .collect::<Vec<_>>();

        TypeKind::Function { return_type: Box::new(self.info.return_type().clone()),
                             params,
                             labels: vec![] }.anon()
    }
}

pub struct ExternFunction {
    inner: RefCell<ExternFunctionInner>,
}

impl ExternFunction {
    pub fn new(attributes: Attributes, visibility: Visibility, name: String, params: Vec<FuncParam>, return_type: Type, span: Span, parent: &ScopeRef) -> ExternFunctionRef {
        let func = ExternFunctionInner { attributes,
                                         visibility,
                                         info: FunctionInfo::new(name, params, return_type, false),
                                         span,
                                         parent: parent.clone() };

        ExternFunctionRef { func: Arc::new(ExternFunction { inner: RefCell::new(func), }), }
    }
}

#[derive(Clone)]
pub struct ExternFunctionRef {
    func: Arc<ExternFunction>,
}

impl Deref for ExternFunctionRef {
    type Target = ExternFunction;

    fn deref(&self) -> &Self::Target { self.func.deref() }
}

impl ExternFunctionRef {
    pub fn take_typ(&self) -> Type { unsafe { &*self.func.inner.as_ptr() }.typ() }
}

impl ExternFunction {
    pub fn borrow(&self) -> Ref<ExternFunctionInner> { self.inner.borrow() }

    pub fn borrow_mut(&self) -> RefMut<ExternFunctionInner> { self.inner.borrow_mut() }
}

impl Debug for ExternFunctionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let func = self.borrow();

        let params = func.info
                         .params()
                         .iter()
                         .map(|param| format!("{param:?}"))
                         .collect::<Vec<_>>()
                         .join(", ");

        write!(f,
               "{} func {}({}): {:?}",
               func.visibility,
               func.info.name(),
               params,
               func.info.return_type())
    }
}
