use std::{ops::Deref, cell::{RefCell, Ref}, sync::{Arc}, fmt::Display};

use crate::value::Instruction;

use super::{FunctionRef, FunctionWeakRef};

#[derive(Clone)]
pub struct BlockRef {
	block: Arc<Block>
}

impl Deref for BlockRef {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl Display for BlockRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.block)
    }
}

pub struct Block {
	label: String,
	idx: u64,

	instructions: RefCell<Vec<Instruction>>,

	func: FunctionWeakRef,
}

impl Block {
	pub fn new(label: String, idx: u64, func: &FunctionRef) -> BlockRef {
		BlockRef {
			block: Arc::new(Self {
				label,
				idx,
				instructions: RefCell::new(Vec::new()),
				func: func.downgrade(),
			})
		}
	}

	pub fn insert_instruction(&self, head: usize, instruction: Instruction) {
		self.instructions
			.borrow_mut()
			.insert(head, instruction)
	}

	pub fn instructions(&self) -> Ref<Vec<Instruction>> {
		self.instructions.borrow()
	}

	pub fn function(&self) -> &FunctionWeakRef {
		&self.func
	}

	pub fn label(&self) -> &String {
		&self.label
	}

	pub fn index(&self) -> u64 {
		self.idx
	}
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.label)?;

		for i in self.instructions().iter() {
			writeln!(f, "    {i}")?;
		}

		Ok(())
    }
}