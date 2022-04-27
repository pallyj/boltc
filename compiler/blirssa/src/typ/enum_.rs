use std::{collections::HashMap, sync::Arc, ops::Deref, cell::{RefCell, Ref}, fmt::Display};

pub struct Enum {
	name: String,
	variants: RefCell<HashMap<String, EnumVariant>>,
}

impl Enum {
	pub fn new(name: String) -> EnumRef {
		EnumRef { enum_ref: Arc::new(Enum { name, variants: RefCell::new(HashMap::new()) }) }
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn add_variant(&self, variant: EnumVariant) {
		self.variants.borrow_mut()
			.insert(variant.name().clone(), variant);
	}

	pub fn get_variant(&self, variant_name: &str) -> Ref<EnumVariant> {
		Ref::map(self.variants.borrow(), |variants| variants.get(variant_name).unwrap())
	}
}

pub struct EnumVariant {
	name: String,
	tag: usize,
}

impl EnumVariant {
	pub fn new(name: String, tag: usize) -> Self {
		EnumVariant { name, tag }
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn tag(&self) -> usize {
		self.tag
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
        writeln!(f, "enum {} {{", self.name)?;

		for (_, variant) in &*self.variants.borrow() {
			writeln!(f, "\t{variant}")?;
		}

		writeln!(f, "}}")
    }
}

impl Display for EnumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name(), self.tag())
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