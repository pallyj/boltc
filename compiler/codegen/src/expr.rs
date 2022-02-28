use blir::{Expr, ExprKind};
use inkwell::{values::{BasicValueEnum, BasicMetadataValueEnum, IntValue}, IntPredicate};

use crate::{context::{LibraryGenContext, FuncGenContext}, typ::generate_type, smt::generate_smt};

pub fn generate_expr<'a, 'ctx>(expr: &Expr, context: &FuncGenContext<'a, 'ctx>) -> BasicValueEnum<'ctx> {
	match expr.kind() {
		ExprKind::IntLiteral(value) => {
			BasicValueEnum::IntValue(
				generate_type(expr.typ_ref(), context.library())
					.unwrap()
					.into_int_type()
					.const_int(*value, false)
			)
		}

		ExprKind::FuncCall { func, args } => {
			let args = args
				.iter()
				.map(|arg| arg.value())
				.map(|arg| generate_expr(arg, context).into())
				.collect::<Vec<_>>();
			
			generate_func_call(&func, &args, context)
		}

		ExprKind::FunctionParameter(n) => {
			context.function().get_nth_param(*n as u32).unwrap()
		}

		ExprKind::LocalVariable(v) => {
			context.get_var(v).unwrap()
		}

		ExprKind::Select { branches, finally } => {
			let select_blocks = (0..(branches.len() - 1))
				.map(|br| context.context().append_basic_block(context.function().clone(), &format!("select{br}")))
				.collect::<Vec<_>>();

			let branch_blocks = branches
				.iter()
				.map(|br| context.context().append_basic_block(context.function().clone(), "then"))
				.collect::<Vec<_>>();

			for i in 0..(branches.len() - 1) {
				let positive_block = branch_blocks[i];
				let negative_block = select_blocks[i];

				let cond = generate_expr(branches[i].condition(), context);

				context.builder().build_conditional_branch(cond.into_int_value(), positive_block, negative_block);

				context.builder().position_at_end(negative_block);
			}

			let continue_block = if let Some(finally) = finally {
				let positive_block = branch_blocks.last().cloned().unwrap();
				let negative_block = context.context().append_basic_block(context.function().clone(), "finally");

				let cond = generate_expr(branches.last().unwrap().condition(), context);

				context.builder().build_conditional_branch(cond.into_int_value(), positive_block, negative_block);

				context.builder().position_at_end(negative_block);

				for smt in finally.statements() {
					generate_smt(&smt.0, context);
				}

				let continue_block = context.context().append_basic_block(context.function().clone(), "continue");

				continue_block
			} else {
				let positive_block = branch_blocks.last().cloned().unwrap();
				let negative_block = context.context().append_basic_block(context.function().clone(), "continue");

				let cond = generate_expr(branches.last().unwrap().condition(), context);

				context.builder().build_conditional_branch(cond.into_int_value(), positive_block, negative_block);

				negative_block
			};

			for branch in branches.into_iter().zip(branch_blocks.into_iter()) {
				context.builder().position_at_end(branch.1);

				for smt in branch.0.code().statements() {
					generate_smt(&smt.0, context);
				}

				context.builder().build_unconditional_branch(continue_block);
			}

			context.builder().position_at_end(continue_block);

			BasicValueEnum::IntValue(context.context().i64_type().const_int(0, true))
		}

		_ => panic!(),
	}
}

fn generate_func_call<'a, 'ctx>(func: &Expr, args: &Vec<BasicMetadataValueEnum<'ctx>>, context: &FuncGenContext<'a, 'ctx>) -> BasicValueEnum<'ctx> {
	match func.kind() {
		ExprKind::IntrinsicFunc(name) => {
			let a = args[0].into_int_value();
			let b = args[1].into_int_value();

			match name.as_str() {
				"integer8Add" | "integer16Add" | "integer32Add" | "integer64Add" => BasicValueEnum::IntValue(context.builder().build_int_add(a, b, "sum")),
				"integer8Sub" | "integer16Sub" | "integer32Sub" | "integer64Sub" => BasicValueEnum::IntValue(context.builder().build_int_sub(a, b, "diff")),
				"integer8Mul" | "integer16Mul" | "integer32Mul" | "integer64Mul" => BasicValueEnum::IntValue(context.builder().build_int_mul(a, b, "mul")),
				"integer8CmpEq" | "integer16CmpEq" | "integer32CmpEq" | "integer64CmpEq" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::EQ, a, b, "cmp_eq")),
				_ => panic!(),
			}
		}

		ExprKind::Function(f) => {
			let func = context.module().get_function(f.name()).unwrap();

			context.builder().build_call(func, args, "call").try_as_basic_value().unwrap_left()
		}

		_ => {
			println!("{}", func);
			panic!()
		},
	}
}