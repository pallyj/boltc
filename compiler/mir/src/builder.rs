use crate::{project::Project, code::{BasicBlockId, BasicBlock, FunctionId, Function}, instr::{Terminator, Instruction}, ty::{Type, StructId, TypeKind}, val::{Place, RValue, PlaceKind}};

///
/// A helper struct to build MIR code
/// 
pub struct Builder<'a> {
	project: &'a mut Project,

	current_basic_block: Option<BasicBlockId>,
	current_function: Option<FunctionId>,
}

impl<'a> Builder<'a> {
	/// 
	/// Creates a new builder from a reference to a project
	/// 
	pub (crate) fn new(project: &'a mut Project) -> Self {
		Self { project,
			   current_basic_block: None,
			   current_function: None }
	}

	///
	/// Adds a function to the underlying project, returning its `FunctionId`
	/// 
	pub fn add_function(&mut self, name: &str, params: Vec<Type>, return_type: Type) -> FunctionId {
		self.project.add_function(name, params, return_type)
	}

	///
	/// Adds a struct to the underlying project, returning its `StructId`
	/// 
	pub fn add_struct(&mut self, name: &str, fields: Vec<(String, Type)>, is_transparent: bool, is_packed: bool) -> StructId {
		self.project.add_struct(name, fields, is_transparent, is_packed)
	}

	///
	/// Positions the builder on the specified function
	/// 
	pub fn position_on_func(&mut self, func: FunctionId) {
		self.current_function = Some(func);
	}

	///
	/// Positions the builder's head at the end of the specified basic_block
	/// 
	pub fn position_at_end(&mut self, basic_block: BasicBlockId) {
		assert_eq!(basic_block.function_id(), self.current_function.expect("position the builder on a func first"));
		self.current_basic_block = Some(basic_block)
	}

	///
	/// Appends a block to the current function
	/// 
	pub fn append_block(&mut self) -> BasicBlockId {
		let block_id = BasicBlockId::new(self.project.next_basic_block(), self.current_function().next_basic_block(), self.current_function.unwrap());

		let block = BasicBlock::new(block_id);
		self.project.add_basic_block(block);

		self.current_function().append_block_id(block_id);

		block_id
	}

	///
	/// Build a local
	/// 
	pub fn build_local(&mut self, ty: Type) -> Place {
		let local_id = self.current_function().add_local(ty.clone());

		Place::new(PlaceKind::Local(local_id), ty)
	}

	///
	/// Adds a terminator to the current basic block
	/// If it already has a terminator, panic
	/// 
	pub fn build_terminator(&mut self, terminator: Terminator) {
		self.current_block()
			.insert_terminator(terminator)
	}

	///
	/// Builds an assign instruction
	/// 
	pub fn build_assign(&mut self, place: &Place, value: RValue) {
		let assign_instruction = Instruction::assign(place.clone(), value);

		self.current_block()
			.insert_instruction(assign_instruction)
	}

	///
	/// 
	/// 
	pub fn build_drop(&mut self, place: &Place) {
		let drop_instruction = Instruction::drop(place.clone());

		self.current_block()
			.insert_instruction(drop_instruction)
	}

	///
	/// 
	/// 
	pub fn build_eval(&mut self, value: RValue) {
		let eval_instruction = Instruction::eval(value);

		self.current_block()
			.insert_instruction(eval_instruction)
	}

	///
	/// 
	/// 
	pub fn build_function(&mut self, function_name: &str) -> RValue {
		let function = self.project.get_function_named(function_name).expect("function doesn't exist");

		RValue::function(&function)
	}

	///
	/// 
	/// 
	pub fn build_field(&mut self, struct_val: &Place, field_name: &str) -> Place {
		if let TypeKind::Struct { id } = struct_val.ty().kind() {
			let struct_def = self.project.get_struct(*id).expect("");
			let field_ty = struct_def.field_type(field_name).cloned().expect("");
			struct_val.field(field_name, field_ty)
		} else {
			panic!()
		}
	}
}

impl<'a> Builder<'a> {
	///
	/// Internal use only
	/// Gets the current block to push an instruction to it
	/// 
	fn current_block(&mut self) -> &mut BasicBlock {
		let current_bb = self.current_basic_block.expect("no basic block is set");
		self.project.basic_block_mut(current_bb).expect("basic block out of range")
	}

	///
	/// Internal use only
	/// Gets the current function
	/// 
	fn current_function(&mut self) -> &mut Function {
		let current_func = self.current_function.expect("no function is set");
		self.project.function_mut(current_func).expect("function out of range")
	}
}