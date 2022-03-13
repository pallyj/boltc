use std::fmt::Display;

use crate::{typ::Type, code::FunctionRef};

use super::{LabelValue, BinaryIntrinsicFn, UnaryIntrinsicFn};

pub enum Value {
	IntegerLiteral { typ: Type, value: u64 },
	FloatLiteral { typ: Type, value: f64 },

	UnaryIntrinsic {
		name: UnaryIntrinsicFn,
		arg: LabelValue,
		return_type: Type,
	},

	BinaryIntrinsic {
		name: BinaryIntrinsicFn,
		left: LabelValue,
		right: LabelValue,
		return_type: Type,
	},

	AllocOnStackUndef {
		typ: Type,
	},

	AllocOnStack {
		value: LabelValue,
		typ: Type,
	},

	Deref {
		pointer: LabelValue,
		typ: Type,
	},

	/// deref-struct-field "field_name" (struct) : (type)
	/// Dereferences a field of a struct
	/// The value passed to this instruction can be either a pointer to a struct or the struct itself
	DerefStructField {
		r#struct: LabelValue,
		field: String,
		typ: Type,
	},

	/// access-struct-field "field_name" (struct) : (type)
	/// Returns a pointer to a field of a struct
	/// The value passed to this instruction must be a pointer to a struct
	AccessStructField {
		r#struct: LabelValue,
		field: String,
		typ: Type
	},

	Function { function: FunctionRef },
	Call { function: LabelValue, args: Vec<LabelValue>, typ: Type }
}

impl Value {
	pub fn typ(&self) -> Type {
		match self {
			Self::IntegerLiteral { typ, .. } => typ,
			Self::FloatLiteral { typ, .. } => typ,

			Self::UnaryIntrinsic { return_type, .. } => return_type,
			Self::BinaryIntrinsic { return_type, .. } => return_type,

			Self::Function { function } => return function.typ(),
			Self::Call { typ, .. } => typ,

			Self::AllocOnStackUndef { typ, .. } => typ,
			Self::AllocOnStack { typ, .. } => typ,
			Self::Deref { typ, .. } => typ,

			Self::AccessStructField { typ, .. } => typ,
			Self::DerefStructField { typ, .. } => typ,
		}.clone()
	}
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Value::IntegerLiteral { typ, value } => write!(f, "integer-literal {value} : {typ}"),
			Value::FloatLiteral { typ, value } => write!(f, "float-literal {value} : {typ}"),

			Value::UnaryIntrinsic { name, arg, return_type } => write!(f, "intrinsic \"{name}\" ( {arg} ) : {return_type}", name = name.name()),
			Value::BinaryIntrinsic { name, left, right, return_type } => write!(f, "intrinsic \"{name}\" ( {left}, {right} ) : {return_type}", name = name.name()),

			Value::AllocOnStackUndef { typ } => write!(f, "alloc-on-stack : {typ}"),
			Value::AllocOnStack { value, typ } => write!(f, "alloc-on-stack {value} : {typ}"),
			Value::Deref { pointer, typ } => write!(f, "deref {pointer} : {typ}"),

			Value::Function { function } => write!(f, "function \"{name}\" : {typ}", name = function.name(), typ = function.typ()),
			Value::Call { function, args, typ } => {
				let args = args
					.iter()
					.map(|arg| arg.to_string())
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "call {function} ({args}) : {typ}")
			}

			Value::AccessStructField { r#struct, field, typ } => write!(f, "access-struct-field \"{field}\" {struct} : {typ}"),
			Value::DerefStructField { r#struct, field, typ } => write!(f, "deref-struct-field \"{field}\" {struct} : {typ}"),
		}
    }
}