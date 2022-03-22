mod struct_;

use std::{ops::{Deref, DerefMut}, fmt::{Debug}, sync::atomic::{AtomicU64, Ordering}};

use errors::Span;
pub use struct_::*;

static NEXT_INFER_KEY: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq)]
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
	Infer { key: u64 },

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

#[derive(Clone)]
pub struct Type {
	kind: TypeKind,
	span: Option<Span>,
}

impl Type {
	pub fn set_kind(&mut self, kind: TypeKind) {
		self.kind = kind;
	}

	pub fn kind(&self) -> &TypeKind {
		&self.kind
	}

	pub fn kind_mut(&mut self) -> &mut TypeKind {
		&mut self.kind
	}

	pub fn span(&self) -> Option<Span> {
		self.span
	}

	pub fn infer_specific(span: Span) -> Type {
		let key = NEXT_INFER_KEY.fetch_add(1, Ordering::AcqRel);

		Type { kind: TypeKind::Infer { key }, span: Some(span) }
	}

	pub fn infer() -> Type {
		let key = NEXT_INFER_KEY.fetch_add(1, Ordering::AcqRel);

		Type { kind: TypeKind::Infer { key }, span: None }
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

impl Eq for Type {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.deref() {
            TypeKind::Named(name) => write!(f, "#{name}"),
            TypeKind::Member { parent, member } => write!(f, "{parent:?}.{member}"),
            TypeKind::Infer { key } => write!(f, "_{key}"),
            TypeKind::Void => write!(f, "()"),
            TypeKind::Function { return_type, params, labels: _ } => {
				let params = params
					.iter()
					.map(|par| format!("{par:?}"))
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "func ({params}): {return_type:?}")
			},
            TypeKind::Struct(struct_ref) => write!(f, "struct {}", struct_ref.name()),
            TypeKind::Integer { bits } => write!(f, "i{bits}"),
            TypeKind::Float { bits } => write!(f, "f{bits}"),
            TypeKind::Divergent => write!(f, "!"),
            TypeKind::Metatype(t) => write!(f, "<{:?}>", t.clone().anon()),
            TypeKind::Error => write!(f, "error"),
        }
    }
}