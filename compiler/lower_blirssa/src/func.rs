use std::{collections::HashMap, cell::RefCell};

use blirssa::{code::{FunctionRef, BlockRef}, value::{Instruction, LabelValue}, typ::Type};
use inkwell::values::FunctionValue;

use crate::{ModuleContext, value::{lower_value, LLVMValue}};

pub fn lower_function<'a, 'ctx>(func: &FunctionRef, context: &ModuleContext<'a, 'ctx>) -> Result<(), String> {
	let llvm_func = context.module.get_function(func.name()).expect("Non-existant function");

	// Create a llvm basic block for every blirssa block,
	// associating a name with each one.
	// This step is necessary to allow for reference to later blocks.

	let mut basic_blocks_lookup = HashMap::new();
	let mut basic_blocks = Vec::new();

	for block in func.blocks().iter() {
		let basic_block = context.context.append_basic_block(llvm_func,block.label());

		basic_blocks_lookup.insert(block.label().clone(), basic_block);
		basic_blocks.push(basic_block);
	}

	// Create a function context
	let function_context = FunctionContext::new();

	// Define the functions arguments
	let Type::Function { pars, .. } = func.typ() else {
		panic!("Error: Function created with non function type");
	};

	for i in 0..(pars.len() as u64) {
		function_context.define_param(i, &llvm_func);
	}

	// Generate code for each block
	for (basic_block, blir_block) in basic_blocks.into_iter().zip(func.blocks().iter()) {
		context.builder.position_at_end(basic_block);

		lower_block(blir_block, &context, &function_context)?;
	}


	Ok(())
}

fn lower_block<'a, 'ctx>(blir_block: &BlockRef, context: &ModuleContext<'a, 'ctx>, fn_ctx: &FunctionContext<'ctx>) -> Result<(), String> {
	// Generate code for each instruction
	for instruction in blir_block.instructions().iter() {
		match instruction {
			Instruction::Assign { label, value } => {
				let llvm_value = lower_value(value, context, fn_ctx)?;

				fn_ctx.define_local(&label, llvm_value);
			}

			Instruction::AssignPtr { pointer, value } => {
				let pointer = fn_ctx.get_local(pointer).basic();
				let value = fn_ctx.get_local(value).basic();

				context.builder.build_store(pointer.into_pointer_value(), value);
			}

			Instruction::Return { value } => {
				if let Some(value) = value {
					// Get the value from the local context
					let llvm_value = fn_ctx.get_local(value).basic();

					context.builder.build_return(Some(&llvm_value));
				} else {
					// Return a void
					context.builder.build_return(None);
				}
			}
		}
	}


	Ok(())
}

pub struct FunctionContext<'ctx> {
	locals: RefCell<HashMap<u64, LLVMValue<'ctx>>>,
}

impl<'ctx> FunctionContext<'ctx> {
	pub fn new() -> Self {
		FunctionContext {
			locals: RefCell::new(HashMap::new())
		}
	}

	pub fn define_param(&self, n: u64, function: &FunctionValue<'ctx>) {
		let value = function.get_nth_param(n as u32).unwrap();

		self.locals
			.borrow_mut()
			.insert(n, LLVMValue::Basic(value));
	}

	pub fn define_local(&self, label: &LabelValue, value: LLVMValue<'ctx>) {
		self.locals
			.borrow_mut()
			.insert(label.label(), value);
	}

	pub fn get_local(&self, label: &LabelValue) -> LLVMValue<'ctx> {
		self.locals
			.borrow()
			.get(&label.label())
			.expect("Undefined label")
			.clone()
	}
}