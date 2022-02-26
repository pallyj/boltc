use std::fmt::Display;

use crate::AstNode;

#[derive(Debug, Clone)]
pub struct Import {
	library: String
}

impl Import {
	pub fn new(library: String) -> Self {
		Import {
			library
		}
	}

	pub fn library(&self) -> String {
		self.library.clone()
	}

	pub fn node(&self) -> AstNode {
		let mut node = AstNode::new("import");

		node.set("library", &self.library);

		node
	}
}

impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "import {}", self.library)
    }
}