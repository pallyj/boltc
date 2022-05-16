use blir::{scope::ScopeRef,
           typ::{Struct, StructRef, Type, TypeKind},
           value::{Constant, ConstantRef, Var, VarRef}};
use mangle::Path;
use parser::ast::{containers::{StructDef, StructItem},
                  var::{LetDef, VariableDef}};

use crate::AstLowerer;

impl AstLowerer {
    pub fn lower_struct_static_let(&self, var: LetDef) -> ConstantRef {
        let visibility = self.lower_visibility(var.visibility());
        let name = var.label();
        let typ = var.typ()
                     .map(|typ| self.lower_type(typ))
                     .unwrap_or_else(Type::infer);
        let Some(expr) = var.value() else {
			// Error
			panic!();
		};
        let value = self.lower_expr(expr);

        let attributes = self.lower_attributes(var.attributes());
        let span = self.span(var.range());

        Constant::new(attributes, visibility, name, typ, value, span)
    }

    pub fn lower_struct_let(&self, var: LetDef) -> VarRef {
        let visibility = self.lower_visibility(var.visibility());
        let name = var.label();
        let typ = var.typ()
                     .map(|typ| self.lower_type(typ))
                     .unwrap_or_else(Type::infer);
        let default_value = var.value().map(|value| self.lower_expr(value));

        let attributes = self.lower_attributes(var.attributes());
        let span = self.span(var.range());

        Var::new(attributes, visibility, name, typ, default_value, true, span)
    }

    pub fn lower_struct_var(&self, var: VariableDef) -> VarRef {
        let visibility = self.lower_visibility(var.visibility());
        let name = var.label();
        let typ = var.typ()
                     .map(|typ| self.lower_type(typ))
                     .unwrap_or_else(Type::infer);
        let default_value = var.value().map(|value| self.lower_expr(value));

        let attributes = self.lower_attributes(var.attributes());
        let span = self.span(var.range());

        Var::new(attributes,
                 visibility,
                 name,
                 typ,
                 default_value,
                 false,
                 span)
    }

    pub fn lower_struct(&self, def: StructDef, parent: &ScopeRef, parent_mangle: &Path) -> StructRef {
        let visibility = self.lower_visibility(def.visibility());
        let name = def.name();

        let attributes = self.lower_attributes(def.attributes());

        let r#struct = Struct::new(attributes, visibility, name, parent, parent_mangle.clone());
        let scope = r#struct.borrow().scope().clone();

        let self_ty = TypeKind::Struct(r#struct.clone()).anon();

        let struct_mangled = r#struct.borrow().path().clone();

        for struct_item in def.body().items().into_iter() {
            match struct_item {
                StructItem::FuncDef(func_def) => {
                    let lowered_method = self.lower_method(func_def, self_ty.clone(), &scope, &struct_mangled);

                    r#struct.add_method(lowered_method);
                }

                StructItem::StructDef(struct_def) => {
                    let lowered_struct = self.lower_struct(struct_def, &scope, &struct_mangled);

                    r#struct.add_substruct(lowered_struct);
                }

                StructItem::LetDef(let_def) => {
                    if let_def.is_static() {
                        r#struct.add_constant(self.lower_struct_static_let(let_def));
                    } else {
                        r#struct.add_var(self.lower_struct_let(let_def));
                    }
                }

                StructItem::VariableDef(var_def) => {
                    let var = self.lower_struct_var(var_def);

                    r#struct.add_var(var);
                }

                StructItem::TypeAlias(type_alias) => {
                    let visibility = self.lower_visibility(type_alias.visibility());
                    let name = type_alias.name();
                    let aliased = self.lower_type(type_alias.aliased_type());

                    r#struct.add_type(name, visibility, aliased.kind);
                }

                StructItem::NoOp(_) => {}

                StructItem::Error => panic!(),
            }
        }

        r#struct
    }
}
