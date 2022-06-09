use errors::Span;

use crate::{project::Project, code::{BasicBlockId, BasicBlock, FunctionId, Function, ExternFunctionId}, instr::{Terminator, Instruction}, ty::{Type, StructId, TypeKind, EnumId}, val::{Place, RValue, PlaceKind}};

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
	/// Adds a function to the underlying project, returning its `FunctionId`
	/// 
	pub fn add_extern_function(&mut self, name: &str, params: Vec<Type>, return_type: Type) -> ExternFunctionId {
		self.project.add_extern_function(name, params, return_type)
	}

	///
	/// 
	/// 
	pub fn get_function_id(&self, name: &str) -> FunctionId {
		self.project.get_function_id(name).expect("Function doesn't exist")
	}

	///
	/// 
	/// 
	pub fn get_function_by_id(&self, id: FunctionId) -> &Function {
		self.project.function(id).expect("Function doesn't exist")
	}

	///
	/// Adds a struct to the underlying project, returning its `StructId`
	/// 
	pub fn add_struct(&mut self, name: &str, is_transparent: bool, is_packed: bool) -> StructId {
		self.project.add_struct(name, is_transparent, is_packed)
	}

	///
	/// 
	/// 
	pub fn add_struct_fields(&mut self, name: &str, fields: Vec<(String, Type)>) {
		self.project.get_struct_mut_named(name).unwrap()
			.insert_fields(fields);
	}

	///
	/// 
	/// 
	pub fn get_struct_id(&self, name: &str) -> StructId {
		self.project.get_struct_id(name).expect("struct name doesn't exist")
	}

	///
	/// Adds a struct to the underlying project, returning its `StructId`
	/// 
	pub fn add_enum(&mut self, name: &str, repr_type: Type) -> EnumId {
		self.project.add_enum(name, repr_type)
	}

	///
	/// 
	/// 
	pub fn add_enum_variants(&mut self, name: &str, variants: Vec<(u64, Type)>) {
		self.project.get_enum_mut_named(name).unwrap()
			.insert_variants(variants)
	}

	///
	/// 
	/// 
	pub fn get_enum_id(&self, name: &str) -> EnumId {
		self.project.get_enum_id(name).expect("enum name doesn't exist")
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
		let block_id = BasicBlockId::new(self.project.next_basic_block(), self.current_function_mut().next_basic_block(), self.current_function.unwrap());

		let block = BasicBlock::new(block_id);
		self.project.add_basic_block(block);

		self.current_function_mut().append_block_id(block_id);

		block_id
	}

	///
	/// Build a local
	/// 
	pub fn build_local(&mut self, ty: Type, is_mutable: bool, span: Span) -> Place {
		let local_id = self.current_function_mut().add_local(ty.clone());

		Place::new(PlaceKind::Local(local_id), ty, is_mutable, span)
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
	pub fn build_function(&mut self, function_name: &str, span: Span) -> RValue {
		let function = self.project.get_function_named(function_name).expect("function doesn't exist");

		RValue::function(&function, span)
	}

	///
	/// 
	/// 
	pub fn build_extern_function(&mut self, function_name: &str, span: Span) -> RValue {
		let function = self.project.get_extern_function_named(function_name).expect("function doesn't exist");

		RValue::extern_function(&function, span)
	}

	///
	/// 
	/// 
	pub fn build_field(&mut self, struct_val: &Place, field_name: &str, span: Span) -> Place {
		if let TypeKind::Struct { id } = struct_val.ty().kind() {
			let struct_def = self.project.get_struct(*id).expect("");
			let field_ty = struct_def.field_type(field_name).cloned().expect("");
			struct_val.field(field_name, field_ty, span)
		} else {
			panic!()
		}
	}

	///
	/// 
	/// 
	pub fn build_cast_variant(&mut self, enum_val: &Place, tag: u64, variant_name: &str, span: Span) -> Place {
		let TypeKind::Enum { id } = enum_val.ty().kind() else {
			panic!()
		};

		let enum_def = self.project.get_enum(*id).expect("");
		let cast_ty = enum_def.get_variant_type(tag).expect("Variant doesn't exist");

		enum_val.cast_variant(tag, variant_name, cast_ty, span)
	}

	///
	/// 
	/// 
	pub fn build_discriminant(&mut self, enum_val: &Place, span: Span) -> Place {
		let TypeKind::Enum { id } = enum_val.ty().kind() else {
			panic!()
		};

		let enum_def = self.project.get_enum(*id).expect("");
		let cast_ty = enum_def.tag_type();

		enum_val.discriminant(cast_ty, span)
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
	pub fn current_function_mut(&mut self) -> &mut Function {
		let current_func = self.current_function.expect("no function is set");
		self.project.function_mut(current_func).expect("function out of range")
	}

	pub fn current_function(&self) -> &Function {
		let current_func = self.current_function.expect("no function is set");
		self.project.function(current_func).expect("function out of range")
	}
}