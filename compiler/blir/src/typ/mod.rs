mod struct_;

use std::ops::{Deref, DerefMut};

pub use struct_::*;

#[derive(Clone, PartialEq, Eq)]
pub enum TypeKind {
	// Virtual types

	/// A named type. This type is created by the parser.
	Named(String),
	
	/// A member type. This type is created by the parser
	Member {
		parent: Box<Type>,
		member: String,
	},
	
	/// Signifies the type must be inferred. This type is created by the parser
	Infer,

	// First-class types
	Void,
	Function {
		return_type: Box<Type>,
		params: Vec<Type>,
		labels: Vec<Option<String>>,
	},
	Struct(StructRef),

	// Intrinsic types
	Integer { bits: u64 },
	Float { bits: u64 },

	// Second-class types
	Divergent,
	Metatype(Box<TypeKind>),

	Error
}

impl TypeKind {
	pub fn anon(self) -> Type {
		Type { kind: self, span: None }
	}

	pub fn spanned(self, span: Span) -> Type {
		Type { kind: self, span: Some(span) }
	}
}

type Span = u32;

#[derive(Clone, Eq)]
pub struct Type {
	kind: TypeKind,
	span: Option<Span>,
}

impl Type {
	pub fn set_kind(&mut self, kind: TypeKind) {
		self.kind = kind;
	}

	pub fn span(&self) -> Option<Span> {
		self.span
	}
}

impl Deref for Type {
    type Target = TypeKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl DerefMut for Type {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}