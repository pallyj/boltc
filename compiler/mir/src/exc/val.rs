use std::collections::HashMap;

use crate::val::ConstValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Int(u64),
	Float(f64),

	Tuple(Vec<Value>),

	Function(String),
	ExternFunction(String),

	Struct(HashMap<String, Value>),

	Undef,
}

impl Value {
	pub fn from_const(constant: ConstValue) -> Self {
		use ConstValue::*;
		
		match constant {
			Integer(n) => Self::Int(n),
			Float(n) => Self::Float(n),
			String(_) => panic!()
		}
	}
}