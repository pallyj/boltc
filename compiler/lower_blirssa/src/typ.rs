use blirssa::typ::{StructRef, Type, EnumRef};
use inkwell::{types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FloatType, FunctionType, IntType, PointerType, StructType},
              AddressSpace};

use crate::ModuleContext;

pub fn lower_basic_typ<'a, 'ctx>(t: &Type, context: &ModuleContext<'a, 'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    Some(match t {
        Type::Integer { bits } => lower_integer_type(*bits, context)?.as_basic_type_enum(),

        Type::Float { bits } => lower_float_type(*bits, context)?.as_basic_type_enum(),

        Type::StrSlice => lower_strslice_type(context).as_basic_type_enum(),

        Type::Function { return_type, pars } => lower_function_type(&*return_type, pars, context)?.ptr_type(AddressSpace::Global)
                                                                                                  .as_basic_type_enum(),

        Type::Tuple(tuple_items) => lower_tuple_type(tuple_items, context)?.as_basic_type_enum(),

        Type::Pointer { pointee } => lower_pointer_typ(&*pointee, context)?.as_basic_type_enum(),

        Type::Struct { container } => lower_struct_typ(container, context)?.as_basic_type_enum(),

        Type::Enum(container) => lower_enum_typ(container, context)?.as_basic_type_enum(),

        Type::Void => return None,
    })
}

pub fn lower_struct_typ<'a, 'ctx>(container: &StructRef, context: &ModuleContext<'a, 'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    if let Some(transparent) = container.transparent_type() {
        lower_basic_typ(&transparent, context)
    } else {
        Some(context.module
                    .get_struct_type(container.name())?
                    .as_basic_type_enum())
    }
}

pub fn lower_enum_typ<'a, 'ctx>(container: &EnumRef, context: &ModuleContext<'a, 'ctx>) -> Option<BasicTypeEnum<'ctx>> {
    Some(context.module
                .get_struct_type(container.name())?
                .as_basic_type_enum())
}

pub fn lower_pointer_typ<'a, 'ctx>(t: &Type, context: &ModuleContext<'a, 'ctx>) -> Option<PointerType<'ctx>> {
    Some(match t {
        Type::Void => context.context.i8_type().ptr_type(AddressSpace::Global),

        Type::Integer { bits } => lower_integer_type(*bits, context)?.ptr_type(AddressSpace::Global),

        Type::Float { bits } => lower_float_type(*bits, context)?.ptr_type(AddressSpace::Global),

        Type::StrSlice => lower_strslice_type(context).ptr_type(AddressSpace::Global),

        Type::Function { return_type, pars } => lower_function_type(&*return_type, pars, context)?.ptr_type(AddressSpace::Global),

        Type::Tuple(tuple_items) => lower_tuple_type(tuple_items, context)?.ptr_type(AddressSpace::Global),

        Type::Struct { container } => lower_struct_typ(container, context)?.ptr_type(AddressSpace::Global),

        Type::Enum(container) => lower_enum_typ(container, context)?.ptr_type(AddressSpace::Global),

        Type::Pointer { pointee } => lower_pointer_typ(&*pointee, context)?.ptr_type(AddressSpace::Global),
    })
}

pub fn lower_strslice_type<'a, 'ctx>(context: &ModuleContext<'a, 'ctx>) -> StructType<'ctx> {
    let string_pointer_type = context.context.i8_type().ptr_type(AddressSpace::Global).as_basic_type_enum();
    let string_len_type = context.context.i64_type().as_basic_type_enum();

    context.context.struct_type(&[string_pointer_type, string_len_type], false)
}

pub fn lower_integer_type<'a, 'ctx>(bits: u32, context: &ModuleContext<'a, 'ctx>) -> Option<IntType<'ctx>> {
    Some(match bits {
        1 => context.context.bool_type(),
        8 => context.context.i8_type(),
        16 => context.context.i16_type(),
        32 => context.context.i32_type(),
        64 => context.context.i64_type(),
        _ => return None,
    })
}

pub fn lower_float_type<'a, 'ctx>(bits: u32, context: &ModuleContext<'a, 'ctx>) -> Option<FloatType<'ctx>> {
    Some(match bits {
        16 => context.context.f16_type(),
        32 => context.context.f32_type(),
        64 => context.context.f64_type(),
        _ => return None,
    })
}

pub fn lower_function_type<'a, 'ctx>(return_type: &Type, pars: &[Type], context: &ModuleContext<'a, 'ctx>) -> Option<FunctionType<'ctx>> {
    let param_types = pars.iter()
                          .filter_map(|par| lower_basic_typ(par, context))
                          .map(|par| par.into())
                          .collect::<Vec<BasicMetadataTypeEnum>>();

    Some(match return_type {
        Type::Void => context.context.void_type().fn_type(&param_types, false),
        basic => lower_basic_typ(basic, context)?.fn_type(&param_types, false),
    })
}

pub fn lower_tuple_type<'a, 'ctx>(tuple_items: &[Type], context: &ModuleContext<'a, 'ctx>) -> Option<StructType<'ctx>> {
    let mut tuple_items_lowered = Vec::with_capacity(tuple_items.len());

    for item in tuple_items {
        tuple_items_lowered.push(lower_basic_typ(item, context)?);
    }

    Some(context.context.struct_type(&tuple_items_lowered, false))
}
