use blir::{Type, TypeKind};
use inkwell::types::{BasicTypeEnum};

use crate::context::LibraryGenContext;

pub fn generate_type<'a, 'ctx>(typ: &Type, ctx: LibraryGenContext<'a, 'ctx>) -> Option<BasicTypeEnum<'ctx>> {
	match typ.kind() {
		TypeKind::Intrinsic(intrinsic) => {
			Some(BasicTypeEnum::IntType(match intrinsic.as_str() {
				"i64" => ctx.context().i64_type(),
				"i32" => ctx.context().i32_type(),
				"i16" => ctx.context().i16_type(),
				"i8" => ctx.context().i8_type(),
				"i1" => ctx.context().bool_type(),
				_ => return None
			}))
		}
		_ => panic!()
	}
}