use std::{cell::{Ref, RefCell, RefMut},
          fmt::Debug,
          ops::Deref,
          sync::Arc};

use errors::Span;
use itertools::Itertools;
use mangle::{Path, MangledGlobal};

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


struct GlobalVarInner {
	meta: 		String,
	attributes: Attributes,
	visibility: Visibility,
    is_const:   bool,
	name: 		String,
	ty:         Type,
    def_value:  Value,

    span:       Span,
    path:       Path,
    mangle:     String,
}

pub struct GlobalVar {
    inner: RefCell<GlobalVarInner>,
}

impl GlobalVar {
    ///
    /// Gets a copy of the metadata for this variable
    /// 
    pub fn meta(&self) -> String {
        self.inner.borrow().meta.clone()
    }

    ///
    /// Gets a list of the variable's attributes
    /// 
    pub fn attributes(&self) -> Ref<Attributes> {
        Ref::map(self.inner.borrow(), |inner| &inner.attributes)
    }

    ///
    /// Gets the visibility of the variable
    /// 
    pub fn visibility(&self) -> Visibility {
        self.inner.borrow().visibility
    }

    ///
    /// Whether the variable is constant
    /// 
    /// If it is declared with the `let` keyword, it is constant
    /// and if it is declared with the `var` keyword, it is a variable.
    /// 
    /// In the future, the syntax might switch to
    /// 
    /// ```
    /// static number: Int = 0
    /// static var controller = Controller()
    /// ```
    /// 
    pub fn is_const(&self) -> bool {
        self.inner.borrow().is_const
    }

    ///
    /// The variable's name
    /// 
    pub fn name(&self) -> String {
        self.inner.borrow().name.clone()
    }

    ///
    /// The variable's symbol
    /// 
    pub fn symbol(&self) -> Ref<String> {
        Ref::map(self.inner.borrow(), |inner| &inner.mangle)
    }

    ///
    /// The type of the variable. This can be automatically inferred.
    /// 
    pub fn ty(&self) -> Ref<Type> {
        Ref::map(self.inner.borrow(), |inner| &inner.ty)
    }

    ///
    /// The type of the variable. This can be automatically inferred.
    /// 
    pub fn ty_mut(&self) -> RefMut<Type> {
        RefMut::map(self.inner.borrow_mut(), |inner| &mut inner.ty)
    }

    ///
    /// The default value of the variable. This will be assigned to it at startup.
    /// 
    pub fn default_value(&self) -> Ref<Value> {
        Ref::map(self.inner.borrow(), |inner| &inner.def_value)
    }

    ///
    /// The default value of the variable. This will be assigned to it at startup.
    /// 
    pub fn default_value_mut(&self) -> RefMut<Value> {
        RefMut::map(self.inner.borrow_mut(), |inner| &mut inner.def_value)
    }

    ///
    /// The default value and type. Used to prevent borrow mut errors
    /// 
    pub fn value_and_ty_mut(&self) -> (RefMut<Value>, RefMut<Type>) {
        RefMut::map_split(self.inner.borrow_mut(), |inner| (&mut inner.def_value, &mut inner.ty))
    }

    ///
    /// The span where the variable was declared
    /// 
    pub fn span(&self) -> Span {
        self.inner.borrow().span
    }

    ///
    /// The path of the variable
    /// 
    pub fn path(&self) -> Ref<Path> {
        Ref::map(self.inner.borrow(), |inner| &inner.path)
    }
}

#[derive(Clone)]
pub struct GlobalVarRef {
    inner: Arc<GlobalVar>
}

impl GlobalVarRef {
    pub fn new(
        meta:       String,
        attributes: Attributes,
        visibility: Visibility,
        is_const:   bool,
        name:       String,
        ty:         Type,
        def_value:  Value,
        span:       Span,
        parent:     &Path) -> Self
    {
        let path = parent.clone().append(&name);
        
        let inner = GlobalVarInner {
            mangle: MangledGlobal(&path).to_string(),
            path,
            meta,
            attributes,
            visibility,
            is_const,
            name,
            ty,
            def_value,
            span,
        };

        Self {
            inner: Arc::new(GlobalVar {
                inner: RefCell::new(inner)
            })
        }
    }
}

impl Deref for GlobalVarRef {
    type Target = GlobalVar;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Debug for GlobalVarRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            r#"{attributes:?}
{visibility} {constant}{name}: {ty:?} = {val:?}"#,
            
            attributes = self.attributes().iter().format(", "),
            visibility = self.visibility(),
            constant = self.is_const().then_some("let ").unwrap_or("var "),
            name = self.name(),
            ty = self.ty(),
            val = self.default_value())
    }
}