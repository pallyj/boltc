use std::fmt::Debug;

use errors::Span;

use crate::{value::Value, typ::{Type, TypeKind}};

#[derive(Clone)]
pub enum PatternKind {
	Wildcard,
	
	Bind(String),

	Variant { variant: Value,
			  items: Vec<Pattern> },

	Literal { value: Value },

	Tuple { items: Vec<Pattern> }
}

impl PatternKind {
	pub fn with_span(self, span: Span) -> Pattern {
		Pattern { kind: self, span, match_type: Type::infer() }
	}

	pub fn with_type(self, ty: Type) -> Pattern {
		Pattern { kind: self, span: Span::empty(), match_type: ty }
	}
}

#[derive(Clone)]
pub struct Pattern {
	pub kind: PatternKind,
	pub span: Span,
	pub match_type: Type
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
			PatternKind::Wildcard => write!(f, "_"),
			PatternKind::Bind(name) => write!(f, "{name}"),
			PatternKind::Literal { value } => write!(f, "{value:?}"),
			PatternKind::Tuple { items } => write!(f, "({})", items.iter().map(|item| format!("{item:?}")).collect::<Vec<_>>().join(", ") ),
			PatternKind::Variant { variant, items } => write!(f, "{variant:?}({})", items.iter().map(|item| format!("{item:?}")).collect::<Vec<_>>().join(", ") )
		}
    }
}

impl Pattern {
	pub fn match_type(&self) -> &Type {
		&self.match_type
	}

	pub fn match_type_mut(&mut self) -> &mut Type {
		&mut self.match_type
	}

	pub fn matches_any(&self) -> bool {
		matches!(&self.kind, PatternKind::Bind(..) | PatternKind::Wildcard)
	}

	pub fn has_children(&self) -> bool {
		matches!(&self.kind, PatternKind::Tuple { .. })
	}
}

impl Default for Pattern {
    fn default() -> Self {
        Self { kind: PatternKind::Wildcard, span: Span::empty(), match_type: TypeKind::Void.anon() }
    }
}

/*

match dir {
	.left(1) => 101,
	.right(1) => 201,
	.left(2) => 102,
	.right(2) => 101,
	.left(n) => n,
	.right(n) => n,
}

var res: Int

switch variant {
	.left => ,
	.right => ,
}


*/