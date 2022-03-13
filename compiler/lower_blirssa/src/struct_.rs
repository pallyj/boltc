use blirssa::typ::StructRef;

use crate::{ModuleContext, typ::lower_basic_typ};

pub fn lower_struct<'a, 'ctx>(r#struct: &StructRef, context: &ModuleContext<'a, 'ctx>) {
	if let Some(struct_ty) = context.module.get_struct_type(r#struct.name()) {
		let field_types = r#struct.fields()
			.iter()
			.filter_map(|field| lower_basic_typ(field.typ_ref(), context))
			.collect::<Vec<_>>();
			
		struct_ty.set_body(&field_types, r#struct.is_packed());
	}
}