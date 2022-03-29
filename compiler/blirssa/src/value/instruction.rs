use std::fmt::Display;

use crate::{value::Value, typ::Type, code::BlockRef};

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

	pub fn void() -> LabelValue {
		LabelValue { label: u64::MAX, typ: Type::Void }
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
	Branch {
		condition: LabelValue,
		positive: BlockRef,
		negative: BlockRef
	},
	AlwaysBranch {
		block: BlockRef,
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

			Instruction::Branch { condition, positive, negative } => {
				write!(f, "br if {condition} {positive}; else {negative}",
					positive = positive.label(),
					negative = negative.label())
			}

			Instruction::AlwaysBranch { block } => {
				write!(f, "br {block}", block = block.label())
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