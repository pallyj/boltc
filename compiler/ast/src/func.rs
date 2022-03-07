use std::fmt::Display;

use prelude::WithSource;

use crate::{Attribute, Visibility, Expression, CodeBlock, Type, AstNode};

#[derive(Debug, Clone)]
pub struct Func {
	attributes: Vec<WithSource<Attribute>>,
	visibility: Option<WithSource<Visibility>>,

	name: Option<String>,

	pars: Vec<WithSource<FuncPar>>,

	return_type: Option<WithSource<Type>>,

	code: Option<WithSource<CodeBlock>>,
}

impl Func {
	pub fn new( name: Option<String>, pars: Vec<WithSource<FuncPar>>, return_type: Option<WithSource<Type>>, code: WithSource<CodeBlock> ) -> Func {
		Func {
			attributes: vec![],
			visibility: None,
			name,
			pars,
			return_type,
			code: Some(code),
		}
	}

	pub fn new_extern(name: Option<String>, pars: Vec<WithSource<FuncPar>>, return_type: Option<WithSource<Type>>) -> Func {
		Func {
			attributes: vec![],
			visibility: None,
			name,
			pars,
			return_type,
			code: None,
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

	pub fn code(&self) -> Option<&CodeBlock> {
		self.code.as_ref().map(|code| code.value())
	}

	pub fn node(&self) -> AstNode {
		let mut node = AstNode::new("func");

		node.fold("name", &self.name);
		node.fold("visibility", &self.visibility);
		node.fold("return-type", &self.return_type);

		self.code
			.as_ref()
			.map(|code| code.value().nodes(&mut node));

		node
	}

	pub fn name(&self) -> &Option<String> {
		&self.name
	}

	pub fn parameters(&self) -> &Vec<WithSource<FuncPar>> {
		&self.pars
	}

	pub fn return_type(&self) -> &Option<WithSource<Type>> {
		&self.return_type
	}
}

#[derive(Debug, Clone)]
pub struct FuncPar {
	label: Option<String>,
	bind: String,
	typ: WithSource<Type>,
}

impl FuncPar {
	pub fn new(label: Option<String>, bind: String, typ: WithSource<Type>) -> Self {
		Self {
			label,
			bind,
			typ
		}
	}

	pub fn label(&self) -> &Option<String> {
		&self.label
	}

	pub fn bind_name(&self) -> &String {
		&self.bind
	}

	pub fn typ(&self) -> &WithSource<Type> {
		&self.typ
	}
}

#[derive(Debug, Clone)]
pub struct FuncArg {
	label: Option<String>,
	value: WithSource<Expression>,
}

impl FuncArg {

	pub fn new(label: Option<String>, value: WithSource<Expression>) -> Self {
		FuncArg {
			label,
			value
		}
	}

	pub fn label_and_value(self) -> (Option<String>, WithSource<Expression>) {
		(self.label, self.value)
	}

	pub fn node(&self) -> AstNode {
		let mut node = AstNode::new("arg");
		node.fold("label", &self.label);
		node.add_child(self.value.value().node());
		node
	}
}

impl Display for FuncArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.label {
			write!(f, "{label}: ")?;
		}

		write!(f, "{}", self.value)
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "func:")?;

		if let Some(ref vis) = self.visibility {
			writeln!(f, "\tvisibility = {}", vis.value())?;
		}

        if let Some(ref name) = self.name {
			writeln!(f, "\tname = {name}")?;
		}

		if let Some(ref return_type) = self.return_type {
			writeln!(f, "\treturns {}", return_type.value())?;
		}



		Ok(())
    }
}