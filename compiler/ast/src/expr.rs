use std::fmt::Display;

use prelude::*;

use crate::{FuncArg, CodeBlock, AstNode};

#[derive(Debug, Clone)]
pub enum Expression {
	// Literals
	StringLiteral(String),
	IntLiteral(u64),
	FloatLiteral(f64),
	CollectionLiteral(Vec<WithSource<Expression>>),
	RecordLiteral(Vec<(WithSource<Expression>, WithSource<Expression>)>),

	Variant(String),

	// Named
	Named(String),

	Unit,
	Tuple(Vec<WithSource<Expression>>),

	FuncCall(Box<WithSource<Expression>>, Vec<WithSource<FuncArg>>),
	Index(Box<WithSource<Expression>>, Box<WithSource<Expression>>),

	Member(Box<WithSource<Expression>>, String),
	OptionalMember(Box<WithSource<Expression>>, String),
	Apply {
		func: Box<WithSource<Expression>>,
		to_expr: Box<WithSource<Expression>>
	},

	InfixOperator(Box<WithSource<Expression>>, String, Box<WithSource<Expression>>),
	FixOperator(Box<WithSource<Expression>>, String),

	If {
		condition: Box<WithSource<Expression>>,
		positive: WithSource<CodeBlock>,
		negative: Option<WithSource<CodeBlock>>,
	}
}

impl Expression {
	pub fn node(&self) -> AstNode {
		match self {
			Self::Apply { func, to_expr } => {
				let mut node = AstNode::new("apply");
				node.add_child(func.value().node());
				node.add_child(to_expr.value().node());
				node
			}
			Self::CollectionLiteral(list) => {
				let mut node = AstNode::new("collection");
				for e in list.iter() {
					node.add_child(e.value().node());
				}
				node
			}
			Self::FixOperator(expr, op) => {
				let mut node = AstNode::new(op.as_str());
				node.add_child(expr.value().node());
				node
			}
			Self::FloatLiteral(f) => {
				let mut node = AstNode::new("literal");
				node.set("float", &f);
				node
			}
			Self::FuncCall(func, args) => {
				let mut node = AstNode::new("call");

				node.add_child(func.value().node());

				for a in args {
					node.add_child(a.value().node());
				}

				node
			}
			Self::If { condition, positive, negative } => {
				let mut node = AstNode::new("if");

				node.add_child(condition.value().node());
				
				let mut positive_node = AstNode::new("positive");
				positive.value().nodes(&mut positive_node);
				node.add_child(positive_node);

				if let Some(neg) = negative {
					let mut negative = AstNode::new("negatve");
					neg.value().nodes(&mut negative);
					node.add_child(negative);
				}

				node
			}
			Self::IntLiteral(i) => {
				let mut node = AstNode::new("literal");
				node.set("int", &i);
				node
			}
			Self::StringLiteral(s) => {
				let mut node = AstNode::new("literal");
				node.set("string", &s);
				node
			}
			Self::InfixOperator(left, op, right) => {
				let mut node = AstNode::new(&op);
				node.add_child(left.value().node());
				node.add_child(right.value().node());
				node
			}
			Self::Named(name) => {
				let mut node = AstNode::new("named");
				node.set("name", name);
				node
			}
			Self::Member(of, member_name) => {
				let mut node = AstNode::new("member");
				node.set("member-name", member_name);

				node.add_child(of.value().node());
				node
			}
			Self::OptionalMember(of, member_name) => {
				let mut node = AstNode::new("optional-member");
				node.set("member-name", member_name);

				node.add_child(of.value().node());
				node
			}
			Self::RecordLiteral(kv_pairs) => {
				let mut node = AstNode::new("record");

				for pair in kv_pairs.iter() {
					let mut item = AstNode::new("item");

					let mut key_node = AstNode::new("key");
					key_node.add_child(pair.0.value().node());

					let mut value_node = AstNode::new("value");
					value_node.add_child(pair.1.value().node());

					item.add_child(key_node);
					item.add_child(value_node);

					node.add_child(item);
				}

				node
			}
			_ => { AstNode::new("unknown") }
		}
	}
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::StringLiteral(ref string) => write!(f, "\"{string}\""),
			Self::IntLiteral(int) => write!(f, "{int}"),
			Self::FloatLiteral(float) => write!(f, "{float}"),

			Self::Named(ref name) => write!(f, "{name}"),

			Self::Tuple(ref members) => {
				let members = members
					.iter()
					.map(|member| member.to_string())
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "({members})")
			},

			Self::Index(ref value, ref idx) => write!(f, "({value}[{idx}])"),
			Self::FuncCall(ref func, ref args) => {
				let args = args
					.iter()
					.map(|arg| arg.to_string())
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "{func}({args})")
			},

			Self::Member(ref expr, ref member) => write!(f, "({expr}.{member})"),
			Self::OptionalMember(ref expr, ref member) => write!(f, "({expr}?.{member})"),
			Self::Apply { ref func, ref to_expr } => write!(f, "({to_expr} -> {func})"),

			Self::InfixOperator(left, ref op, right) => write!(f, "({} {} {})", op, left.value(), right.value()),
			Self::FixOperator(unit, ref op) => write!(f, "({} {})", op, unit.value()),

			_ => write!(f, ""),
		}
    }
}