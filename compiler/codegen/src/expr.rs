use blir::{Expr, ExprKind};
use inkwell::{values::{BasicValueEnum, BasicMetadataValueEnum}, IntPredicate};

use crate::{context::{FuncGenContext}, typ::generate_type, smt::generate_smt};

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
				.map(|_| context.context().append_basic_block(context.function().clone(), "then"))
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
			let b = args.get(1).map(|b| b.into_int_value());

			match name.as_str() {
				"integer8Add" | "integer16Add" | "integer32Add" | "integer64Add" => BasicValueEnum::IntValue(context.builder().build_int_add(a, b.unwrap(), "sum")),
				"integer8Sub" | "integer16Sub" | "integer32Sub" | "integer64Sub" => BasicValueEnum::IntValue(context.builder().build_int_sub(a, b.unwrap(), "diff")),
				"integer8Mul" | "integer16Mul" | "integer32Mul" | "integer64Mul" => BasicValueEnum::IntValue(context.builder().build_int_mul(a, b.unwrap(), "mul")),
				"integer8Div" | "integer16Div" | "integer32Div" | "integer64Div" => BasicValueEnum::IntValue(context.builder().build_int_unsigned_div(a, b.unwrap(), "div")),
				"integer8DivSig" | "integer16DivSig" | "integer32DivSig" | "integer64DivSig" => BasicValueEnum::IntValue(context.builder().build_int_signed_div(a, b.unwrap(), "div")),
				"integer8Rem" | "integer16Rem" | "integer32Rem" | "integer64Rem" => BasicValueEnum::IntValue(context.builder().build_int_unsigned_rem(a, b.unwrap(), "rem")),
				"integer8RemSig" | "integer16RemSig" | "integer32RemSig" | "integer64RemSig" => BasicValueEnum::IntValue(context.builder().build_int_unsigned_rem(a, b.unwrap(), "rem")),
				
				"integer8And" | "integer16And" | "integer32And" | "integer64And" => BasicValueEnum::IntValue(context.builder().build_and(a, b.unwrap(), "and")),
				"integer8Xor" | "integer16Xor" | "integer32Xor" | "integer64Xor" => BasicValueEnum::IntValue(context.builder().build_xor(a, b.unwrap(), "xor")),
				"integer8Or" | "integer16Or" | "integer32Or" | "integer64Or" => BasicValueEnum::IntValue(context.builder().build_xor(a, b.unwrap(), "or")),

				"integer8Shl" | "integer16Shl" | "integer32Shl" | "integer64Shl" => BasicValueEnum::IntValue(context.builder().build_left_shift(a, b.unwrap(), "left_shift")),
				"integer8Shr" | "integer16Shr" | "integer32Shr" | "integer64Shr" => BasicValueEnum::IntValue(context.builder().build_right_shift(a, b.unwrap(), false, "diff")),
				"integer8ShrSig" | "integer16ShrSig" | "integer32ShrSig" | "integer64ShrSig" => BasicValueEnum::IntValue(context.builder().build_right_shift(a, b.unwrap(), true, "sum")),

				"integer8CmpEq" | "integer16CmpEq" | "integer32CmpEq" | "integer64CmpEq" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::EQ, a, b.unwrap(), "cmp_eq")),
				"integer8CmpNeq" | "integer16CmpNeq" | "integer32CmpNeq" | "integer64CmpNeq" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::NE, a, b.unwrap(), "cmp_neq")),
				"integer8CmpLt" | "integer16CmpLt" | "integer32CmpLt" | "integer64CmpLt" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::ULT, a, b.unwrap(), "cmp_lt")),
				"integer8CmpGt" | "integer16CmpGt" | "integer32CmpGt" | "integer64CmpGt" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::UGT, a, b.unwrap(), "cmp_gt")),
				"integer8CmpLte" | "integer16CmpLte" | "integer32CmpLte" | "integer64CmpLte" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::ULE, a, b.unwrap(), "cmp_lte")),
				"integer8CmpGte" | "integer16CmpGte" | "integer32CmpGte" | "integer64CmpGte" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::UGE, a, b.unwrap(), "cmp_gte")),
				"integer8CmpLtSig" | "integer16CmpLtSig" | "integer32CmpLtSig" | "integer64CmpLtSig" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::SLT, a, b.unwrap(), "cmp_lt")),
				"integer8CmpGtSig" | "integer16CmpGtSig" | "integer32CmpGtSig" | "integer64CmpGtSig" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::SGT, a, b.unwrap(), "cmp_gt")),
				"integer8CmpLteSig" | "integer16CmpLteSig" | "integer32CmpLteSig" | "integer64CmpLteSig" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::SLE, a, b.unwrap(), "cmp_lte")),
				"integer8CmpGteSig" | "integer16CmpGteSig" | "integer32CmpGteSig" | "integer64CmpGteSig" => BasicValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::SGE, a, b.unwrap(), "cmp_gte")),

				"integer8Negate" | "integer16Negate" | "integer32Negate" | "integer64Negate" => BasicValueEnum::IntValue(context.builder().build_int_neg(a, "negate")),

				"integer8ExtZero64" | "integer16ExtZero64" | "integer32ExtZero64" => BasicValueEnum::IntValue(context.builder().build_int_z_extend(a, context.context().i64_type(), "zext64")),
				"integer8ExtSig64" | "integer16ExtSig64" | "integer32ExtSig64" => BasicValueEnum::IntValue(context.builder().build_int_s_extend(a, context.context().i64_type(), "sext64")),
				"integer8ExtZero32" | "integer16ExtZero32" => BasicValueEnum::IntValue(context.builder().build_int_z_extend(a, context.context().i32_type(), "zext32")),
				"integer8ExtSig32" | "integer16ExtSig32" => BasicValueEnum::IntValue(context.builder().build_int_s_extend(a, context.context().i32_type(), "sext32")),
				"integer8ExtZero16" => BasicValueEnum::IntValue(context.builder().build_int_z_extend(a, context.context().i16_type(), "zext16")),
				"integer8ExtSig16" => BasicValueEnum::IntValue(context.builder().build_int_s_extend(a, context.context().i16_type(), "zext16")),

				"integer16Trunc8" | "integer32Trunc8" | "integer64Trunc8" => BasicValueEnum::IntValue(context.builder().build_int_truncate(a, context.context().i8_type(), "trunc8")),
				"integer32Trun16" | "integer64Trun16" => BasicValueEnum::IntValue(context.builder().build_int_truncate(a, context.context().i16_type(), "trunc16")),
				"integer64Trunc32" => BasicValueEnum::IntValue(context.builder().build_int_truncate(a, context.context().i32_type(), "trunc32")),

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