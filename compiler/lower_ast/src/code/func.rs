use blir::{code::{ExternFunction, ExternFunctionRef, FuncParam, Function, FunctionRef, Method, MethodRef},
           scope::ScopeRef,
           typ::{Type, TypeKind},
           Visibility};
use mangle::Path;
use parser::{ast::func::FuncDef, lexer::SyntaxKind};

use crate::AstLowerer;

impl<'a, 'b> AstLowerer<'a, 'b> {
    pub fn lower_func(&mut self, func: FuncDef, parent: &ScopeRef, parent_mangled: &Path) -> FunctionRef {
        let range = func.range();
        let span = self.span(range);

        let visibility = self.lower_visibility(func.visibility());
        let name = func.name();
        let params = func.parameters()
                         .iter()
                         .map(|param| {
                             let (label, bind_name) = if let Some(bind_name) = param.second_label() {
                                 (Some(param.first_label()), bind_name)
                             } else {
                                 (None, param.first_label())
                             };

                             FuncParam { label,
                                         bind_name,
                                         typ: self.lower_type(param.typ()),
                                         is_shared: false }
                         })
                         .collect();
        let return_type = func.return_type()
                              .map(|rt| self.lower_type(rt))
                              .unwrap_or_else(|| TypeKind::Void.anon());
        let code = self.lower_code_block(func.code().unwrap(), None);

        let attributes = self.lower_attributes(func.attributes());

        Function::new(attributes,
                      visibility,
                      name,
                      params,
                      return_type,
                      code,
                      span,
                      parent,
                      parent_mangled.clone())
    }

    pub fn lower_extern_func(&self, func: FuncDef, parent: &ScopeRef) -> ExternFunctionRef {
        let range = func.range();
        let span = self.span(range);

        let visibility = self.lower_visibility(func.visibility());
        let name = func.name();
        let params = func.parameters()
                         .iter()
                         .map(|param| {
                             let (label, bind_name) = if let Some(bind_name) = param.second_label() {
                                 (Some(param.first_label()), bind_name)
                             } else {
                                 (None, param.first_label())
                             };

                             FuncParam { label,
                                         bind_name,
                                         typ: self.lower_type(param.typ()),
                                         is_shared: false }
                         })
                         .collect();
        let return_type = func.return_type()
                              .map(|rt| self.lower_type(rt))
                              .unwrap_or_else(|| TypeKind::Void.anon());

        let attributes = self.lower_attributes(func.attributes());

        ExternFunction::new(attributes,
                            visibility,
                            name,
                            params,
                            return_type,
                            span,
                            parent)
    }

    pub fn lower_method(&mut self, func: FuncDef, reciever: Type, parent: &ScopeRef, parent_mangled: &Path) -> MethodRef {
        let range = func.range();
        let span = self.span(range);

        let visibility = self.lower_visibility(func.visibility());
        let is_static = func.is_static() || func.is_operator() || func.is_init(); // todo: is_init and is_operator are NOT static
        let is_operator = func.is_operator();
        let is_mutating = func.is_mutating();
        let is_init = func.is_init();
        let name = if is_init {
            "init".to_string()
        } else {
            func.name()
        };
        let params = func.parameters()
                         .iter()
                         .map(|param| {
                             let (label, bind_name) = if let Some(bind_name) = param.second_label() {
                                 (Some(param.first_label()), bind_name)
                             } else {
                                 (None, param.first_label())
                             };

                             FuncParam { label,
                                         bind_name,
                                         typ: self.lower_type(param.typ()),
                                         is_shared: false }
                         })
                         .collect();
        let return_type = if is_init {
            reciever.clone()
        } else {
            func.return_type()
                .map(|rt| self.lower_type(rt))
                .unwrap_or_else(|| TypeKind::Void.anon())
        };
        let code = self.lower_code_block(func.code().unwrap(), None);

        let attributes = self.lower_attributes(func.attributes());

        Method::new(attributes,
                    reciever,
                    is_static,
                    is_operator,
                    is_mutating, // todo: check if it mutates
                    visibility,
                    name,
                    params,
                    return_type,
                    code,
                    span,
                    parent,
                    parent_mangled)
    }

    pub fn lower_visibility(&self, visibility: Option<SyntaxKind>) -> Visibility {
        match visibility {
            Some(SyntaxKind::PublicKw) => Visibility::Public,
            Some(SyntaxKind::InternalKw) => Visibility::Internal,
            Some(SyntaxKind::FilePrivateKw) => Visibility::Fileprivate,
            Some(SyntaxKind::PrivateKw) => Visibility::Private,

            None => Visibility::Internal,

            Some(other) => panic!("{other:?}"),
        }
    }
}
