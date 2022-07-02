use itertools::Itertools;
use serde::{Serialize, Deserialize};

use crate::{visibility::Visibility, func::Function, struct_::Struct, ty::Type, attribute::Attribute};

#[derive(Serialize, Deserialize)]
pub struct Enum {
	///
	/// 
	/// 
	pub meta: String,

	///
	/// 
	/// 
	pub attributes: Vec<Attribute>,

	///
	/// 
	/// 
	pub visibility: Visibility,

	///
	/// 
	/// 
	pub name: String,

	///
	/// 
	/// 
	pub variants: Vec<EnumVariant>,

	///
	/// The struct's methods
	/// 
	pub methods: Vec<Function>,

	///
	/// Structs defined under this struct
	/// 
	pub substructs: Vec<Struct>,

	///
	/// Enums defined under this struct
	/// 
	pub subenums: Vec<Enum>,

	///
	/// Typealiases defined under this struct
	/// 
	pub typealiases: Vec<()>
}

#[derive(Serialize, Deserialize)]
pub struct EnumVariant {
	///
	/// The doc comment on the variant
	/// 
	pub meta: String,

	///
	/// The name of the variant
	/// 
	pub name: String,

	///
	/// Types asscoiated with the enum variant
	/// 
	pub associated_types: Vec<(Option<String>, Type)>
}

impl Enum {
	pub fn compose(from: &blir::typ::EnumRef) -> Enum {
		let meta = from.meta();
		let attributes = from.attributes().iter().map(Attribute::compose).collect_vec();
		let visibility = Visibility::compose(from.visibility());
		let name = from.name().to_string();

		let variants = from.variants().iter().map(EnumVariant::compose).collect_vec();
		let methods = from.methods().iter().filter_map(Function::compose_method).collect_vec();
		let substructs = from.substructs().iter().map(Struct::compose).collect_vec();
		let subenums = from.subenums().iter().map(Enum::compose).collect_vec();

		Enum {
			meta,
			attributes,
			visibility,
			name,
			variants,
			methods,
			substructs,
			subenums,
			typealiases: Vec::new(),
		}
	}

	pub fn hide_below(&mut self, visibility: crate::visibility::Visibility) {
        self.methods.retain(|func| func.visibility >= visibility);
        self.substructs.retain(|func| func.visibility >= visibility);
        self.subenums.retain(|func| func.visibility >= visibility);

        for each_struct in &mut self.substructs {
            each_struct.hide_below(visibility);
        }

        for each_enum in &mut self.subenums {
            each_enum.hide_below(visibility);
        }
    }
}

impl EnumVariant {
	pub fn compose(from: &blir::typ::CaseRef) -> EnumVariant {
		let meta = from.meta();
		let name = from.name().clone();
		let associated_types = from.labels().iter().cloned().zip(from.associated_types().iter().map(Type::compose)).collect_vec();

		EnumVariant {
			meta,
			name,
			associated_types
		}
	}
}