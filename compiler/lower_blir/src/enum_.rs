use blir::typ::{EnumRef, TypeKind};
use blirssa::typ::EnumVariant;

use crate::BlirLowerer;

impl BlirLowerer {
	pub(super) fn lower_enum_definition(&mut self, enum_def: EnumRef) {
		let ssa_lib = self.ssa_library_mut();

		let mangle = enum_def.mangle();
		enum_def.set_link_name(mangle);

		let TypeKind::Integer { bits } = enum_def.repr_type().kind() else {
			panic!()
		};

        ssa_lib.add_enum(&enum_def.link_name(), *bits);
	}

	pub(super) fn lower_enum_signature(&mut self, enum_def: EnumRef) {
		let self_enum = self.ssa_library()
                            .get_enum(&enum_def.link_name())
                            .cloned()
                            .unwrap();

		for variant in enum_def.variants().iter() {
			let variant_name = variant.name().clone();
			let variant_tag = variant.tag();
			let variant_tuple = variant.associated_types()
				.iter()
				.map(|typ| self.lower_type(typ))
				.collect();

			let ssa_variant = EnumVariant::new(variant_name, variant_tag, variant_tuple);

			self_enum.add_variant(ssa_variant);
		}
	}
}