use blir::attributes::{Attribute, Attributes};

use crate::AstLowerer;

impl AstLowerer {
	pub fn lower_attribute(&self, attribute: &parser::ast::attribute::Attribute) -> Attribute {
		let span = self.span(attribute.range());
		
		Attribute::new(attribute.attribute_name(), span)
	}

	pub fn lower_attributes(&self, attributes: parser::ast::attribute::Attributes) -> Attributes {
		let attributes = attributes.list()
			.map(|attribute| self.lower_attribute(&attribute));

		Attributes::new(attributes)
	}
}