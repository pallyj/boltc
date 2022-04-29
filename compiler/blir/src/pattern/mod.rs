use std::fmt::Debug;

use errors::Span;

use crate::value::Value;

#[derive(Clone)]
pub enum PatternKind {
	Wildcard,
	
	//Bind { name: String },

	//Variant { name: String },

	Literal { value: Value },


}

impl PatternKind {
	pub fn with_span(self, span: Span) -> Pattern {
		Pattern { kind: self, span }
	}
}

#[derive(Clone)]
pub struct Pattern {
	pub kind: PatternKind,
	span: Span
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
			PatternKind::Wildcard => write!(f, "_"),
			PatternKind::Literal { value } => write!(f, "{value:?}"),
		}
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