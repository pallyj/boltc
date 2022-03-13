use std::ops::{Deref, DerefMut};

use crate::{value::Value, typ::{Type, TypeKind}};

pub enum StatementKind {
	Eval { value: Value, escaped: bool },

	Bind { name: String, typ: Type, value: Value },

	Return { value: Option<Value> },
}

impl StatementKind {
	pub fn anon(self) -> Statement {
		Statement { kind: self, span: None }
	}

	pub fn spanned(self, span: Span) -> Statement {
		Statement { kind: self, span: Some(span) }
	}
}

type Span = u32;

pub struct Statement {
	kind: StatementKind,
	span: Option<Span>,
}

impl Statement {
	pub fn set_kind(&mut self, kind: StatementKind) {
		self.kind = kind;
	}

	pub fn span(&self) -> Option<&Span> {
		self.span.as_ref()
	}

	pub fn typ(&self) -> Type {
		match self.deref() {
			StatementKind::Bind { .. } => TypeKind::Void.anon(),
			StatementKind::Return { .. } => TypeKind::Divergent.anon(),
			StatementKind::Eval { value, escaped } => {
				let ty = value.typ.clone();

				if let TypeKind::Divergent = ty.deref() {
					return ty
				}

				if *escaped {
					TypeKind::Void.anon()
				} else {
					ty
				}
			}
		}
	}

	pub fn diverges(&self) -> bool {
		match self.deref() {
			StatementKind::Return { .. } => true,
			StatementKind::Eval { value, .. } => {
				if let TypeKind::Divergent = value.typ.deref() {
					true
				} else {
					false
				}
			}
			_ => false,
		}
	}
}

impl Deref for Statement {
    type Target = StatementKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl DerefMut for Statement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}