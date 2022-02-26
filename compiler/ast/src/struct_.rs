use std::fmt::Display;

use prelude::WithSource;

use crate::{Type, Visibility, Attribute, Let, Func, Var, AstNode};

#[derive(Debug, Clone)]
pub struct Struct {
	attributes: Vec<WithSource<Attribute>>,

	vis: Option<WithSource<Visibility>>,

	name: Option<String>,

	implements: Vec<WithSource<Type>>,

	items: Vec<WithSource<StructItem>>,	
}

#[derive(Debug, Clone)]
pub enum StructItem {
	Variable(Var),
	Let(Let),
	Method(Func),
	SubStruct(Struct)
}

impl Struct {
	pub fn new(name: Option<String>, implements: Vec<WithSource<Type>>, items: Vec<WithSource<StructItem>>) -> Self {
		Self {
			attributes: vec![],
			vis: None,
			name,
			implements,
			items,
		}
	}

	pub fn with_visibility(mut self, visibility: Option<WithSource<Visibility>>) -> Self {
		self.vis = visibility;
		self
	}

	pub fn with_attributes(mut self, attributes: Vec<WithSource<Attribute>>) -> Self {
		self.attributes = attributes;
		self
	}

	pub fn node(&self) -> AstNode {
		let mut node = AstNode::new("struct");

		node.fold("visibility", &self.vis);
		node.fold("name", &self.name);

		for item in self.items.iter() {
			match item.value() {
				StructItem::Method(f) => node.add_child(f.node()),
				StructItem::SubStruct(s) => node.add_child(s.node()),
				StructItem::Let(l) => node.add_child(l.node()),
				_ => {}
			}
		}

		node
	}

	pub fn items(&self) -> &Vec<WithSource<StructItem>> {
		&self.items
	}

	pub fn into_items(self) -> Vec<WithSource<StructItem>> {
		self.items
	}

	pub fn name(&self) -> &Option<String> {
		&self.name
	}

	pub fn visibility(&self) -> &Option<WithSource<Visibility>> {
		&self.vis
	}
}