use std::fmt::Display;

use prelude::WithSource;

use crate::{Expression, Type, Visibility, Attribute, AstNode};

#[derive(Debug, Clone)]
pub struct Let {
	attributes: Vec<WithSource<Attribute>>,
	visibility: Option<WithSource<Visibility>>,
	name: String,
	typ: Option<WithSource<Type>>,
	value: Option<WithSource<Expression>>,
}

impl Let {
	pub fn new(name: String, typ: Option<WithSource<Type>>, value: Option<WithSource<Expression>>) -> Self {
		Self {
			attributes: vec![],
			visibility: None,
			name,
			typ,
			value
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
		let mut node = AstNode::new("let");

		node.fold("visibility", &self.visibility);
		node.set("name", &self.name);
		node.fold("type", &self.typ);

		let mut value_node = AstNode::new("value");
		self.value.as_ref().map(|value| {
			value_node.add_child(value.value().node());
		});
		node.add_child(value_node);

		node
	}
}

#[derive(Debug, Clone)]
pub struct Var {
	attributes: Vec<WithSource<Attribute>>,
	visibility: Option<WithSource<Visibility>>,
	name: String,
	typ: Option<WithSource<Type>>,
	value: Option<WithSource<Expression>>,
}

impl Var {
	pub fn new(name: String, typ: Option<WithSource<Type>>, value: Option<WithSource<Expression>>) -> Self {
		Self {
			attributes: vec![],
			visibility: None,
			name,
			typ,
			value
		}
	}

	pub fn with_attributes(mut self, attrs: Vec<WithSource<Attribute>>) -> Self {
		self.attributes = attrs;
		self
	}

	pub fn with_visibility(mut self, vis: Option<WithSource<Visibility>>) -> Self {
		self.visibility = vis;
		self
	}

	pub fn node(&self) -> AstNode {
		let mut node = AstNode::new("let");

		node.fold("visibility", &self.visibility);
		node.set("name", &self.name);
		node.fold("type", &self.typ);
		node.fold("value", &self.value);

		node
	}
}