use crate::lexer::SyntaxKind;

use super::find_token;

ast!(struct Attribute(Attribute));
ast!(struct Attributes(Attributes));

impl Attribute {
	pub fn attribute_name(&self) -> String {
		let func_name = self.0.children()
			.find(|node| node.kind() == SyntaxKind::FuncName)
			.unwrap();

		find_token(&func_name, SyntaxKind::Ident)
			.map(|name| name.text().to_string())
			.unwrap()
	}
}

impl Attributes {
	pub fn list(&self) -> impl Iterator<Item = Attribute> {
		self.0
			.children()
			.map(|attribute| Attribute::cast(attribute).unwrap())
	}
}