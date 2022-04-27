use blir::{scope::ScopeRef, typ::{Enum, EnumRef, Case, CaseRef}};
use mangle::Path;
use parser::ast::containers::{EnumDef, EnumItem, CaseDef};

use crate::AstLowerer;

impl AstLowerer {
	pub fn lower_enum(
		&self,
		def: EnumDef,
		parent: &ScopeRef,
		parent_path: &Path) -> EnumRef
	{
		let attributes = self.lower_attributes(def.attributes());
		let visibility = self.lower_visibility(def.visibility());
		let name = def.name();

		let enum_def = Enum::new(attributes, visibility, name, parent, parent_path);
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

				EnumItem::Error => panic!(),

				_ => panic!(),
			}
		}

		enum_def
	}

	pub fn lower_cases(
		&self,
		cases: CaseDef) -> Vec<CaseRef>
	{
		cases.items()
			 .map(|item| Case::new(item.name()))
			 .collect()
	}
}