use blirssa::typ::{EnumRef, Type};

use crate::{typ::lower_basic_typ, ModuleContext};

pub fn lower_enum<'a, 'ctx>(r#enum: &EnumRef, context: &ModuleContext<'a, 'ctx>) {
    let Some(enum_ty) = context.module.get_struct_type(r#enum.name()) else {
        return
    };

    let enum_tag_repr = Type::Integer { bits: 64 }; // TODO: It can be anything

    let field_types = vec![
        lower_basic_typ(&enum_tag_repr, context).unwrap()
    ];

    enum_ty.set_body(&field_types, false);
}
