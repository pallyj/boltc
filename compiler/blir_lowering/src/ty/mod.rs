use itertools::Itertools;

use crate::BlirLowerer;

mod struct_;
mod enum_;

impl<'a> BlirLowerer<'a> {
	pub fn lower_ty(&mut self, ty: &blir::typ::Type) -> mir::ty::Type {
		use blir::typ::TypeKind::*;
		
		match ty.kind() {
			Void => mir::ty::Type::void(),

			Integer { bits } => mir::ty::Type::int(*bits as u32),
			Float { bits } => mir::ty::Type::float(*bits as u32),

			Function { return_type, params, .. } => self.lower_ty(return_type)
														.func(params.iter().map(|param| self.lower_ty(param)).collect_vec()),
			Method { reciever, return_type, params } => self.lower_ty(return_type).func(std::iter::once(reciever.as_ref()).chain(params.iter()).map(|param| self.lower_ty(param)).collect_vec()),
			Struct(struct_ref) => mir::ty::Type::c_struct(self.builder.get_struct_id(&struct_ref.link_name())),
			Enum(enum_ref) => mir::ty::Type::c_enum(self.builder.get_enum_id(enum_ref.link_name())),
			Tuple(items, _) => mir::ty::Type::tuple(items.iter().map(|item| self.lower_ty(item)).collect_vec()),
			Array { item, len } => self.lower_ty(&item).array(*len),
			StrSlice => todo!(),
			Divergent => mir::ty::Type::void(),

			Named(name) => panic!("compiler error: failed to catch unresolved type `{name}`"),
			Member { parent, member } => panic!("compiler error: failed to catch unresolved type `{parent:?}.{member}`"),

			Metatype(metatype) => panic!("compiler error: failed to catch metatype {metatype:?}"),

			UnknownInfer |
			Infer { .. } => panic!("compiler error: failed to catch uninferred type"),

			SomeInteger => panic!("compiler error: failed to catch uninferred type"),
			SomeFloat => panic!("compiler error: failed to catch uninferred type"),
			SomeBool => panic!("compiler error: failed to catch uninferred type"),
			SomeFunction => panic!("compiler error: failed to catch uninferred type"),

			GenericParam(_) => panic!(""),
			HKRawPointer => panic!(""),
			GenericOf { higher_kind: _, params: _ } => panic!(),
			RawPointer { pointer_type } => self.lower_ty(pointer_type).shared_pointer(),

			Slice(_) => panic!(),

			Error => panic!("compiler error: failed to catch error type"),
		}
	}
}