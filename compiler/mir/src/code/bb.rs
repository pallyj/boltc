use std::fmt::Display;

use crate::instr::{Terminator, Instruction};

use super::func::FunctionId;

///
/// A `BasicBlockId` is a way to refer to a `BasicBlock` in a project
/// 
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BasicBlockId {
	unique_idx: usize,
	local_idx: usize,
	function_id: FunctionId,
}

///
/// A basic block is a container for instructions
/// A `BasicBlock` is composed of any number of instructions,
/// and a terminator. The lack of a terminator or two terminators
/// will cause a panic.
/// 
pub struct BasicBlock {
	id: BasicBlockId,
	instructions: Vec<Instruction>,
	terminator: Option<Terminator>
}

impl BasicBlockId {
	///
	/// Internal Use Only
	/// Creates a new BasicBlockId
	/// 
	pub (crate) fn new(unique_idx: usize, local_idx: usize, function_id: FunctionId) -> BasicBlockId {
		BasicBlockId {
			unique_idx,
			local_idx,
			function_id
		}
	}

	///
	/// The basic block's index in the project
	/// 
	pub fn unique_idx(&self) -> usize {
		self.unique_idx
	}

	///
	/// The basic block's index in the function
	/// 
	pub fn local_idx(&self) -> usize {
		self.local_idx
	}

	/// 
	/// The function containing this basic block
	/// 
	pub fn function_id(&self) -> FunctionId {
		self.function_id
	}
}

impl BasicBlock {
	///
	/// Internal Use Only
	/// Creates a new basic block
	/// 
	pub (crate) fn new(id: BasicBlockId) -> BasicBlock {
		BasicBlock {
			id,
			instructions: vec![],
			terminator: None
		}
	}

	///
	/// Gets an id for this BasicBlock
	/// 
	pub fn id(&self) -> BasicBlockId {
		self.id
	}


	/// 
	/// Adds a new terminator to the basic block
	/// If the block already has one, panic
	/// 
	pub fn insert_terminator(&mut self, terminator: Terminator) {
		if self.terminator.is_some() {
			panic!("{} is already terminated with {}, {} is unnecessary", self.id(), self.terminator.as_ref().unwrap(), terminator);
		}

		self.terminator = Some(terminator);
	}

	/// 
	/// Adds a new instruction to the basic block
	/// 
	pub fn insert_instruction(&mut self, instruction: Instruction) {
		self.instructions.push(instruction);
	}

	///
	/// Gets a list of the instructions in this BasicBLock
	/// 
	pub fn instructions(&self) -> &[Instruction] {
		&self.instructions
	}

	/// Gets the terminator in the basic block
	pub fn terminator(&self) -> &Terminator {
		self.terminator.as_ref().unwrap()
	}
}



impl Display for BasicBlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bb{}", self.local_idx)
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} : {{", self.id)?;

		// Write each statement with an indent of 1
		for smt in &self.instructions {
			writeln!(f, "\t{smt}")?;
		}

		if let Some(terminator) = &self.terminator {
			writeln!(f, "\t{terminator}")?;
		}

		writeln!(f, "}}")
    }
}