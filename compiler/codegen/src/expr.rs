use blir::{Expr, ExprKind, TypeKind};
use inkwell::{values::{AnyValueEnum, AnyValue, BasicValueEnum, AggregateValueEnum}, IntPredicate, FloatPredicate};

use crate::{context::{FuncGenContext}, typ::generate_type, smt::generate_smt};

pub fn generate_expr<'a, 'ctx>(expr: &Expr, context: &FuncGenContext<'a, 'ctx>) -> Option<AnyValueEnum<'ctx>> {
	Some(match expr.kind() {
		ExprKind::IntLiteral(value) => {
			AnyValueEnum::IntValue(
				generate_type(expr.typ_ref(), context.library())
					.unwrap()
					.into_int_type()
					.const_int(*value, false)
			)
		}

		ExprKind::FloatLiteral(value) => {
			AnyValueEnum::FloatValue(
				generate_type(expr.typ_ref(), context.library())
					.unwrap()
					.into_float_type()
					.const_float(*value)
			)
		}

		ExprKind::FuncCall { func, args } => {
			let args = args
				.iter()
				.map(|arg| arg.value())
				.filter_map(|arg| generate_expr(arg, context))
				.collect::<Vec<_>>();
			
			return generate_func_call(&func, &args, context)
		}

		ExprKind::FunctionParameter(n) => {
			context.function().get_nth_param(*n as u32).unwrap().as_any_value_enum()
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

				let cond = generate_expr(branches[i].condition(), context).unwrap();

				context.builder().build_conditional_branch(cond.into_int_value(), positive_block, negative_block);

				context.builder().position_at_end(negative_block);
			}

			let continue_block = if let Some(finally) = finally {
				let positive_block = branch_blocks.last().cloned().unwrap();
				let negative_block = context.context().append_basic_block(context.function().clone(), "finally");

				let cond = generate_expr(branches.last().unwrap().condition(), context).unwrap();

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

				let cond = generate_expr(branches.last().unwrap().condition(), context).unwrap();

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

			AnyValueEnum::IntValue(context.context().i64_type().const_int(0, true))
		}

		ExprKind::InstanceVariable { instance, variable } => {
			let expr = generate_expr(&instance, context)?.into_struct_value();

			return context.builder().build_extract_value(expr, variable.field_index() as u32, "member_access")
				.map(|x| x.as_any_value_enum())
		}

		n => panic!("{n}"),
	})
}

fn generate_func_call<'a, 'ctx>(func: &Expr, args: &Vec<AnyValueEnum<'ctx>>, context: &FuncGenContext<'a, 'ctx>) -> Option<AnyValueEnum<'ctx>> {
	match func.kind() {
		ExprKind::IntrinsicFunc(name) => {
			let a = args[0];
			let b = args.get(1);

			Some(match name.as_str() {
				"integer8Add" | "integer16Add" | "integer32Add" | "integer64Add" => AnyValueEnum::IntValue(context.builder().build_int_add(a.into_int_value(), b.unwrap().into_int_value(), "sum")),
				"integer8Sub" | "integer16Sub" | "integer32Sub" | "integer64Sub" => AnyValueEnum::IntValue(context.builder().build_int_sub(a.into_int_value(), b.unwrap().into_int_value(), "diff")),
				"integer8Mul" | "integer16Mul" | "integer32Mul" | "integer64Mul" => AnyValueEnum::IntValue(context.builder().build_int_mul(a.into_int_value(), b.unwrap().into_int_value(), "mul")),
				"integer8Div" | "integer16Div" | "integer32Div" | "integer64Div" => AnyValueEnum::IntValue(context.builder().build_int_unsigned_div(a.into_int_value(), b.unwrap().into_int_value(), "div")),
				"integer8DivSig" | "integer16DivSig" | "integer32DivSig" | "integer64DivSig" => AnyValueEnum::IntValue(context.builder().build_int_signed_div(a.into_int_value(), b.unwrap().into_int_value(), "div")),
				"integer8Rem" | "integer16Rem" | "integer32Rem" | "integer64Rem" => AnyValueEnum::IntValue(context.builder().build_int_unsigned_rem(a.into_int_value(), b.unwrap().into_int_value(), "rem")),
				"integer8RemSig" | "integer16RemSig" | "integer32RemSig" | "integer64RemSig" => AnyValueEnum::IntValue(context.builder().build_int_unsigned_rem(a.into_int_value(), b.unwrap().into_int_value(), "rem")),
				
				"integer8And" | "integer16And" | "integer32And" | "integer64And" => AnyValueEnum::IntValue(context.builder().build_and(a.into_int_value(), b.unwrap().into_int_value(), "and")),
				"integer8Xor" | "integer16Xor" | "integer32Xor" | "integer64Xor" => AnyValueEnum::IntValue(context.builder().build_xor(a.into_int_value(), b.unwrap().into_int_value(), "xor")),
				"integer8Or" | "integer16Or" | "integer32Or" | "integer64Or" => AnyValueEnum::IntValue(context.builder().build_xor(a.into_int_value(), b.unwrap().into_int_value(), "or")),

				"integer8Shl" | "integer16Shl" | "integer32Shl" | "integer64Shl" => AnyValueEnum::IntValue(context.builder().build_left_shift(a.into_int_value(), b.unwrap().into_int_value(), "left_shift")),
				"integer8Shr" | "integer16Shr" | "integer32Shr" | "integer64Shr" => AnyValueEnum::IntValue(context.builder().build_right_shift(a.into_int_value(), b.unwrap().into_int_value(), false, "diff")),
				"integer8ShrSig" | "integer16ShrSig" | "integer32ShrSig" | "integer64ShrSig" => AnyValueEnum::IntValue(context.builder().build_right_shift(a.into_int_value(), b.unwrap().into_int_value(), true, "sum")),

				"integer8CmpEq" | "integer16CmpEq" | "integer32CmpEq" | "integer64CmpEq" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::EQ, a.into_int_value(), b.unwrap().into_int_value(), "cmp_eq")),
				"integer8CmpNeq" | "integer16CmpNeq" | "integer32CmpNeq" | "integer64CmpNeq" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::NE, a.into_int_value(), b.unwrap().into_int_value(), "cmp_neq")),
				"integer8CmpLt" | "integer16CmpLt" | "integer32CmpLt" | "integer64CmpLt" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::ULT, a.into_int_value(), b.unwrap().into_int_value(), "cmp_lt")),
				"integer8CmpGt" | "integer16CmpGt" | "integer32CmpGt" | "integer64CmpGt" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::UGT, a.into_int_value(), b.unwrap().into_int_value(), "cmp_gt")),
				"integer8CmpLte" | "integer16CmpLte" | "integer32CmpLte" | "integer64CmpLte" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::ULE, a.into_int_value(), b.unwrap().into_int_value(), "cmp_lte")),
				"integer8CmpGte" | "integer16CmpGte" | "integer32CmpGte" | "integer64CmpGte" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::UGE, a.into_int_value(), b.unwrap().into_int_value(), "cmp_gte")),
				"integer8CmpLtSig" | "integer16CmpLtSig" | "integer32CmpLtSig" | "integer64CmpLtSig" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::SLT, a.into_int_value(), b.unwrap().into_int_value(), "cmp_lt")),
				"integer8CmpGtSig" | "integer16CmpGtSig" | "integer32CmpGtSig" | "integer64CmpGtSig" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::SGT, a.into_int_value(), b.unwrap().into_int_value(), "cmp_gt")),
				"integer8CmpLteSig" | "integer16CmpLteSig" | "integer32CmpLteSig" | "integer64CmpLteSig" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::SLE, a.into_int_value(), b.unwrap().into_int_value(), "cmp_lte")),
				"integer8CmpGteSig" | "integer16CmpGteSig" | "integer32CmpGteSig" | "integer64CmpGteSig" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::SGE, a.into_int_value(), b.unwrap().into_int_value(), "cmp_gte")),

				"integer8Negate" | "integer16Negate" | "integer32Negate" | "integer64Negate" => AnyValueEnum::IntValue(context.builder().build_int_neg(a.into_int_value(), "negate")),

				"integer8ExtZero64" | "integer16ExtZero64" | "integer32ExtZero64" => AnyValueEnum::IntValue(context.builder().build_int_z_extend(a.into_int_value(), context.context().i64_type(), "zext64")),
				"integer8ExtSig64" | "integer16ExtSig64" | "integer32ExtSig64" => AnyValueEnum::IntValue(context.builder().build_int_s_extend(a.into_int_value(), context.context().i64_type(), "sext64")),
				"integer8ExtZero32" | "integer16ExtZero32" => AnyValueEnum::IntValue(context.builder().build_int_z_extend(a.into_int_value(), context.context().i32_type(), "zext32")),
				"integer8ExtSig32" | "integer16ExtSig32" => AnyValueEnum::IntValue(context.builder().build_int_s_extend(a.into_int_value(), context.context().i32_type(), "sext32")),
				"integer8ExtZero16" => AnyValueEnum::IntValue(context.builder().build_int_z_extend(a.into_int_value(), context.context().i16_type(), "zext16")),
				"integer8ExtSig16" => AnyValueEnum::IntValue(context.builder().build_int_s_extend(a.into_int_value(), context.context().i16_type(), "zext16")),

				"integer16Trunc8" | "integer32Trunc8" | "integer64Trunc8" => AnyValueEnum::IntValue(context.builder().build_int_truncate(a.into_int_value(), context.context().i8_type(), "trunc8")),
				"integer32Trun16" | "integer64Trun16" => AnyValueEnum::IntValue(context.builder().build_int_truncate(a.into_int_value(), context.context().i16_type(), "trunc16")),
				"integer64Trunc32" => AnyValueEnum::IntValue(context.builder().build_int_truncate(a.into_int_value(), context.context().i32_type(), "trunc32")),
				

				"float16Add" | "float32Add" | "float64Add" => AnyValueEnum::FloatValue(context.builder().build_float_add(a.into_float_value(), b.unwrap().into_float_value(), "add")),
				"float16Sub" | "float32Sub" | "float64Sub" => AnyValueEnum::FloatValue(context.builder().build_float_sub(a.into_float_value(), b.unwrap().into_float_value(), "sub")),
				"float16Mul" | "float32Mul" | "float64Mul" => AnyValueEnum::FloatValue(context.builder().build_float_mul(a.into_float_value(), b.unwrap().into_float_value(), "mul")),
				"float16Div" | "float32Div" | "float64Div" => AnyValueEnum::FloatValue(context.builder().build_float_div(a.into_float_value(), b.unwrap().into_float_value(), "div")),
				"float16Rem" | "float32Rem" | "float64Rem" => AnyValueEnum::FloatValue(context.builder().build_float_rem(a.into_float_value(), b.unwrap().into_float_value(), "rem")),

				"float16CmpEq" | "float32CmpEq" | "float64CmpEq" => AnyValueEnum::IntValue(context.builder().build_float_compare(FloatPredicate::OEQ, a.into_float_value(), b.unwrap().into_float_value(), "eq")),
				"float16CmpNeq" | "float32CmpNeq" | "float64CmpNeq" => AnyValueEnum::IntValue(context.builder().build_float_compare(FloatPredicate::ONE, a.into_float_value(), b.unwrap().into_float_value(), "neq")),
				"float16CmpLt" | "float32CmpLt" | "float64CmpLt" => AnyValueEnum::IntValue(context.builder().build_float_compare(FloatPredicate::OLT, a.into_float_value(), b.unwrap().into_float_value(), "cmpLt")),
				"float16CmpGt" | "float32CmpGt" | "float64CmpGt" => AnyValueEnum::IntValue(context.builder().build_float_compare(FloatPredicate::OGT, a.into_float_value(), b.unwrap().into_float_value(), "cmpGt")),
				"float16CmpLte" | "float32CmpLte" | "float64CmpLte" => AnyValueEnum::IntValue(context.builder().build_float_compare(FloatPredicate::OLE, a.into_float_value(), b.unwrap().into_float_value(), "cmpLte")),
				"float16CmpGte" | "float32CmpGte" | "float64CmpGte" => AnyValueEnum::IntValue(context.builder().build_float_compare(FloatPredicate::OGE, a.into_float_value(), b.unwrap().into_float_value(), "cmpGte")),

				"float16Negate" | "float32Negate" | "float64Negate" => AnyValueEnum::FloatValue(context.builder().build_float_neg(a.into_float_value(), "negate")),

				"float32Trunc16" | "float64Trunc16" => AnyValueEnum::FloatValue(context.builder().build_float_trunc(a.into_float_value(), context.context().f16_type(), "trunc16")),
				"float64Trunc32" => AnyValueEnum::FloatValue(context.builder().build_float_trunc(a.into_float_value(), context.context().f32_type(), "trunc32")),

				"float16Ext32" => AnyValueEnum::FloatValue(context.builder().build_float_ext(a.into_float_value(), context.context().f32_type(), "ext32")),
				"float32Ext64" | "float16Ext64" => AnyValueEnum::FloatValue(context.builder().build_float_ext(a.into_float_value(), context.context().f64_type(), "ext64")),

				"float64ToInt" => AnyValueEnum::IntValue(context.builder().build_float_to_unsigned_int(a.into_float_value(), context.context().i64_type(), "toInt64")),
				"float64ToIntSig" => AnyValueEnum::IntValue(context.builder().build_float_to_signed_int(a.into_float_value(), context.context().i64_type(), "toInt64")),

				"float64FromInt" => AnyValueEnum::FloatValue(context.builder().build_unsigned_int_to_float(a.into_int_value(), context.context().f64_type(), "fromInt64")),
				"float64FromIntSig" => AnyValueEnum::FloatValue(context.builder().build_signed_int_to_float(a.into_int_value(), context.context().f64_type(), "fromInt64")),
				"float32FromInt" => AnyValueEnum::FloatValue(context.builder().build_unsigned_int_to_float(a.into_int_value(), context.context().f32_type(), "fromInt32")),
				"float32FromIntSig" => AnyValueEnum::FloatValue(context.builder().build_signed_int_to_float(a.into_int_value(), context.context().f32_type(), "fromInt32")),
				"float16FromInt" => AnyValueEnum::FloatValue(context.builder().build_unsigned_int_to_float(a.into_int_value(), context.context().f16_type(), "fromInt16")),
				"float16FromIntSig" => AnyValueEnum::FloatValue(context.builder().build_signed_int_to_float(a.into_int_value(), context.context().f16_type(), "fromInt16")),

				"integer1And" => AnyValueEnum::IntValue(context.builder().build_and(a.into_int_value(), b.unwrap().into_int_value(), "and")),
				"integer1Xor" => AnyValueEnum::IntValue(context.builder().build_xor(a.into_int_value(), b.unwrap().into_int_value(), "xor")),
				"integer1Or" => AnyValueEnum::IntValue(context.builder().build_or(a.into_int_value(), b.unwrap().into_int_value(), "or")),
				"integer1Invert" => AnyValueEnum::IntValue(context.builder().build_int_neg(a.into_int_value(), "invert")),
				"integer1CmpEq" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::EQ, a.into_int_value(), b.unwrap().into_int_value(), "cmpEq")),
				"integer1CmpNeq" => AnyValueEnum::IntValue(context.builder().build_int_compare(IntPredicate::EQ, a.into_int_value(), b.unwrap().into_int_value(), "cmpNeq")),

				_ => panic!(),
			})
		}

		ExprKind::Function(f) => {
			let func = context.module().get_function(&f.link_name()).unwrap();
			
			let args = args.iter()
				.filter_map(|a| BasicValueEnum::try_from(*a).ok())
				.map(|a| a.into()).collect::<Vec<_>>();

			context.builder().build_call(func, &args, "call")
				.try_as_basic_value()
				.left()
				.map(|a| a.as_any_value_enum())
		}

		ExprKind::StaticMethod(f) => {
			let func = context.module().get_function(&f.link_name()).unwrap();

			let args = args.iter()
				.filter_map(|a| BasicValueEnum::try_from(*a).ok())
				.map(|a| a.into()).collect::<Vec<_>>();

			context.builder().build_call(func, &args, "call")
				.try_as_basic_value()
				.left()
				.map(|a| a.as_any_value_enum())
		}

		ExprKind::Init(ty) => {
			match ty.kind() {
				TypeKind::StructRef(r#struct) => {
					let llvm_typ = context.types().get_type(&r#struct.link_name());

					let mut struct_val = llvm_typ
						.into_struct_type()
						.get_undef();

					for (i, arg) in args.iter().enumerate() {
						if let Ok(basic_arg) = BasicValueEnum::try_from(*arg) {
							struct_val = context.builder().build_insert_value(struct_val, basic_arg, i as u32, "init").unwrap().into_struct_value();
						}
						
					}

					Some(struct_val.as_any_value_enum())
				}
				_ => panic!(),
			}
		}

		ExprKind::ExternFunction(f) => {
			let func = context.module().get_function(&f.link_name()).unwrap();

			let args = args.iter()
				.filter_map(|a| BasicValueEnum::try_from(*a).ok())
				.map(|a| a.into()).collect::<Vec<_>>();

			context.builder().build_call(func, &args, "call")
				.try_as_basic_value()
				.left()
				.map(|a| a.as_any_value_enum())
		}

		_ => {
			println!("Panicked on {func}");
			panic!()
		},
	}
}