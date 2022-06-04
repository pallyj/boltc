mod term;
mod local;

use std::fmt::Display;

pub use term::*;
pub use local::*;

use crate::val::{Place, RValue};

/// 
/// 
/// 
pub enum InstructionKind {
	///
	/// Assigns a value to a place
	/// 
	Assign(Place, RValue),

	///
	/// Drops the value stored in a place
	/// 
	Drop(Place),

	///
	/// Evaluates a value
	/// 
	Eval(RValue),
}

pub struct Instruction {
	kind: InstructionKind
}

impl Instruction {
	///
	/// 
	/// 
	pub fn assign(place: Place, value: RValue) -> Self {
		Self { kind: InstructionKind::Assign(place, value) }
	}

	///
	/// 
	/// 
	pub fn drop(place: Place) -> Self {
		Self { kind: InstructionKind::Drop(place) }
	}

	///
	/// 
	/// 
	pub fn eval(value: RValue) -> Self {
		Self { kind: InstructionKind::Eval(value) }
	}

	///
	/// 
	/// 
	pub fn kind(&self) -> &InstructionKind {
		&self.kind
	}
}

impl Display for InstructionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			InstructionKind::Assign(place, value) => write!(f, "{place} = {value}"),
			InstructionKind::Drop(place) => write!(f, "drop {place}"),
			InstructionKind::Eval(value) => write!(f, "{value}"),
		}
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}