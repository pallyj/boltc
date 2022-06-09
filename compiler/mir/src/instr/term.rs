use std::fmt::Display;

use crate::{val::RValue, code::BasicBlockId};

/// 
/// A branch of a switch match
/// 
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SwitchArm {
	pub match_value: u64,
	pub arm_block: BasicBlockId,
}

///
/// The kind of a `Terminator`
/// 
/// goto - Jump to another basic block
/// branchif - Jump to one of two basic blocks depending on a condition
/// switch - Jump to one of many basic blocks depending on a value
/// return - Returns void
/// return `value` - Returns a value
/// panic - Exits the process
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum TerminatorKind {
	/// 
	/// Jump to another basic block
	/// 
	Goto(BasicBlockId),

	///
	/// Jump to `positive` if `condition` is true,
	/// `negative` if not
	/// 
	BranchIf {
		condition: RValue,
		positive: BasicBlockId,
		negative: BasicBlockId,
	},

	///
	/// Switch over `scrutinee`, if one of the arms matches it, then jump
	/// to that basic block, otherwise jump to `default`
	/// 
	Switch {
		scrutinee: RValue,
		arms: Vec<SwitchArm>,
		default: BasicBlockId
	},

	///
	/// Exit the current function without returning a value
	/// 
	ReturnVoid,

	/// 
	/// Exits the current function and returns a value
	/// 
	Return { value: RValue },

	///
	/// Exits the current applicatioon
	/// 
	Panic,
}

pub struct Terminator {
	kind: TerminatorKind
}

impl Terminator {

	/// 
	/// Jump to another basic block
	///
	pub fn goto(id: BasicBlockId) -> Self {
		Terminator { kind: TerminatorKind::Goto(id) }
	} 

	///
	/// Jump to `positive` if `condition` is true,
	/// `negative` if not
	///
	pub fn branch_if(condition: RValue, positive: BasicBlockId, negative: BasicBlockId) -> Self {
		Terminator { kind: TerminatorKind::BranchIf { condition, positive, negative } }
	}

	///
	/// Switch over `scrutinee`, if one of the arms matches it, then jump
	/// to that basic block, otherwise jump to `default`
	/// 
	pub fn switch(scrutinee: RValue, arms: Vec<SwitchArm>, default: BasicBlockId) -> Self {
		Terminator { kind: TerminatorKind::Switch { scrutinee, arms, default } }
	}

	///
	/// Exit the current function without returning a value
	/// 
	pub fn return_void() -> Self {
		Terminator { kind: TerminatorKind::ReturnVoid }
	}

	///
	/// Exits the current function and returns a value
	/// 
	pub fn returns(value: RValue) -> Self {
		Terminator { kind: TerminatorKind::Return { value } }
	}


	///
	/// Exits the current application
	/// 
	pub fn panic() -> Self {
		Terminator { kind: TerminatorKind::Panic }
	}


	///
	/// What kind of terminator this is
	/// 
	pub fn kind(&self) -> &TerminatorKind {
		&self.kind
	}
}

impl Display for TerminatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminatorKind::Goto(basic_block) => write!(f, "goto {basic_block}"),
            TerminatorKind::BranchIf { condition, positive, negative } => write!(f, "branchif {condition} {positive}; else {negative}"),
			TerminatorKind::Switch { scrutinee, arms, default } => {
				writeln!(f, "switch {scrutinee} else {default} {{")?;
				for branch in arms {
					writeln!(f, "\t\t{value} => {branch}", value = branch.match_value, branch = branch.arm_block)?;
				}
				write!(f, "\t}}")
			}
            TerminatorKind::ReturnVoid => write!(f, "return"),
            TerminatorKind::Return { value } => write!(f, "return {value}"),
            TerminatorKind::Panic => write!(f, "panic"),
        }
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}