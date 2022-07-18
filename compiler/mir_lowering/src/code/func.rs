use inkwell::{module::Linkage, values::PointerValue, basic_block::BasicBlock};
use itertools::Itertools;
use mir::{instr::{Terminator, TerminatorKind, LocalId, Instruction}, code::BasicBlockId, ty::TypeKind};

use crate::{MirLowerContext, code::func};

impl<'a, 'ctx> MirLowerContext<'a, 'ctx>
{
	pub fn create_extern_function(&self, func: &mir::code::ExternFunction)
	{
		let return_type = func.return_type().clone();
		let params = func.params().clone();
		let llvm_function_type = self.lower_func_ty(return_type, params);

		self.module.add_function(func.name(), llvm_function_type, Some(Linkage::External));
	}

	pub fn create_function(&self, func: &mir::code::Function)
	{
		let return_type = func.return_type().clone();
		let params = func.params().clone();
		let llvm_function_type = self.lower_func_ty(return_type, params);

		self.module.add_function(func.name(), llvm_function_type, None);
	}

	pub fn lower_function_code(&self, func: &mir::code::Function)
	{
		let llvm_function = self.module.get_function(func.name()).unwrap();

		// Create locals for the function
		let local_builder = self.context.append_basic_block(llvm_function, "bb");
		self.builder.position_at_end(local_builder);

		if func.is_entry_point()
		{
			if let Some(init) = self.module.get_function(".init")
			{
				self.builder.build_call(init, &[], "cv");
			}
		}

		let params = func.params()
						 .iter()
						 .enumerate()
						 .map(|(i, param)| {
							let llvm_param_ty = self.lower_ty(param.clone());
							let alloca = self.builder.build_alloca(llvm_param_ty, &format!("_{i}"));
							let param_value = llvm_function.get_nth_param(i as u32).unwrap();
							self.builder.build_store(alloca, param_value);
							alloca
						 });

		let locals = func.locals()
						 .iter()
						 .map(|local| {
							let local_ty = local.ty().clone();
							let llvm_local_ty = self.lower_ty(local_ty);
							self.builder.build_alloca(llvm_local_ty, &format!("{}", local.id()))
						 });

		let locals = params.chain(locals)
						   .collect_vec();

		// Create the basic blocks for the function first, so later blocks can be referenced
		let blocks = func.basic_blocks()
						 .iter()
						 .cloned()
						 .map(|bb_id| {
							self.context.append_basic_block(llvm_function, &format!("{bb_id}"))
						 })
						 .collect_vec();

		let function_context = FunctionContext::new(&locals, &blocks);

		if blocks.is_empty() {
			//self.builder.build_return(None);
			return
		}

		self.builder.build_unconditional_branch(*blocks.first().unwrap());

		// Now, we can start lowering each basic block
		for bb_id in func.basic_blocks() {
			let basic_block = self.project.basic_block(*bb_id).unwrap();

			// Attach the builder to the correct block
			self.builder.position_at_end(blocks[basic_block.id().local_idx()]);

			// Build each instruction
			for instruction in basic_block.instructions()
			{
				self.lower_instruction(instruction, function_context);
			}

			// Build the terminator
			self.lower_terminator(basic_block.terminator(), function_context);

		}
	}

	fn lower_terminator(
		&self,
		terminator: &Terminator,
		function: FunctionContext<'a, 'ctx>)
	{
		use TerminatorKind::*;

		match terminator.kind() {
			Goto(bb) => {
				let basic_block = function.get_basic_block(*bb);

				self.builder.build_unconditional_branch(basic_block);
			},
			BranchIf { condition, positive, negative } => {
				let llvm_value = self.lower_rvalue(condition, function);
				if !llvm_value.is_int_value() {
					panic!("compiler error: branched on non-integer value");
				}
				let llvm_condition = llvm_value.into_int_value();

				let llvm_positive = function.get_basic_block(*positive);
				let llvm_negative = function.get_basic_block(*negative);

				self.builder.build_conditional_branch(llvm_condition, llvm_positive, llvm_negative);
			},
			Switch { scrutinee, arms, default } => {
				let llvm_value = self.lower_rvalue(scrutinee, function);
				if !llvm_value.is_int_value() {
					panic!("compiler error: branched on non-integer value");
				}
				let llvm_scrutinee = llvm_value.into_int_value();

				let llvm_ty = llvm_value.get_type().into_int_type();

				let llvm_default = function.get_basic_block(*default);

				let cases = arms.iter()
								.map(|case| {
									let llvm_value = llvm_ty.const_int(case.match_value, false);
									let block = function.get_basic_block(case.arm_block);

									(llvm_value, block)
								})
								.collect_vec();

				self.builder.build_switch(llvm_scrutinee, llvm_default, &cases);
			},
			ReturnVoid => { self.builder.build_return(None); },
			Return { value } => {
				match value.ty().kind()
				{
					TypeKind::Tuple(items) if items.is_empty() => {
						self.builder.build_return(None);
					}
					_ => {
						let val = self.lower_rvalue(value, function);

						self.builder.build_return(Some(&val));
					}
				}
			}
			Panic => {
				self.builder.build_unreachable();
			}
		};
	}

	fn lower_instruction(
		&self,
		instruction: &Instruction,
		function: FunctionContext<'a, 'ctx>)
	{
		match instruction.kind()
		{
			mir::instr::InstructionKind::Assign(place, value) => {
				let place = self.lower_place(place, function);
				let value = self.lower_rvalue(value, function);

				self.builder.build_store(place, value);
			},
			mir::instr::InstructionKind::Drop(_) => {},
			mir::instr::InstructionKind::Eval(rvalue) => {
				self.lower_rvalue(rvalue, function);
			}
		}
	}
}

#[derive(Copy, Clone)]
pub struct FunctionContext<'a, 'ctx> {
	locals: &'a Vec<PointerValue<'ctx>>,
	blocks: &'a Vec<BasicBlock<'ctx>>,
}

impl<'a, 'ctx> FunctionContext<'a, 'ctx> {
	pub fn new(locals: &'a Vec<PointerValue<'ctx>>, blocks: &'a Vec<BasicBlock<'ctx>>) -> Self {
		Self {
			locals,
			blocks
		}
	}

	pub fn get_local(&self, local_id: LocalId) -> PointerValue<'ctx>
	{
		self.locals[local_id.local_idx()]
	}

	pub fn get_basic_block(&self, basic_block_id: BasicBlockId) -> BasicBlock<'ctx>
	{
		self.blocks[basic_block_id.local_idx()]
	}
}