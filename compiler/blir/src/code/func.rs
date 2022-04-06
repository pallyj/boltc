use std::{cell::{Ref, RefCell, RefMut},
          fmt::Debug,
          ops::Deref,
          sync::Arc};

use errors::Span;
use mangle::{MangleComponent, Mangled};

use super::CodeBlock;
use crate::{attributes::Attributes,
            scope::{ScopeRef, ScopeRelation, ScopeType},
            typ::{Type, TypeKind},
            value::ValueKind,
            Symbol, Visibility};

#[derive(Clone)]
pub struct FunctionInner {
    pub attributes: Attributes,
    pub visibility: Visibility,
    pub info:       FunctionInfo,
    pub code:       CodeBlock,
    pub span:       Span,
    scope:          ScopeRef,
    parent_mangled: Mangled,
}

impl FunctionInner {
    pub fn add_params(&self) {
        self.scope
            .define_scope_type("return", self.info.return_type.clone());

        for p in self.info.params.iter() {
            let val = ValueKind::FunctionParam(p.bind_name.clone()).anon(p.typ.clone());

            self.scope
                .add_symbol(p.bind_name.clone(), Visibility::Public, Symbol::Value(val));
        }
    }

    pub fn typ(&self) -> Type {
        let params = self.info
                         .params
                         .iter()
                         .map(|param| param.typ.clone())
                         .collect::<Vec<_>>();

        TypeKind::Function { return_type: Box::new(self.info.return_type.clone()),
                             params,
                             labels: vec![] }.anon()
    }

    pub fn scope(&self) -> &ScopeRef { &self.scope }

    pub fn mangled(&self) -> Mangled {
        self.parent_mangled
            .clone()
            .append(MangleComponent::Function(self.info.name.clone()))
    }
}

pub struct Function {
    inner: RefCell<FunctionInner>,
}

#[derive(Clone)]
pub struct FuncParam {
    pub label:     Option<String>,
    pub bind_name: String,
    pub typ:       Type,
}

impl Debug for FuncParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.label {
            write!(f,
                   "{label} {bind_name}: {ty:?}",
                   bind_name = self.bind_name,
                   ty = self.typ)
        } else {
            write!(f,
                   "{bind_name}: {ty:?}",
                   bind_name = self.bind_name,
                   ty = self.typ)
        }
    }
}

impl Function {
    pub fn new(attributes: Attributes,
               visibility: Visibility,
               name: String,
               params: Vec<FuncParam>,
               return_type: Type,
               code: CodeBlock,
               span: Span,
               parent: &ScopeRef,
               parent_mangled: Mangled)
               -> FunctionRef {
        let func = FunctionInner { attributes,
                                   visibility,
                                   info: FunctionInfo::new(name, params, return_type, false),
                                   code,
                                   span,
                                   scope: ScopeRef::new(Some(parent),
                                                        ScopeRelation::SameFile,
                                                        ScopeType::Code,
                                                        false,
                                                        true),
                                   parent_mangled };

        FunctionRef { func: Arc::new(Function { inner: RefCell::new(func), }), }
    }
}

#[derive(Clone)]
pub struct FunctionRef {
    func: Arc<Function>,
}

impl Deref for FunctionRef {
    type Target = Function;

    fn deref(&self) -> &Self::Target { self.func.deref() }
}

impl FunctionRef {
    pub fn take_typ(&self) -> Type { unsafe { &*self.func.inner.as_ptr() }.typ() }

    pub fn take_name(&self) -> &str { unsafe { &*self.func.inner.as_ptr() }.info.name() }
}

impl Function {
    pub fn borrow(&self) -> Ref<FunctionInner> { self.inner.borrow() }

    pub fn borrow_mut(&self) -> RefMut<FunctionInner> { self.inner.borrow_mut() }
}

impl Debug for FunctionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let func = self.borrow();

        let params = func.info
                         .params
                         .iter()
                         .map(|param| format!("{param:?}"))
                         .collect::<Vec<_>>()
                         .join(", ");

        write!(f,
               "{} func {}({}): {:?} {:?}",
               func.visibility, func.info.name, params, func.info.return_type, func.code)
    }
}

#[derive(Clone)]
pub struct FunctionInfo {
    name:      String,
    link_name: String,

    params:      Vec<FuncParam>,
    return_type: Type,

    always_link: bool,
    is_method:   bool,
}

impl FunctionInfo {
    pub fn new(name: String, params: Vec<FuncParam>, return_type: Type, is_method: bool) -> Self {
        Self { link_name: name.clone(),
               name,

               params,
               return_type,

               always_link: false,
               is_method }
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn link_name(&self) -> &String { &self.link_name }

    pub fn set_link_name(&mut self, name: String) { self.link_name = name; }

    pub fn params(&self) -> &Vec<FuncParam> { &self.params }

    pub fn params_mut(&mut self) -> &mut Vec<FuncParam> { &mut self.params }

    pub fn return_type(&self) -> &Type { &self.return_type }

    pub fn return_type_mut(&mut self) -> &mut Type { &mut self.return_type }

    pub fn always_links(&self) -> bool { self.always_link }

    pub fn always_link(&mut self) { self.always_link = true; }

    pub fn is_method(&self) -> bool { self.is_method }
}
