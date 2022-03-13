mod smt;
mod func;
mod method;

pub use smt::*;
pub use func::*;
pub use method::*;

use crate::{value::Span, typ::{Type, TypeKind}};

pub struct CodeBlock {
	statements: Vec<Statement>,
	span: Option<Span>,
}

impl CodeBlock {
	pub fn new(statements: Vec<Statement>, span: Span) -> CodeBlock {
		CodeBlock {
			statements,
			span: Some(span)
		}	
	}

	pub fn anon(statements: Vec<Statement>) -> CodeBlock {
		CodeBlock {
			statements,
			span: None,
		}
	}

	pub fn span(&self) -> Option<&Span> {
		self.span.as_ref()
	}

	pub fn statements(&self) -> &Vec<Statement> {
		&self.statements
	}

	pub fn typ(&self) -> Type {
		for smt in self.statements.iter() {
			if smt.diverges() {
				return TypeKind::Divergent.anon();
			}
		}

		if let Some(smt) = self.statements().last() {
			return smt.typ()
		}

		return TypeKind::Void.anon()
	}
}