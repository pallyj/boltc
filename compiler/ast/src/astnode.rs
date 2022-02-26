use std::{collections::HashMap, fmt::Display};

pub struct AstNode {
	name: String,
	properties: HashMap<&'static str, String>,
	children: Vec<AstNode>,
}

impl AstNode {
	pub fn new(name: &str) -> AstNode {
		AstNode {
			name: name.to_string(),
			properties: HashMap::new(),
			children: Vec::new(),
		}
	}

	pub fn set<T: Display>(&mut self, name: &'static str, property: &T) {
		self.properties.insert(name, property.to_string());
	}

	pub fn fold<T: Display>(&mut self, name: &'static str, property: &Option<T>) {
		if let Some(prop) = property {
			self.properties.insert(name, prop.to_string());
		}
	}

	pub fn add_child(&mut self, node: AstNode) {
		self.children.push(node);
	}
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.name)?;

		for prop in self.properties.iter() {
			writeln!(f, "  {} = {}", prop.0, prop.1)?;
		}

		if self.children.len() > 0 && self.properties.len() > 0 {
			writeln!(f)?;
		}

		write!(f, "  {}", self.children.iter().map(|c| c.to_string().replace("\n", "\n  ")).collect::<Vec<_>>().join("\n  "))
    }
}