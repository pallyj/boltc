use crate::{ty::Type, Project, instr::{LocalId, Local}};

use super::{BasicBlockId};

///
/// Refers to a function in a project
/// 
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionId {
	id: usize,
}

///
/// Represents a function, with a name, parameters, a return_type and a collection of `BasicBlocks`
/// 
pub struct Function {
	id: FunctionId,

	name: String,

	params: Vec<Type>,
	return_type: Type,

	basic_blocks: Vec<BasicBlockId>,

	locals: Vec<Local>,

	n_locals: usize,
}

impl Function {
	///
	/// Creates a new function
	/// 
	pub (crate) fn new(id: FunctionId, name: &str, params: Vec<Type>, return_type: Type) -> Function {
		let n_locals = params.len();
		Function {
			id,
			name: name.to_string(),
			params,
			return_type,
			basic_blocks: vec![],
			locals: vec![],
			n_locals }
	}

	///
	/// The unique identifier of a function
	/// 
	pub fn id(&self) -> FunctionId {
		self.id
	}

	///
	/// Gets the unique, mangled name of a function
	/// 
	pub fn name(&self) -> &str {
		&self.name
	}

	///
	/// Gets the locals in the function
	/// 
	pub fn locals(&self) -> &[Local] {
		&self.locals
	}

	///
	/// The parameters a function takes
	/// 
	pub fn params(&self) -> &[Type] {
		&self.params
	}

	///
	/// The type a function returns
	/// 
	pub fn return_type(&self) -> &Type {
		&self.return_type
	}

	///
	/// 
	/// 
	pub fn func_type(&self) -> Type {
		self.return_type.clone().func(self.params.clone())
	}

	///
	/// Adds a block to the function
	/// 
	pub (crate) fn append_block_id(&mut self, block_id: BasicBlockId) {
		self.basic_blocks.push(block_id);
	}

	///
	/// The index of the next `BasicBlock` to be added
	/// 
	pub (crate) fn next_basic_block(&self) -> usize {
		self.basic_blocks.len()
	}

	///
	/// Returns the index of the next local id, incrementing it
	/// 
	pub (crate) fn add_local(&mut self, ty: Type) -> LocalId {
		let local_id = LocalId::new(self.n_locals);

		self.locals.push(Local::new(local_id, ty));
		self.n_locals += 1;

		return local_id
	}

	///
	/// 
	/// 
	pub (crate) fn first_basic_block(&self) -> BasicBlockId {
		self.basic_blocks.first().cloned().expect("function is empty")
	}

	///
	/// Writes a function to a Formatter
	/// 
	/// Takes a project parameter for access to the basic blocks in the function, necessitating
	/// the use of a custom function instead of Display
	/// 
	#[allow(unstable_name_collisions)]
	pub (crate) fn write(&self, f: &mut std::fmt::Formatter, project: &Project) -> std::fmt::Result {
		write!(f, "func {} (", self.name())?;

		self.params().iter()
					 .enumerate()
					 .map(|(i, item)| { write!(f, "_{i}: ")?; item.write(f, project)?; write!(f, ", ")})	
					 .collect::<std::fmt::Result>()?;

		write!(f, ") -> ")?; 

		self.return_type.write(f, project)?;

		writeln!(f, " {{")?;

		for local in &self.locals {
			write!(f, "\t")?;
			local.write(f, project)?;
			writeln!(f)?;
		}

		if !self.locals.is_empty() {
			writeln!(f)?;
		}

		for bb_id in &self.basic_blocks {
			let bb = project.basic_block(*bb_id).unwrap();

			writeln!(f, "\t{}", bb.to_string().replace("\n", "\n\t"))?;
		}

		writeln!(f, "}}")
	}
}

impl FunctionId {
	///
	/// Creates a new `FunctionId`
	/// 
	pub (crate) fn new(id: usize) -> Self {
		FunctionId { id }
	}

	///
	/// The unique identifier of the `FunctionId`
	/// 
	pub (crate) fn unique_idx(&self) -> usize {
		self.id
	}
}