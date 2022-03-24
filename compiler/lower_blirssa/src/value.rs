use blirssa::{value::{Value, BinaryIntrinsicFn, UnaryIntrinsicFn}, typ::Type};
use inkwell::{values::{BasicValueEnum, BasicValue, FunctionValue}, builder::Builder, IntPredicate, FloatPredicate, context::Context};

use crate::{ModuleContext, func::FunctionContext, typ::lower_basic_typ};

pub fn lower_value<'a, 'ctx>(value: &Value, context: &ModuleContext<'a, 'ctx>, fn_ctx: &FunctionContext<'ctx>) -> Result<LLVMValue<'ctx>, String> {
	Ok(match value {
		Value::IntegerLiteral { typ, value } => {
			LLVMValue::Basic(
				lower_basic_typ(typ, context)
					.expect("Not an integer type")
					.into_int_type()
					.const_int(*value, false)
					.as_basic_value_enum()
			)
		}

		Value::FloatLiteral { typ, value } => {
			LLVMValue::Basic(
				lower_basic_typ(typ, context)
					.expect("Not a float type")
					.into_float_type()
					.const_float(*value)
					.as_basic_value_enum()
			)
		}

		Value::BinaryIntrinsic { name, left, right, .. } => {
			let lhs = fn_ctx.get_local(&left).basic();
			let rhs = fn_ctx.get_local(&right).basic();

			LLVMValue::Basic(
				build_binary_intrinsic(*name, lhs, rhs, context.builder)
			)
		}

		Value::UnaryIntrinsic { name, arg, .. } => {
			let value = fn_ctx.get_local(&arg).basic();

			LLVMValue::Basic(
				build_unary_intrinsic(*name, value, context.context, context.builder)
			)
		}

		Value::AllocOnStackUndef { typ } => {
			let Type::Pointer { pointee } = typ else {
				panic!();
			};

			let lowered_ty = lower_basic_typ(pointee, context).expect("Can't alloc type");

			LLVMValue::Basic(
				context.builder.build_alloca(lowered_ty, "stack-alloc").as_basic_value_enum()
			)
		}

		Value::AllocOnStack { value, .. } => {
			let ty = lower_basic_typ(value.typ_ref(), context).expect("Can't alloc type");

			let ptr_value = context.builder.build_alloca(ty, "stack-alloc");

			LLVMValue::Basic(ptr_value.as_basic_value_enum())
		}

		Value::Deref { pointer, .. } => {
			let pointer = fn_ctx.get_local(&pointer).basic();

			LLVMValue::Basic(
				context.builder.build_load(pointer.into_pointer_value(), "load")
					.as_basic_value_enum()
			)
		}

		Value::Function { function } => {
			let function = context.module.get_function(function.name())
				.expect("Function doesn't exist");

			LLVMValue::Function(function)
		}

		Value::ExternFunction { function } => {
			let function = context.module.get_function(function.name())
				.expect("Function doesn't exist");

			LLVMValue::Function(function)
		}

		Value::Call { function, args, .. } => {
			let function = fn_ctx.get_local(&function);

			match function {
				LLVMValue::Function(function) => {
					// Maybe make a pointer to the function if it's an arg
					let args = args
						.iter()
						.filter_map(|arg| fn_ctx.get_local(&arg).try_basic())
						.map(|basic| basic.into())
						.collect::<Vec<_>>();

					match context.builder.build_call(function, &args, "call").try_as_basic_value().left() {
						Some(basic_value) => LLVMValue::Basic(basic_value),
						None => LLVMValue::Void,
					}
				}

				// Only handle second-class functions
				_ => panic!(),
			}
		}

		Value::DerefStructField { r#struct, field, .. } => {
			match r#struct.typ_ref() {
				Type::Pointer { pointee } => {
					let Type::Struct { container } = &**pointee else {
						panic!("deref-struct-field can only be used on a pointer to a struct");
					};
		
					let r#struct = fn_ctx.get_local(r#struct);
		
					let gep = if container.transparent_type().is_some() {
						r#struct.basic().into_pointer_value()
					} else {
						let ptr = r#struct.basic().into_pointer_value();
						let idx = container.get_field_index(field);

						context.builder.build_struct_gep(ptr, idx, "access-struct-field")
								.unwrap()	
					};

					LLVMValue::Basic(
						context.builder.build_load(gep, "deref")
					)
				}

				Type::Struct { container } => {
					let r#struct = fn_ctx.get_local(r#struct);

					if container.transparent_type().is_some() {
						r#struct
					} else {
						let ptr = r#struct.basic().into_struct_value();
						let idx = container.get_field_index(field);
		
						LLVMValue::Basic(
							context.builder.build_extract_value(ptr, idx, "deref-struct-field")
								.unwrap()
								.as_basic_value_enum()
						)
					}
				}

				_ => panic!("deref-struct-field can only be used on a struct or a pointer to one")
			}
		}

		Value::AccessStructField { r#struct, field, .. } => {
			let Type::Pointer { pointee } = r#struct.typ_ref() else {
				panic!("access-struct-field can only be used on a pointer to a struct");
			};

			let Type::Struct { container } = &**pointee else {
				panic!("access-struct-field can only be used on a pointer to a struct");
			};

			let r#struct = fn_ctx.get_local(r#struct);

			if container.transparent_type().is_some() {
				r#struct
			} else {
				let ptr = r#struct.basic().into_pointer_value();
				let idx = container.get_field_index(field);

				LLVMValue::Basic(
					context.builder.build_struct_gep(ptr, idx, "access-struct-field")
						.unwrap()
						.as_basic_value_enum()
				)
			}
		}
	})
}

fn build_binary_intrinsic<'ctx>(name: BinaryIntrinsicFn, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, builder: &Builder<'ctx>) -> BasicValueEnum<'ctx> {
	match name {
		BinaryIntrinsicFn::IntegerAdd => builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "add").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerSub => builder.build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "sub").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerMul => builder.build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "mul").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerDiv => builder.build_int_unsigned_div(lhs.into_int_value(), rhs.into_int_value(), "div").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerRem => builder.build_int_unsigned_rem(lhs.into_int_value(), rhs.into_int_value(), "rem").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerDivSig => builder.build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "div").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerRemSig => builder.build_int_signed_rem(lhs.into_int_value(), rhs.into_int_value(), "rem").as_basic_value_enum(),

		BinaryIntrinsicFn::IntegerAnd => builder.build_and(lhs.into_int_value(), rhs.into_int_value(), "and").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerXor => builder.build_xor(lhs.into_int_value(), rhs.into_int_value(), "xor").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerOr => builder.build_or(lhs.into_int_value(), rhs.into_int_value(), "or").as_basic_value_enum(),

		BinaryIntrinsicFn::IntegerShl => builder.build_left_shift(lhs.into_int_value(), rhs.into_int_value(), "shl").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerShr => builder.build_right_shift(lhs.into_int_value(), rhs.into_int_value(), false, "shr").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerShrSig => builder.build_right_shift(lhs.into_int_value(), rhs.into_int_value(), true, "shr").as_basic_value_enum(),

		BinaryIntrinsicFn::IntegerCmpEq => builder.build_int_compare(IntPredicate::EQ, lhs.into_int_value(), rhs.into_int_value(), "cmpEq").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpNeq => builder.build_int_compare(IntPredicate::NE, lhs.into_int_value(), rhs.into_int_value(), "cmpNeq").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpLt => builder.build_int_compare(IntPredicate::ULT, lhs.into_int_value(), rhs.into_int_value(), "cmpLt").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpLte => builder.build_int_compare(IntPredicate::ULE, lhs.into_int_value(), rhs.into_int_value(), "cmpLte").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpGt => builder.build_int_compare(IntPredicate::UGT, lhs.into_int_value(), rhs.into_int_value(), "cmpGt").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpGte => builder.build_int_compare(IntPredicate::UGE, lhs.into_int_value(), rhs.into_int_value(), "cmpGte").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpLtSig => builder.build_int_compare(IntPredicate::SLT, lhs.into_int_value(), rhs.into_int_value(), "cmpLt").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpLteSig => builder.build_int_compare(IntPredicate::SLE, lhs.into_int_value(), rhs.into_int_value(), "cmpLte").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpGtSig => builder.build_int_compare(IntPredicate::SGT, lhs.into_int_value(), rhs.into_int_value(), "cmpGt").as_basic_value_enum(),
		BinaryIntrinsicFn::IntegerCmpGteSig => builder.build_int_compare(IntPredicate::SGE, lhs.into_int_value(), rhs.into_int_value(), "cmpGte").as_basic_value_enum(),

		BinaryIntrinsicFn::FloatAdd => builder.build_float_add(lhs.into_float_value(), rhs.into_float_value(), "add").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatSub => builder.build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "sub").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatMul => builder.build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "mul").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatDiv => builder.build_float_div(lhs.into_float_value(), rhs.into_float_value(), "div").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatRem => builder.build_float_rem(lhs.into_float_value(), rhs.into_float_value(), "rem").as_basic_value_enum(),

		// Should these be U or O?
		BinaryIntrinsicFn::FloatCmpEq => builder.build_float_compare(FloatPredicate::OEQ, lhs.into_float_value(), rhs.into_float_value(), "cmpEq").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatCmpNeq => builder.build_float_compare(FloatPredicate::ONE, lhs.into_float_value(), rhs.into_float_value(), "cmpNeq").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatCmpLt => builder.build_float_compare(FloatPredicate::OLT, lhs.into_float_value(), rhs.into_float_value(), "cmpLt").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatCmpLte => builder.build_float_compare(FloatPredicate::OLE, lhs.into_float_value(), rhs.into_float_value(), "cmpLte").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatCmpGt => builder.build_float_compare(FloatPredicate::OGT, lhs.into_float_value(), rhs.into_float_value(), "cmpGt").as_basic_value_enum(),
		BinaryIntrinsicFn::FloatCmpGte => builder.build_float_compare(FloatPredicate::OGE, lhs.into_float_value(), rhs.into_float_value(), "cmpGte").as_basic_value_enum(),
	}
}

fn build_unary_intrinsic<'ctx>(name: UnaryIntrinsicFn, value: BasicValueEnum<'ctx>, context: &'ctx Context, builder: &Builder<'ctx>) -> BasicValueEnum<'ctx> {
	match name {
		UnaryIntrinsicFn::IntegerNegate => builder.build_int_neg(value.into_int_value(), "negate").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerInvert => builder.build_not(value.into_int_value(), "invert").as_basic_value_enum(),

		UnaryIntrinsicFn::IntegerExt64 => builder.build_int_z_extend(value.into_int_value(), context.i64_type(), "zext64").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerExt32 => builder.build_int_z_extend(value.into_int_value(), context.i32_type(), "zext32").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerExt16 => builder.build_int_z_extend(value.into_int_value(), context.i16_type(), "zext16").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerExt64Sig => builder.build_int_s_extend(value.into_int_value(), context.i64_type(), "sext64").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerExt32Sig => builder.build_int_s_extend(value.into_int_value(), context.i32_type(), "sext32").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerExt16Sig => builder.build_int_s_extend(value.into_int_value(), context.i16_type(), "sext16").as_basic_value_enum(),

		UnaryIntrinsicFn::IntegerTrunc32 => builder.build_int_truncate(value.into_int_value(), context.i32_type(), "trunc32").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerTrunc16 => builder.build_int_truncate(value.into_int_value(), context.i16_type(), "trunc16").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerTrunc8 => builder.build_int_truncate(value.into_int_value(), context.i8_type(), "trunc8").as_basic_value_enum(),

		UnaryIntrinsicFn::IntegerToFloat16 => builder.build_unsigned_int_to_float(value.into_int_value(), context.f16_type(), "uitof").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerToFloat32 => builder.build_unsigned_int_to_float(value.into_int_value(), context.f32_type(), "uitof").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerToFloat64 => builder.build_unsigned_int_to_float(value.into_int_value(), context.f64_type(), "uitof").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerToFloat16Sig => builder.build_signed_int_to_float(value.into_int_value(), context.f16_type(), "itof").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerToFloat32Sig => builder.build_signed_int_to_float(value.into_int_value(), context.f32_type(), "itof").as_basic_value_enum(),
		UnaryIntrinsicFn::IntegerToFloat64Sig => builder.build_signed_int_to_float(value.into_int_value(), context.f64_type(), "itof").as_basic_value_enum(),


		UnaryIntrinsicFn::FloatNegate => builder.build_float_neg(value.into_float_value(), "negate").as_basic_value_enum(),

		UnaryIntrinsicFn::FloatExt64 => builder.build_float_ext(value.into_float_value(), context.f64_type(), "floatExt64").as_basic_value_enum(),
		UnaryIntrinsicFn::FloatExt32 => builder.build_float_ext(value.into_float_value(), context.f32_type(), "floatExt32").as_basic_value_enum(),

		UnaryIntrinsicFn::FloatTrunc32 => builder.build_float_trunc(value.into_float_value(), context.f32_type(), "floatTrunc32").as_basic_value_enum(),
		UnaryIntrinsicFn::FloatTrunc16 => builder.build_float_trunc(value.into_float_value(), context.f16_type(), "floatTrunc16").as_basic_value_enum(),

		// TODO: Add float to signed int
		UnaryIntrinsicFn::FloatToInt => builder.build_float_to_unsigned_int(value.into_float_value(), context.i64_type(), "ftoui").as_basic_value_enum(),
		UnaryIntrinsicFn::FloatToIntSig => builder.build_float_to_signed_int(value.into_float_value(), context.i64_type(), "ftosi").as_basic_value_enum(),
	}
}

#[derive(Clone, Copy)]
pub enum LLVMValue<'ctx> {
	Void,
	Function(FunctionValue<'ctx>),
	Basic(BasicValueEnum<'ctx>),
}

impl<'ctx> LLVMValue<'ctx> {
	pub fn basic(self) -> BasicValueEnum<'ctx> {
		match self {
			Self::Basic(basic) => basic,
			_ => panic!("Not a basic value")
		}
	}

	pub fn try_basic(self) -> Option<BasicValueEnum<'ctx>> {
		match self {
			Self::Basic(basic) => Some(basic),
			_ => None,
		}
	}
}