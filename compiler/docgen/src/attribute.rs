use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Attribute {
	///
	/// What attribute is applied to the item
	/// 
	attribute: String
}

impl Attribute {
	pub fn compose(from: &blir::attributes::Attribute) -> Attribute {
		let attribute = from.name().to_string();

		Attribute {
			attribute
		}
	}
}