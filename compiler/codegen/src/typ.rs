use blir::{Type, TypeKind};
use inkwell::{types::{BasicTypeEnum}};

use crate::context::LibraryGenContext;

pub fn generate_type<'a, 'ctx>(typ: &Type, ctx: LibraryGenContext<'a, 'ctx>) -> Option<BasicTypeEnum<'ctx>> {
	match typ.kind() {
		TypeKind::Intrinsic(intrinsic) => {
			Some(match intrinsic.as_str() {
				"i64" => BasicTypeEnum::IntType(ctx.context().i64_type()),
				"i32" => BasicTypeEnum::IntType(ctx.context().i32_type()),
				"i16" => BasicTypeEnum::IntType(ctx.context().i16_type()),
				"i8" => BasicTypeEnum::IntType(ctx.context().i8_type()),
				"i1" => BasicTypeEnum::IntType(ctx.context().bool_type()),

				"f64" => BasicTypeEnum::FloatType(ctx.context().f64_type()),
				"f32" => BasicTypeEnum::FloatType(ctx.context().f32_type()),
				"f16" => BasicTypeEnum::FloatType(ctx.context().f16_type()),

				_ => return None
			})
		}
		TypeKind::StructRef(r#struct) => {
			Some(ctx.types.get_type(&r#struct.link_name()))
		}
		_ => {
			println!("{typ}");
			panic!()
		}
	}
}