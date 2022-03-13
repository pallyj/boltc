use std::fmt::Display;

use crate::{value::Value, typ::Type};

#[derive(Clone)]
pub struct LabelValue {
	pub (crate) label: u64,
	pub (crate) typ: Type,
}

impl LabelValue {
	pub fn typ(&self) -> Type {
		self.typ.clone()
	}

	pub fn typ_ref(&self) -> &Type {
		&self.typ
	}

	pub fn label(&self) -> u64 {
		self.label
	}
}

impl Display for LabelValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} %{}", self.typ, self.label)
    }
}

pub enum Instruction {
	Assign {
		label: LabelValue,
		value: Value,
	},
	AssignPtr {
		pointer: LabelValue,
		value: LabelValue,
	},
	Return {
		value: Option<LabelValue>,
	}
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Instruction::Assign { label, value } => {
				write!(f, "%{} = {}", label.label(), value)
			}

			Instruction::AssignPtr { pointer, value } => {
				write!(f, "assign-ptr {pointer}, {value}")
			}

			Instruction::Return { value } => {
				if let Some(value) = value {
					write!(f, "return {value}")
				} else {
					write!(f, "return")
				}
			}
		}
    }
}