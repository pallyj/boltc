mod var;

pub use var::*;

use std::ops::{DerefMut, Deref};

use crate::typ::{Type, TypeKind};

pub enum ValueKind {
	// Virtual Values
	Named(String),
	Member {
		parent: Box<Value>,
		member: String,
	},

	// Literal Values
	IntLiteral(u64),
	FloatLiteral(f64),
	BoolLiteral(bool),

	// Variable Values
	Metatype(TypeKind),
	LocalVariable(String),
	FunctionParam(String),

	// Functions
	UnaryIntrinsic {
		intrinsic: u64,
		arg: Box<Value>,
	},
	BinaryIntrinsic { 
		intrinsic: u64,
		left: Box<Value>,
		right: Box<Value>
	},

	// Second-class Values
	Unit,

	Error,
}

impl ValueKind {
	pub fn anon(self, typ: Type) -> Value {
		Value { kind: self, span: None, typ }
	}

	pub fn spanned(self, typ: Type, span: Span) -> Value {
		Value { kind: self, span: Some(span), typ }
	}
}

pub type Span = u32;

pub struct Value {
	pub kind: ValueKind,
	pub span: Option<Span>,
	pub typ: Type,
}

impl Deref for Value {
    type Target = ValueKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

pub struct FunctionArgs {

}