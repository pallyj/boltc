use std::{cell::RefCell, collections::HashMap};

use blirssa::{code::{BlockRef, FunctionRef},
              typ::Type,
              value::{Instruction, LabelValue}};
use inkwell::{basic_block::BasicBlock, values::FunctionValue};

use crate::{value::{lower_value, LLVMValue},
            ModuleContext, typ::lower_integer_type};

pub fn lower_function<'a, 'ctx>(func: &FunctionRef, context: &ModuleContext<'a, 'ctx>) -> Result<(), String> {
    let llvm_func = context.module
                           .get_function(func.name())
                           .expect("Non-existant function");

    // Create a llvm basic block for every blirssa block,
    // associating a name with each one.
    // This step is necessary to allow for reference to later blocks.

    // Create a function context
    let function_context = FunctionContext::new();

    let mut basic_blocks = Vec::new();

    for block in func.blocks().iter() {
        let basic_block = context.context.append_basic_block(llvm_func, &block.label());

        function_context.add_basic_block(block.index(), basic_block);
        basic_blocks.push(basic_block);
    }

    // Define the functions arguments
    let Type::Function { pars, .. } = func.typ() else {
		panic!("Error: Function created with non function type");
	};

    let mut param_n = 0;
    for param in &pars {
        if let Type::Void = param {
            continue;
        }

        function_context.define_param(param_n, &llvm_func);
        param_n += 1;
    }

    // Generate code for each block
    for (basic_block, blir_block) in basic_blocks.into_iter().zip(func.blocks().iter()) {
        context.builder.position_at_end(basic_block);

        lower_block(blir_block, context, &function_context)?;
    }

    Ok(())
}

fn lower_block<'a, 'ctx>(blir_block: &BlockRef, context: &ModuleContext<'a, 'ctx>, fn_ctx: &FunctionContext<'ctx>) -> Result<(), String> {
    // Generate code for each instruction
    for instruction in blir_block.instructions().iter() {
        match instruction {
            Instruction::Assign { label, value } => {
                let llvm_value = lower_value(value, context, fn_ctx)?;

                fn_ctx.define_local(label, llvm_value);
            }

            Instruction::AssignPtr { pointer, value } => {
                let pointer = fn_ctx.get_local(pointer).unwrap().basic();
                let value = fn_ctx.get_local(value).unwrap().basic();

                context.builder
                       .build_store(pointer.into_pointer_value(), value);
            }

            Instruction::Branch { condition,
                                  positive,
                                  negative, } => {
                let condition = fn_ctx.get_local(condition)
                                      .unwrap()
                                      .basic()
                                      .into_int_value();

                let then_block = fn_ctx.get_basic_block(positive.index());
                let else_block = fn_ctx.get_basic_block(negative.index());

                context.builder
                       .build_conditional_branch(condition, then_block, else_block);
            }

            Instruction::AlwaysBranch { block } => {
                let block = fn_ctx.get_basic_block(block.index());

                context.builder.build_unconditional_branch(block);
            }

            Instruction::Return { value } => {
                if let Some(value) = value {
                    // Get the value from the local context
                    if let Some(llvm_value) = fn_ctx.get_local(value).and_then(|basic| basic.try_basic()) {
                        context.builder.build_return(Some(&llvm_value));
                    } else {
                        context.builder.build_return(None);
                    }
                } else {
                    // Return a void
                    context.builder.build_return(None);
                }
            }

            Instruction::SelectInteger { value, branches, default } => {
                let value = fn_ctx.get_local(value).unwrap().basic().into_int_value();

                let default_block = fn_ctx.get_basic_block(default.index());

                let cases = branches
                    .iter()
                    .map(|branch| {
                        let value = fn_ctx.get_local(&branch.0).unwrap().basic().into_int_value();
                        let cref = fn_ctx.get_basic_block(branch.1.index());

                        (value, cref)
                    })
                    .collect::<Vec<_>>();

                context.builder.build_switch(value, default_block, &cases);
            }

            Instruction::SelectEnumTag { value, branches, default } => {
                let Type::Enum(enum_ref) = value.typ() else {
                    panic!()
                };

                let value = fn_ctx.get_local(value).unwrap().basic().into_struct_value();
                let tag_value = context.builder.build_extract_value(value, 0, "enum-tag").unwrap().into_int_value();

                let default_block = fn_ctx.get_basic_block(default.index());

                let cases = branches
                    .iter()
                    .map(|branch| {
                        let block_ref = fn_ctx.get_basic_block(branch.1.index());

                        let enum_variant = enum_ref.get_variant(&branch.0);
                        let tag = enum_variant.tag();

                        let tag_value = lower_integer_type(enum_ref.bits() as u32, context)
                            .unwrap()
                            .const_int(tag as u64, false);

                        (tag_value, block_ref)
                    })
                    .collect::<Vec<_>>();

                context.builder.build_switch(tag_value, default_block, &cases);
            }
        }
    }

    Ok(())
}

pub struct FunctionContext<'ctx> {
    locals:       RefCell<HashMap<u64, LLVMValue<'ctx>>>,
    basic_blocks: RefCell<HashMap<u64, BasicBlock<'ctx>>>,
}

impl<'ctx> FunctionContext<'ctx> {
    pub fn new() -> Self {
        FunctionContext { locals:       RefCell::new(HashMap::new()),
                          basic_blocks: RefCell::new(HashMap::new()), }
    }

    pub fn define_param(&self, n: u64, function: &FunctionValue<'ctx>) {
        let Some(value) = function.get_nth_param(n as u32) else {
            panic!("Expected param {n} in function {:?}", function.get_name());
        };

        self.locals.borrow_mut().insert(n, LLVMValue::Basic(value));
    }

    pub fn define_local(&self, label: &LabelValue, value: LLVMValue<'ctx>) { self.locals.borrow_mut().insert(label.label(), value); }

    pub fn get_local(&self, label: &LabelValue) -> Option<LLVMValue<'ctx>> { self.locals.borrow().get(&label.label()).cloned() }

    pub fn add_basic_block(&self, n: u64, block: BasicBlock<'ctx>) { self.basic_blocks.borrow_mut().insert(n, block); }

    pub fn get_basic_block(&self, n: u64) -> BasicBlock { *self.basic_blocks.borrow().get(&n).unwrap() }
}
