use blirssa::typ::{EnumRef};
use inkwell::{types::{BasicType}};

use crate::{typ::lower_basic_typ, ModuleContext};

pub fn lower_enum<'a, 'ctx>(r#enum: &EnumRef, context: &ModuleContext<'a, 'ctx>) {
    let Some(enum_ty) = context.module.get_struct_type(r#enum.name()) else {
        return
    };

    let enum_tag_repr = r#enum.tag();

    // TODO: This is HORRIBLY unoptimized
    let exe = context.module.create_execution_engine().unwrap();
    let layout = exe.get_target_data();

    let mut n_bits = 0;

    for variant in r#enum.variants().values() {
        let tuple = lower_basic_typ(variant.tuple_type(), context).unwrap();

        let size_of_tuple = layout.get_abi_size(&tuple);

        if size_of_tuple > n_bits {
            n_bits = size_of_tuple;
        }
    }
    exe.remove_module(context.module).unwrap();

    let field_types = vec![
        lower_basic_typ(&enum_tag_repr, context).unwrap(),
        context.context
                .i8_type()
                .array_type(n_bits as u32)
                .as_basic_type_enum()
    ];

    enum_ty.set_body(&field_types, false);
}
