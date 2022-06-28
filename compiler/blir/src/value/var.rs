use std::{cell::{Ref, RefCell, RefMut},
          fmt::Debug,
          ops::Deref,
          sync::Arc};

use errors::Span;

use super::Type;
use crate::{attributes::Attributes, value::Value, Visibility};

pub struct VarInner {
    pub attributes: Attributes,

    pub visibility: Visibility,

    pub name: String,

    pub typ: Type,

    pub default_value: Option<Value>,

    pub is_constant: bool,

    pub span: Span,

    pub meta: String
}

pub struct Var {
    var: RefCell<VarInner>,
}

impl Var {
    pub fn new(attributes: Attributes, visibility: Visibility, name: String, typ: Type, default_value: Option<Value>, is_constant: bool, span: Span, meta: String) -> VarRef {
        let var_inner = VarInner { attributes,
                                   visibility,
                                   name,
                                   typ,
                                   default_value,
                                   is_constant,
                                   span,
                                   meta };

        VarRef { var: Arc::new(Var { var: RefCell::new(var_inner), }), }
    }

    pub fn borrow(&self) -> Ref<VarInner> { self.var.borrow() }

    pub fn borrow_mut(&self) -> RefMut<VarInner> { self.var.borrow_mut() }
}

#[derive(Clone)]
pub struct VarRef {
    var: Arc<Var>,
}

impl Deref for VarRef {
    type Target = Var;

    fn deref(&self) -> &Self::Target { self.var.deref() }
}

impl Debug for VarRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = &self.borrow().default_value {
            write!(f,
                   "{visibility} var {name}: {typ:?} = {value:?}",
                   visibility = self.borrow().visibility,
                   name = self.borrow().name,
                   typ = self.borrow().typ,
                   value = val)
        } else {
            write!(f,
                   "{visibility} var {name}: {typ:?}",
                   visibility = self.borrow().visibility,
                   name = self.borrow().name,
                   typ = self.borrow().typ)
        }
    }
}
