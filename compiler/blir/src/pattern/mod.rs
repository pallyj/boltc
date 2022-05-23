use std::fmt::Debug;

use errors::Span;

use crate::{value::Value, typ::{Type, TypeKind}};

#[derive(Clone)]
pub enum PatternKind {
	Wildcard,
	
	Bind(String),

	Variant { variant: Value,
			  items: Vec<Pattern>,
			  labels: Vec<Option<String>> },

	Literal { value: Value },

	Integer { value: u64 },

	Tuple { items:  Vec<Pattern>,
			labels: Vec<Option<String>> }
}

impl PatternKind {
	pub fn with_span(self, span: Span) -> Pattern {
		Pattern { kind: self, span, match_type: Type::infer().kind.spanned(span) }
	}

	/*pub fn with_type(self, ty: Type) -> Pattern {
		Pattern { kind: self, span: Span::empty(), match_type: ty }
	}*/

	pub fn with(self, span: Span, ty: Type) -> Pattern {
		Pattern { kind: self, span, match_type: ty.kind.spanned(span) }
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
			PatternKind::Integer { value } => write!(f, "{value}"),
			PatternKind::Literal { value } => write!(f, "{value:?}"),
			PatternKind::Tuple { items, labels } => write!(f, "({})", items.iter().zip(labels).map(|(item, label)| if let Some(label) = label { format!("{label}: {item:?}") } else { format!("{item:?}") }).collect::<Vec<_>>().join(", ") ),
			PatternKind::Variant { variant, items, labels } =>
				write!(f, "{variant:?}({})", items.iter()
												  .zip(labels)
												  .map(|(item, label)| if let Some(label) = label {
													  format!("{label}: {item:?}")
												  } else {
													  format!("{item:?}")
												  } )
												  .collect::<Vec<_>>()
												  .join(", ") )
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
		match &self.kind {
			PatternKind::Tuple { .. } => true,
			PatternKind::Variant { .. } => true,
			_ => false
		}
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