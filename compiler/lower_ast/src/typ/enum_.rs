use blir::{scope::ScopeRef, typ::{Enum, EnumRef, Case, CaseRef, TypeKind}};
use mangle::Path;
use parser::ast::containers::{EnumDef, EnumItem, CaseDef};

use crate::AstLowerer;

impl<'a, 'b> AstLowerer<'a, 'b> {
	pub fn lower_enum(
		&mut self,
		def: EnumDef,
		parent: &ScopeRef,
		parent_path: &Path) -> EnumRef
	{
		let attributes = self.lower_attributes(def.attributes());
		let visibility = self.lower_visibility(def.visibility());
		let name = def.name();

		let repr_type = def
			.repr_type()
			.map(|ty| self.lower_type(ty))
			.unwrap_or(TypeKind::Integer { bits: 32 }.anon());

		let enum_def = Enum::new(attributes, visibility, name, parent, parent_path, repr_type);
		let enum_scope = enum_def.scope();

		let enum_type = enum_def.get_type().anon();
		let enum_path = enum_def.path();

		for enum_item in def.body().items() {
			match enum_item {
				EnumItem::FuncDef(func_def) => {
                    let lowered_method = self.lower_method(
						func_def, 
						enum_type.clone(),
						&enum_scope,
						&enum_path);

                    enum_def.add_method(lowered_method);
                }

				EnumItem::CaseDef(case_def) => {
					let cases = self.lower_cases(
						case_def);

					enum_def.add_cases(cases);
				}

				EnumItem::StructDef(struct_def) => {
                    let lowered_struct = self.lower_struct(struct_def, &parent, &enum_path);

                    enum_def.add_substruct(lowered_struct);
                }

				EnumItem::EnumDef(subenum) => {
                    let lowered_enum = self.lower_enum(subenum, &parent, &enum_path);

                    enum_def.add_subenum(lowered_enum);
                }

				EnumItem::TypeAlias(type_alias) => {
                    let visibility = self.lower_visibility(type_alias.visibility());
                    let name = type_alias.name();
                    let aliased = self.lower_type(type_alias.aliased_type());

                    r#enum_def.add_type(name, visibility, aliased.kind);
                }

				EnumItem::Error => panic!(),
			}
		}

		enum_def
	}

	pub fn lower_cases(
		&self,
		cases: CaseDef) -> Vec<CaseRef>
	{
		cases.items()
			 .map(|item| {
				      let associated_types = if let Some(associated_types) = item.associated_types() {
					      associated_types
						      .map(|typ| self.lower_type(typ))
						      .collect()
					  } else {
						  vec![]
					  };
					      
				      Case::new(item.name(), associated_types)
				  })
			 .collect()
	}
}