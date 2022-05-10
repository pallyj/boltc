use std::{collections::HashMap, sync::Arc, ops::Deref, cell::{RefCell, Ref}, fmt::Display};

use super::Type;

pub struct Enum {
	name: String,
	bits: u64,
	variants: RefCell<HashMap<String, EnumVariant>>,
}

impl Enum {
	pub fn new(name: String, bits: u64) -> EnumRef {
		EnumRef { enum_ref: Arc::new(Enum { name, variants: RefCell::new(HashMap::new()), bits }) }
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn tag(&self) -> Type {
		Type::Integer { bits: self.bits as u32 }
	}

	pub fn bits(&self) -> u64 {
		self.bits
	}

	pub fn add_variant(&self, variant: EnumVariant) {
		self.variants.borrow_mut()
			.insert(variant.name().clone(), variant);
	}

	pub fn get_variant(&self, variant_name: &str) -> Ref<EnumVariant> {
		Ref::map(self.variants.borrow(), |variants| variants.get(variant_name).unwrap())
	}

	pub fn variants(&self) -> Ref<HashMap<String, EnumVariant>> {
		self.variants.borrow()
	}
}

pub struct EnumVariant {
	name: String,
	tag: usize,
	tuple_type: Type,
}

impl EnumVariant {
	pub fn new(name: String, tag: usize, associated_types: Vec<Type>) -> Self {
		EnumVariant { name, tag, tuple_type: Type::Tuple(associated_types) }
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn tag(&self) -> usize {
		self.tag
	}

	pub fn tuple_type(&self) -> &Type {
		&self.tuple_type
	}
}

#[derive(Clone)]
pub struct EnumRef {
	enum_ref: Arc<Enum>
}

impl Deref for EnumRef {
    type Target = Enum;

    fn deref(&self) -> &Self::Target {
        &self.enum_ref
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "enum {}: i{} {{", self.name, self.bits)?;

		for (_, variant) in &*self.variants.borrow() {
			writeln!(f, "\t{variant}")?;
		}

		writeln!(f, "}}")
    }
}

impl Display for EnumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} = {}", self.name(), self.tuple_type(), self.tag())
    }
}

impl Display for EnumRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.enum_ref)
    }
}

impl PartialEq for EnumRef {
    fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.enum_ref, &other.enum_ref)
    }
}

impl Eq for EnumRef {
    fn assert_receiver_is_total_eq(&self) {}
}