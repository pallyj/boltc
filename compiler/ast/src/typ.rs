use std::fmt::Display;

use prelude::WithSource;

use crate::{Expression, AstNode};

#[derive(Debug, Clone)]
pub enum Type {
	Named(String),

	Unit,
	Tuple(Vec<WithSource<Type>>),
	Function {
		takes: Vec<WithSource<Type>>,
		returns: Box<WithSource<Type>>,
	},

	Array {
		unit: Box<WithSource<Type>>,
		len: Expression
	},

	// Sugar
	Optional(Box<WithSource<Type>>),
	Fallible {
		success: Box<WithSource<Type>>,
		error: Box<WithSource<Type>>,
	},
	Collection(Box<WithSource<Type>>),
	Record {
		key: Box<WithSource<Type>>,
		value: Box<WithSource<Type>>
	}
}

impl Type {
	pub fn node(&self) -> AstNode {
		AstNode::new("type")
	}
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Type::Named(ref name) => write!(f, "{name}"),

			Type::Unit => write!(f, "()"),
			Type::Tuple(ref members) => {
				let members = members
					.iter()
					.map(|member| member.to_string())
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "({members})")
			},
			Type::Function { returns, takes } => {
				let args = takes
					.iter()
					.map(|t| t.to_string())
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "({args}) -> {returns}")
			}
			Type::Array { unit, len } => {
				write!(f, "{unit}[{len}]")
			}
			
			Type::Optional(unit) => write!(f, "{unit}?"),
			Type::Fallible { success, error } => write!(f, "{success} throws {error}"),
			Type::Collection(unit) => write!(f, "[{unit}]"),
			Type::Record { key, value } => write!(f, "[{key}: {value}]"),

			_ => write!(f, ""),
		}
    }
}