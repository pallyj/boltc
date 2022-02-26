use prelude::WithSource;

use crate::{Expression, AstNode, Type};

#[derive(Debug, Clone)]
pub enum Statement {
	Expr(Expression),

	Bind { name: String, typ: Option<WithSource<Type>>, expr: Option<WithSource<Expression>> },

	Break(Option<String>),
	Continue(Option<String>),
	Return(Option<WithSource<Expression>>),
	Throw(WithSource<Expression>),

	While {
		condition: WithSource<Expression>,
		code: CodeBlock,
	},
	Repeat {
		code: CodeBlock
	}
}

impl Statement {
	pub fn has_value(&self) -> bool {
		match self {
			Self::Expr(_) => true,
			_ => false
		}
	}

	pub fn node(&self) -> AstNode {
		match self {
			Self::Break(label) => {
				let mut node = AstNode::new("break");
				node.fold("label", &label);
				node
			}
			Self::Continue(label) => {
				let mut node = AstNode::new("continue");
				node.fold("label", &label);
				node
			}
			Self::Bind { name, typ, expr } => {
				let mut node = AstNode::new("let");
				node.set("name", &name);

				if let Some(typ) = typ {
					let mut typ_node = AstNode::new("type");
					typ_node.add_child(typ.value().node());
					node.add_child(typ_node);
				}

				if let Some(expr) = expr {
					let mut expr_node = AstNode::new("value");

					expr_node.add_child(expr.value().node());

					node.add_child(expr_node)
				}

				node
			}
			Self::Expr(e) => {
				e.node()
			}
			Self::Repeat { code } => {
				let mut node = AstNode::new("repeat");

				code.nodes(&mut node);

				node
			}
			Self::Return(val) => {
				let mut node = AstNode::new("return");
				node.fold("value", &val);
				node
			}
			Self::Throw(err) => {
				let mut node = AstNode::new("throw");
				node.set("error", err);
				node
			}
			Self::While { condition, code } => {
				let mut node = AstNode::new("while");
				node.set("condition", condition);

				code.nodes(&mut node);

				node
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct CodeBlockItem {
	smt: WithSource<Statement>,
	can_return: bool,
}

impl CodeBlockItem {
	pub fn new(smt: WithSource<Statement>, can_return: bool) -> Self {
		Self {
			smt,
			can_return
		}
	}

	pub fn can_return(&self) -> bool {
		self.can_return
	}

	pub fn statement(&self) -> &WithSource<Statement> {
		&self.smt
	}

	pub fn into_statement(self) -> WithSource<Statement> {
		self.smt
	}
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
	statements: Vec<CodeBlockItem>
}

impl CodeBlock {
	pub fn new(statements: Vec<CodeBlockItem>) -> Self {
		Self {
			statements
		}
	}

	pub fn statements(&self) -> &Vec<CodeBlockItem> {
		&self.statements
	}

	pub fn into_statements(self) -> Vec<CodeBlockItem> {
		self.statements
	}

	pub fn nodes(&self, on: &mut AstNode) {
		for smt in self.statements.iter() {
			on.add_child(smt.smt.value().node());
		}
	}
}