use blir::scope::ScopeRef;
use blir::typ::{TypeKind, Type};
use parser::ast::containers::{StructDef, StructItem};
use parser::ast::var::{VariableDef};
use blir::{typ::{Struct, StructRef}, value::{VarRef, Var}};

use crate::AstLowerer;

impl AstLowerer {
	pub fn lower_struct_var(&self, var: VariableDef) -> VarRef {
		let visibility = self.lower_visibility(var.visibility());
		let name = var.label();
		let typ = var.typ()
			.map(|typ| self.lower_type(typ))
			.unwrap_or(Type::infer());
		let default_value = var.value()
			.map(|value| self.lower_expr(value));

		Var::new(visibility, name, typ, default_value)
	}

	pub fn lower_struct(&self, def: StructDef, parent: &ScopeRef) -> StructRef {
		let visibility = self.lower_visibility(def.visibility());
		let name = def.name();

		let r#struct = Struct::new(visibility, name, parent);
		let scope = r#struct.borrow().scope().clone();

		let self_ty = TypeKind::Struct(r#struct.clone()).anon();

		for struct_item in def.body()
			.items()
			.into_iter()
		{
			match struct_item {
				StructItem::FuncDef(func_def) => {
					let lowered_method = self.lower_method(func_def, self_ty.clone(), &scope);

					r#struct.add_method(lowered_method);
				}

				StructItem::StructDef(struct_def) => {
					let lowered_struct = self.lower_struct(struct_def, &scope);

					r#struct.add_substruct(lowered_struct);
				}

				StructItem::LetDef(_let_def) => {

				}

				StructItem::VariableDef(var_def) => {
					let var = self.lower_struct_var(var_def);

					r#struct.add_var(var);
				}

				StructItem::Error => panic!(),
			}
		}

		r#struct
	}
}