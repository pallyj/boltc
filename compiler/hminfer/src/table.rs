use std::{collections::HashMap, fmt::Debug};

use blir::typ::{TypeKind, Type};

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
			TypeVariant::Void => {
				TypeKind::Void
			},

			TypeVariant::IntrinsicInteger { bits } => TypeKind::Integer { bits },
			TypeVariant::IntrinsicFloat { bits } => TypeKind::Float { bits },
			TypeVariant::IntrinsicBool => TypeKind::Integer { bits: 1 },

			TypeVariant::Struct(r#struct) => TypeKind::Struct(r#struct),

			_ => { return }
		};

		self.types.insert(key, ty);
	}

	pub fn get(&self, key: &u64) -> Option<&TypeKind> {
		self.types.get(&key)
	}
}

impl Debug for TypeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.types {
			write!(f, "{k}: {v:?}")?;
		}

		Ok(())
    }
}

pub struct GuessTable {
	types: HashMap<u64, Type>
}

impl GuessTable {

	pub fn new() -> GuessTable {
		GuessTable {
			types: HashMap::new()
		}
	}

	pub fn insert(&mut self, key: u64, ty: Type) {
		self.types.insert(key, ty);
	}

	pub fn get(&self, key: &u64) -> Option<&Type> {
		self.types.get(&key)
	}
}