use std::{cell::{Ref, RefCell, RefMut},
          fmt::Debug,
          ops::Deref,
          sync::Arc};

use errors::Span;
use mangle::{Path, MangledFunction};

use super::{CodeBlock, FuncParam, FunctionInfo};
use crate::{attributes::Attributes,
            scope::{ScopeRef, ScopeRelation, ScopeType},
            typ::{Type, TypeKind},
            value::ValueKind,
            Symbol, Visibility};

pub struct Method {
    inner: RefCell<MethodInner>,
}

#[allow(dead_code)]
pub struct MethodInner {
    pub attributes: Attributes,
    pub visibility: Visibility,
    pub is_static:  bool,
    pub is_operator: bool,
    pub info:       FunctionInfo,
    pub code:       CodeBlock,
    pub span:       Span,
    scope:          ScopeRef,
    self_type:      Type,
    path:         Path,
}

impl MethodInner {
    pub fn scope(&self) -> &ScopeRef { &self.scope }

    pub fn add_params(&mut self) {
        let sym = Symbol::Value(ValueKind::SelfVal.anon(self.self_type.clone()));
        if !self.is_static {
            self.scope
                .add_symbol("self".to_string(), Visibility::Local, sym);
            self.scope.define_scope_type("self", self.self_type.clone());
        }

        if self.is_operator {
            let self_param = FuncParam {
                label: None,
                bind_name: "self".to_string(),
                typ: self.self_type.clone(),
            };

            self.info.params_mut()
                .insert(0, self_param)
        }

        self.scope
            .define_scope_type("return", self.info.return_type().clone());

        for p in self.info.params().iter() {
            let val = ValueKind::FunctionParam(p.bind_name.clone()).anon(p.typ.clone());

            self.scope
                .add_symbol(p.bind_name.clone(), Visibility::Local, Symbol::Value(val));
        }
    }

    pub fn typ(&self) -> Type {
        if self.is_static {
            let params = self.info
                             .params()
                             .iter()
                             .map(|param| param.typ.clone())
                             .collect::<Vec<_>>();

            TypeKind::Function { return_type: Box::new(self.info.return_type().clone()),
                                 params,
                                 labels: vec![] }.anon()
        } else {
            let params = self.info
                             .params()
                             .iter()
                             .map(|param| param.typ.clone())
                             .collect::<Vec<_>>();

            TypeKind::Method { reciever: Box::new(self.self_type.clone()),
                               return_type: Box::new(self.info.return_type().clone()),
                               params }.anon()
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn mangle(&self) -> String {
        let (args, labels) = self.info.params()
            .iter()
            .map(|param| (param.typ.mangle(), param.label.as_ref().map(|param| param.as_str())))
            .unzip();

        MangledFunction {
            path: &self.path,
            args,
            labels,
        }.to_string()
    }
}

impl Method {
    pub fn new(attributes: Attributes,
               self_type: Type,
               is_static: bool,
               is_operator: bool,
               visibility: Visibility,
               name: String,
               params: Vec<FuncParam>,
               return_type: Type,
               code: CodeBlock,
               span: Span,
               parent: &ScopeRef,
               parent_path: &Path)
               -> MethodRef {
        let func = MethodInner { attributes,
                                 visibility,
                                 is_static,
                                 is_operator,
                                 path: parent_path.clone().append(&name),
                                 info: FunctionInfo::new(name, params, return_type, true),
                                 code,
                                 span,
                                 scope: ScopeRef::new(Some(parent),
                                                      ScopeRelation::SameContainer,
                                                      ScopeType::Code,
                                                      !is_static,
                                                      true),
                                 self_type, };

        MethodRef { func: Arc::new(Method { inner: RefCell::new(func), }), }
    }

    pub fn is_static(&self) -> bool { self.inner.borrow().is_static }

    pub fn is_operator(&self) -> bool { self.inner.borrow().is_operator }

    pub fn name(&self) -> String { self.inner.borrow().info.name().clone() }

    pub fn visibility(&self) -> Visibility { self.inner.borrow().visibility }

    pub fn borrow(&self) -> Ref<MethodInner> { self.inner.borrow() }

    pub fn borrow_mut(&self) -> RefMut<MethodInner> { self.inner.borrow_mut() }
}

#[derive(Clone)]
pub struct MethodRef {
    func: Arc<Method>,
}

impl MethodRef {
    pub fn take_typ(&self) -> Type { unsafe { &*self.func.inner.as_ptr() }.typ() }

    pub fn info(&self) -> &FunctionInfo { &unsafe { &*self.func.inner.as_ptr() }.info }
}

impl Deref for MethodRef {
    type Target = Method;

    fn deref(&self) -> &Self::Target { self.func.deref() }
}

impl Debug for MethodRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = self.borrow();

        let params = method.info
                           .params()
                           .iter()
                           .map(|param| format!("{param:?}"))
                           .collect::<Vec<_>>()
                           .join(", ");

        write!(f,
               "{} func {}({}): {:?} {:?}",
               method.visibility,
               method.info.name(),
               params,
               method.info.return_type(),
               method.code)
    }
}
