use prelude::WithSource;

use crate::{AstNode, Visibility, Attribute, Expression, Type};

#[derive(Debug, Clone)]
pub struct Enum {
	/// Attributes describing the enum
	attributes: Vec<WithSource<Attribute>>,

	/// The visibility of the enum
	visibility: Option<WithSource<Visibility>>,

	/// The name of the enum
	name: Option<WithSource<String>>,

	/// The type backing the enum
	repr: Option<WithSource<Type>>,

	/// Variants of the enum
	variants: Vec<WithSource<EnumVariant>>,
}

impl Enum {
	pub fn new(name: Option<WithSource<String>>, repr: Option<WithSource<Type>>, variants: Vec<WithSource<EnumVariant>>) -> Self {
		Enum {
			attributes: vec![],
			visibility: None,
			name,
			repr,
			variants,
		}
	}

	pub fn with_visibility(mut self, visibility: Option<WithSource<Visibility>>) -> Self {
		self.visibility = visibility;

		self
	}

	pub fn with_attributes(mut self, attributes: Vec<WithSource<Attribute>>) -> Self {
		self.attributes = attributes;

		self
	}

	pub fn node(&self) -> AstNode {
		let mut node = AstNode::new("enum");

		node.fold("name", &self.name);
		node.fold("visibility", &self.visibility);
		node.fold("repr", &self.repr);

		for variant in self.variants.iter() {
			node.add_child(variant.value().node());
		}

		node
	}
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
	/// The variant name
	name: WithSource<String>,

	associated: Vec<WithSource<Type>>,

	/// The assigned value of the variant
	assigned: Option<WithSource<Expression>>,
}

impl EnumVariant {
	pub fn new(name: WithSource<String>, associated: Vec<WithSource<Type>>) -> Self {
		Self {
			name,
			associated,
			assigned: None,
		}
	}

	pub fn assign_value(&mut self, value: WithSource<Expression>) {
		self.assigned = Some(value);
	}

	pub fn node(&self) -> AstNode {
		let mut node = AstNode::new("variant");

		node.set("name", &self.name);
		node.fold("value", &self.assigned);
		if self.associated.len() > 0 {
			let associated_list = self.associated.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ");

			node.set("associated", &associated_list);
		}

		node
	}
}