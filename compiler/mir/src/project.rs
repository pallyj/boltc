use std::{fmt::Display, collections::HashMap};

use crate::{code::{BasicBlock, BasicBlockId, FunctionId, Function}, ty::{Type, Struct, StructId}, Builder, exc::ExecutionEngine};

///
/// A `Project` encapsulates an entire Bolt projects
/// 
/// It consists of the functions and types defined in the code
/// 
pub struct Project {
	name: String,

	basic_blocks: Vec<BasicBlock>,
	functions: Vec<Function>,
	structs: Vec<Struct>,

	function_names: HashMap<String, FunctionId>,
}

impl Project {
	///
	/// Creates a new project with the specified name
	/// 
	/// The name should be the name of the project being built
	/// 
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),

			basic_blocks: vec![],
			functions: vec![],
			structs: vec![],

			function_names: HashMap::new(),
		}
	}

	///
	/// The name of the project
	/// 
	pub fn name(&self) -> &str {
		&self.name
	}

	/// 
	/// Adds a function to the project
	/// 
	/// 
	pub (crate) fn add_function(&mut self, name: &str, params: Vec<Type>, return_type: Type) -> FunctionId {
		let function_id = FunctionId::new(self.functions.len());
		let function = Function::new(function_id, name, params, return_type);

		self.functions.push(function);
		self.function_names.insert(name.to_string(), function_id);
		function_id
	}

	///
	/// Gets a function by its name
	/// 
	pub (crate) fn get_function_named(&self, name: &str) -> Option<&Function> {
		let function_id = self.function_names.get(name)?;

		self.function(*function_id)
	}

	///
	/// 
	/// 
	pub (crate) fn add_struct(&mut self, name: &str, fields: Vec<(String, Type)>, is_transparent: bool, is_packed: bool) -> StructId {
		let struct_id = StructId::new(self.structs.len());
		let struct_val = Struct::new(struct_id, name.to_string(), fields, is_transparent, is_packed);

		self.structs.push(struct_val);
		struct_id
	}

	///
	/// Gets the next basic block unique id
	/// 
	pub (crate) fn next_basic_block(&self) -> usize {
		self.basic_blocks.len()
	}

	///
	/// Adds a basic block to the project
	/// 
	pub (crate) fn add_basic_block(&mut self, block: BasicBlock) -> BasicBlockId {
		let id = block.id();

		self.basic_blocks.push(block);

		id
	}

	///
	/// Gets the basic block at an index
	/// 
	pub (crate) fn basic_block(&self, block_id: BasicBlockId) -> Option<&BasicBlock> {
		self.basic_blocks.get(block_id.unique_idx())
	}

	///
	/// Gets the basic block at an index
	/// 
	pub (crate) fn basic_block_mut(&mut self, block_id: BasicBlockId) -> Option<&mut BasicBlock> {
		self.basic_blocks.get_mut(block_id.unique_idx())
	}

	///
	/// Gets the functions at an index
	/// 
	pub (crate) fn function(&self, func_id: FunctionId) -> Option<&Function> {
		self.functions.get(func_id.unique_idx())
	}

	///
	/// Gets the functions at an index
	/// 
	pub (crate) fn function_mut(&mut self, func_id: FunctionId) -> Option<&mut Function> {
		self.functions.get_mut(func_id.unique_idx())
	}

	///
	/// Gets a struct 
	/// 
	pub (crate) fn get_struct(&self, struct_id: StructId) -> Option<&Struct> {
		self.structs.get(struct_id.unique_idx())
	}

	///
	/// Gets a builder for this project
	/// The project can't be accessed while the builder is running
	/// 
	pub fn builder<'a>(&'a mut self) -> Builder<'a> {
		Builder::new(self)
	} 

	///
	/// 
	/// 
	pub fn execute<'a>(&'a self) -> ExecutionEngine<'a> {
		ExecutionEngine::new(self)
	}
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for struct_val in &self.structs {
			struct_val.write(f, self)?;
		}

        for function in &self.functions {
			function.write(f, self)?;
		}

		Ok(())
    }
}