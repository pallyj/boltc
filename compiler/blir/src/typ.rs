use std::{sync::{Arc, Mutex}, fmt::Display};

use prelude::Source;

use crate::{func::FuncSig, structdef::StructDef, class::ClassDef, protocol::ProtocolDef, enumdef::EnumDef};

#[derive(Clone)]
pub enum TypeKind {
	/// A blank type
	Unit,

	/// An unresolved, named type
	Named(String),

	/// A tuple
	Tuple(Vec<Type>),

	/// A type representing a function
	Func(Box<FuncSig>),

	/// A type respresenting a struct
	StructRef(Arc<Mutex<StructDef>>),

	/// A type representing a class
	ClassRef(Arc<Mutex<ClassDef>>),

	/// A type representing an enum
	EnumRef(Arc<Mutex<EnumDef>>),

	/// A type representing a VTable of a protocol
	ProtocolRef(Arc<Mutex<ProtocolDef>>),

	/// A type built in to the language
	Intrinsic(String),

	/// A type inference context
	Infer(u64),

	Diverging
}

#[derive(Clone)]
pub struct Type {
	kind: TypeKind,
	source: Option<Source>,
}

impl Type {
	pub fn new(kind: TypeKind, source: Source) -> Type {
		Type { kind, source: Some(source) }
	}

	pub fn new_anon(kind: TypeKind) -> Type {
		Type { kind, source: None}
	}

	pub fn kind(&self) -> &TypeKind {
		&self.kind
	}

	pub fn kind_mut(&mut self) -> &mut TypeKind {
		&mut self.kind
	}
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::Unit => write!(f, "()"),
			Self::Tuple(items) => {
				let items = items
					.iter()
					.map(|item| item.to_string())
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "({items})")
			}
			Self::Func(sig) => {
				let pars = sig.parameters();
				let ret = sig.return_type();

				let pars = pars
					.iter()
					.map(|par| par.to_string())
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "func ({pars}): {ret}")
			}
			Self::Intrinsic(i_name) => {
				write!(f, "{i_name}")
			}
			Self::Infer(ctx) => write!(f, "infer{ctx}"),
			Self::Named(name) => {
				write!(f, "${name}")
			}

			Self::StructRef(r#struct) => {
				write!(f, "struct")
			}

			Self::ClassRef(r#class) => {
				write!(f, "class")
			}

			Self::EnumRef(r#enum) => {
				write!(f, "enum")
			}

			Self::ProtocolRef(protocol) => {
				write!(f, "protocol")
			}

			Self::Diverging => write!(f, "!")
		}
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}