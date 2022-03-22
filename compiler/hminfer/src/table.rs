use std::collections::HashMap;

use blir::typ::{TypeKind};

use crate::variant::TypeVariant;

pub struct TypeTable {
	types: HashMap<u64, TypeKind>,
}

impl TypeTable {
	pub fn new() -> TypeTable {
		TypeTable {
			types: HashMap::new()
		}
	}

	pub fn insert_variant(&mut self, key: u64, variant: TypeVariant) {
		let ty = match variant {
			TypeVariant::Diverges => TypeKind::Divergent,
			TypeVariant::Void => TypeKind::Void,

			TypeVariant::IntrinsicInteger { bits } => TypeKind::Integer { bits },
			TypeVariant::IntrinsicFloat { bits } => TypeKind::Float { bits },
			TypeVariant::IntrinsicBool => TypeKind::Integer { bits: 1 },

			_ => { return }
		};

		self.types.insert(key, ty);
	}

	pub fn get(&self, key: &u64) -> Option<&TypeKind> {
		self.types.get(&key)
	}
}