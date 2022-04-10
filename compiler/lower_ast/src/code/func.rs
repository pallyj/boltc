use blir::{code::{ExternFunction, ExternFunctionRef, FuncParam, Function, FunctionRef, Method, MethodRef},
           scope::ScopeRef,
           typ::{Type, TypeKind},
           Visibility};
use mangle::{Path};
use parser::{ast::func::FuncDef, lexer::SyntaxKind};

use crate::AstLowerer;

impl AstLowerer {
    pub fn lower_func(&self, func: FuncDef, parent: &ScopeRef, parent_mangled: &Path) -> FunctionRef {
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
                                         typ:       self.lower_type(param.typ()), }
                         })
                         .collect();
        let return_type = func.return_type()
                              .map(|rt| self.lower_type(rt))
                              .unwrap_or_else(|| TypeKind::Void.anon());
        let code = self.lower_code_block(func.code().unwrap());

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
                                        typ:       self.lower_type(param.typ()), }
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

    pub fn lower_method(&self, func: FuncDef, reciever: Type, parent: &ScopeRef, parent_mangled: &Path) -> MethodRef {
        let range = func.range();
        let span = self.span(range);

        let visibility = self.lower_visibility(func.visibility());
        let is_static = func.is_static();
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
                                        typ:       self.lower_type(param.typ()), }
                         })
                         .collect();
        let return_type = func.return_type()
                              .map(|rt| self.lower_type(rt))
                              .unwrap_or_else(|| TypeKind::Void.anon());
        let code = self.lower_code_block(func.code().unwrap());

        let attributes = self.lower_attributes(func.attributes());

        Method::new(attributes,
                    reciever,
                    is_static,
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

            _ => panic!(),
        }
    }
}
