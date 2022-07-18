use inkwell::{values::{BasicValueEnum, IntValue, FloatValue, PointerValue, BasicMetadataValueEnum, CallableValue}, IntPredicate, FloatPredicate};
use itertools::Itertools;
use mir::{val::{RValueKind, ConstValue, PlaceKind, DuoIntrinsic, RValue, SoloIntrinsic}, ty::TypeKind, code::Function};

use crate::{MirLowerContext, code::func};

use super::func::FunctionContext;

impl<'a, 'ctx> MirLowerContext<'a, 'ctx>
{
	pub fn lower_rvalue(
		&self,
		value: &mir::val::RValue,
		function: FunctionContext<'a, 'ctx>) -> BasicValueEnum<'ctx>
	{
		use RValueKind::*;
		use ConstValue::*;

		match value.kind() {
			Const(const_value) => match const_value {
				mir::val::ConstValue::Integer(n) => self.lower_int_value(*n, value.ty()).into(),
				mir::val::ConstValue::Float(n) => self.lower_float_value(*n, value.ty()).into(),
				mir::val::ConstValue::String(s) => { 
					self.builder.build_global_string_ptr(&s, "gstr").as_pointer_value().into()
				}
			},

			Move(place) | Copy(place) => {
				let llvm_place = self.lower_place(place, function);

				self.builder.build_load(llvm_place, "copy")
			}

			Function { name, .. } => {
				self.module.get_function(name)
						   .unwrap()
						   .as_global_value()
						   .as_pointer_value()
						   .into()
			}

			Ref(place) => self.lower_place(place, function).into(),

			Call { function: func, params } => self.lower_call_value(func, params, function),

			Tuple { items } => {
				let tuple_items = items.iter()
									   .map(|item| self.lower_rvalue(item, function))
									   .collect_vec();

				let mut tuple = self.lower_ty(value.ty()).const_zero().into_struct_value();

				for (i, item) in tuple_items.into_iter().enumerate() {
					tuple = self.builder.build_insert_value(tuple, item, i as u32, "tuple").unwrap().into_struct_value();
				}

				return tuple.into();
			}

			SoloIntrinsic { intrinsic, operand } => self.lower_solo_intrinsic(*intrinsic, operand, function),

			DuoIntrinsic { intrinsic, left, right } => self.lower_duo_intrinsic(*intrinsic, left, right, function),
		}
	}

	pub fn lower_place(
		&self,
		value: &mir::val::Place,
		function: FunctionContext<'a, 'ctx>) -> PointerValue<'ctx>
	{
		use PlaceKind::*;
		
		match value.kind()
		{
			Local(local_id) => {
				function.get_local(*local_id)
			}
			Global(global_id) => {
				let global = self.project.global(*global_id).unwrap();
				let llvm_global = self.module.get_global(global.name()).unwrap();

				llvm_global.as_pointer_value()
			}
			StructField(place, field_name) => {
				let llvm_place = self.lower_place(place, function);

				let place_id = match place.ty().kind() {
					TypeKind::Struct { id } => { id },
					_ => unreachable!()
				};

				let structure = self.project.get_struct(*place_id).unwrap();

				if structure.is_transparent() {
					return llvm_place;	
				} 

				let field_index = structure.field_index(field_name).unwrap();

				self.builder.build_struct_gep(llvm_place, field_index as u32, "gep").unwrap()
			}
			CastEnumVariant(place, tag, name) => {
				let TypeKind::Enum { id } = place.ty().kind() else {
					panic!()
				};

				let enum_def = self.project.get_enum(*id).unwrap();
				let variant_type = self.lower_pointer_ty(enum_def.get_variant_type(*tag).unwrap());

				let lowered_enum = self.lower_place(place, function);
				let generic_variant = self.builder.build_struct_gep(lowered_enum, 2, "gvariant").unwrap();
				self.builder.build_bitcast(generic_variant, variant_type, "castvariant").into_pointer_value()
			}
			TupleItem(tuple, n) => {
				let llvm_tuple = self.lower_place(tuple, function);

				self.builder.build_struct_gep(llvm_tuple, *n as u32, "tuple.item").unwrap()
			}
			ArrayIndex(array, index) => {
				let array_place = self.lower_place(array, function);

				let index = self.lower_rvalue(index, function).into_int_value();

				unsafe {
					self.builder.build_gep(array_place, &[index], "array_index")
				}
			} 
			Deref(rvalue) => {
				self.lower_rvalue(rvalue, function).into_pointer_value()
			}
			Discriminant(place) => {
				let enum_value = self.lower_place(place, function);

				self.builder.build_struct_gep(enum_value, 0, "discriminant").unwrap()
			}
		}
	}

	pub fn lower_solo_intrinsic(
		&self,
		intrinsic: SoloIntrinsic,
		operand: &RValue,
		function: FunctionContext<'a, 'ctx>) -> BasicValueEnum<'ctx>
	{
		let llvm_operand = self.lower_rvalue(operand, function);

		use SoloIntrinsic::*;

		if llvm_operand.is_int_value()
		{
			let llvm_operand = llvm_operand.into_int_value();
			
			match intrinsic
			{
				INeg => self.builder.build_int_neg(llvm_operand, "ineg").into(),
				IInv => self.builder.build_not(llvm_operand, "iinv").into(),
				IZext16 => self.builder.build_int_z_extend(llvm_operand, self.context.i16_type(), "izext16").into(),
				IZext32 => self.builder.build_int_z_extend(llvm_operand, self.context.i32_type(), "izext32").into(),
				IZext64 => self.builder.build_int_z_extend(llvm_operand, self.context.i64_type(), "izext64").into(),
				ISext16 => self.builder.build_int_s_extend(llvm_operand, self.context.i16_type(), "izext16").into(),
				ISext32 => self.builder.build_int_s_extend(llvm_operand, self.context.i32_type(), "izext32").into(),
				ISext64 => self.builder.build_int_s_extend(llvm_operand, self.context.i64_type(), "izext64").into(),
				ITrunc8 => self.builder.build_int_truncate(llvm_operand, self.context.i8_type(), "itrunc8").into(),
				ITrunc16 => self.builder.build_int_truncate(llvm_operand, self.context.i16_type(), "itrunc16").into(),
				ITrunc32 => self.builder.build_int_truncate(llvm_operand, self.context.i32_type(), "itrunc32").into(),
				ICnvF16 => self.builder.build_unsigned_int_to_float(llvm_operand, self.context.f16_type(), "icnvf16").into(),
				ICnvF32 => self.builder.build_unsigned_int_to_float(llvm_operand, self.context.f32_type(), "icnvf32").into(),
				ICnvF64 => self.builder.build_unsigned_int_to_float(llvm_operand, self.context.f64_type(), "icnvf64").into(),
				ICnvF16Sig => self.builder.build_signed_int_to_float(llvm_operand, self.context.f16_type(), "icnvf16").into(),
				ICnvF32Sig => self.builder.build_signed_int_to_float(llvm_operand, self.context.f32_type(), "icnvf32").into(),
				ICnvF64Sig => self.builder.build_signed_int_to_float(llvm_operand, self.context.f64_type(), "icnvf64").into(),
				AddrCnvPtr => {
					let ty = self.lower_pointer_ty(operand.ty());
					self.builder.build_int_to_ptr(llvm_operand, ty, "addrcnvptr").into()
				}

				_ => unreachable!(),
			}
		}
		else if llvm_operand.is_float_value()
		{
			let llvm_operand = llvm_operand.into_float_value();

			match intrinsic
			{
				FNeg => self.builder.build_float_neg(llvm_operand, "fneg").into(),
				FExt32 => self.builder.build_float_ext(llvm_operand, self.context.f32_type(), "fext32").into(),
				FExt64 => self.builder.build_float_ext(llvm_operand, self.context.f64_type(), "fext64").into(),
				FTrunc16 => self.builder.build_float_trunc(llvm_operand, self.context.f16_type(), "ftrunc16").into(),
				FTrunc32 => self.builder.build_float_trunc(llvm_operand, self.context.f32_type(), "ftrunc32").into(),
				FCnvI => self.builder.build_float_to_signed_int(llvm_operand, self.context.i64_type(), "fcnvi").into(),

				_ => unreachable!()
			}
		}
		else if llvm_operand.is_pointer_value()
		{
			let llvm_operand = llvm_operand.into_pointer_value();

			match intrinsic
			{
				PtrCnvAddr => self.builder.build_ptr_to_int(llvm_operand, self.context.i64_type(), "ptrcnvaddr").into(),

				_ => unreachable!(),
			}
		}
		else {
			unreachable!()
		}
	}

	pub fn lower_duo_intrinsic(
		&self,
		intrinsic: DuoIntrinsic,
		left: &RValue,
		right: &RValue,
		function: FunctionContext<'a, 'ctx>) -> BasicValueEnum<'ctx>
	{
		let llvm_left = self.lower_rvalue(left, function);
		let llvm_right = self.lower_rvalue(right, function);

		use DuoIntrinsic::*;

		if llvm_left.is_int_value() {
			let llvm_left = llvm_left.into_int_value();
			let llvm_right = llvm_right.into_int_value();

			match intrinsic {
				IAdd => self.builder.build_int_add(llvm_left, llvm_right, "iadd"),
				ISub => self.builder.build_int_sub(llvm_left, llvm_right, "isub"),
				IMul => self.builder.build_int_mul(llvm_left, llvm_right, "imul"),
				IDiv => self.builder.build_int_unsigned_div(llvm_left, llvm_right, "idiv"),
				IDivSig => self.builder.build_int_signed_div(llvm_left, llvm_right, "idiv"),
				IRem => self.builder.build_int_unsigned_rem(llvm_left, llvm_right, "irem"),
				IRemSig => self.builder.build_int_signed_rem(llvm_left, llvm_right, "irem"),
				IAnd => self.builder.build_and(llvm_left, llvm_right, "iand"),
				IOr => self.builder.build_or(llvm_left, llvm_right, "ior"),
				IXor =>  self.builder.build_xor(llvm_left, llvm_right, "ixor"),
				IShl => self.builder.build_left_shift(llvm_left, llvm_right, "ishl"),
				IShr => self.builder.build_right_shift(llvm_left, llvm_right, false, "ishr"),
				IShrSig => self.builder.build_right_shift(llvm_left, llvm_right, true, "ishr"),
				ICmpEq => self.builder.build_int_compare(IntPredicate::EQ, llvm_left, llvm_right, "eq"),
				ICmpNeq => self.builder.build_int_compare(IntPredicate::NE, llvm_left, llvm_right, "neq"),
				ICmpLt => self.builder.build_int_compare(IntPredicate::ULT, llvm_left, llvm_right, "lt"),
				ICmpLte => self.builder.build_int_compare(IntPredicate::ULE, llvm_left, llvm_right, "lte"),
				ICmpGt => self.builder.build_int_compare(IntPredicate::UGT, llvm_left, llvm_right, "gt"),
				ICmpGte => self.builder.build_int_compare(IntPredicate::UGE, llvm_left, llvm_right, "gte"),
				ICmpLtSig => self.builder.build_int_compare(IntPredicate::SLT, llvm_left, llvm_right, "lt"),
				ICmpLteSig => self.builder.build_int_compare(IntPredicate::SLE, llvm_left, llvm_right, "lte"),
				ICmpGtSig => self.builder.build_int_compare(IntPredicate::SGT, llvm_left, llvm_right, "gt"),
				ICmpGteSig => self.builder.build_int_compare(IntPredicate::SGE, llvm_left, llvm_right, "gte"),
				_ => unreachable!()
			}.into()
		}
		else if llvm_left.is_float_value() {
			let llvm_left = llvm_left.into_float_value();
			let llvm_right = llvm_right.into_float_value();

			match intrinsic
			{
				FAdd => self.builder.build_float_add(llvm_left, llvm_right, "fadd").into(),
				FSub => self.builder.build_float_sub(llvm_left, llvm_right, "fsub").into(),
				FMul => self.builder.build_float_mul(llvm_left, llvm_right, "fmul").into(),
				FDiv => self.builder.build_float_div(llvm_left, llvm_right, "fdiv").into(),
				FRem => self.builder.build_float_rem(llvm_left, llvm_right, "frem").into(),
				FCmpEq => self.builder.build_float_compare(FloatPredicate::OEQ, llvm_left, llvm_right, "feq").into(),
				FCmpNeq => self.builder.build_float_compare(FloatPredicate::ONE, llvm_left, llvm_right, "fneq").into(),
				FCmpLt => self.builder.build_float_compare(FloatPredicate::OLT, llvm_left, llvm_right, "flt").into(),
				FCmpLte => self.builder.build_float_compare(FloatPredicate::OLE, llvm_left, llvm_right, "flte").into(),
				FCmpGt => self.builder.build_float_compare(FloatPredicate::OGT, llvm_left, llvm_right, "fgt").into(),
				FCmpGte => self.builder.build_float_compare(FloatPredicate::OGE, llvm_left, llvm_right, "fgte").into(),
				_ => unreachable!()
			}
		}
		else if llvm_left.is_pointer_value() {
			todo!()
		}
		else if llvm_left.is_array_value() {
			todo!()
		}
		else {
			unreachable!()
		}
	}

	fn lower_int_value(
		&self,
		n: u64,
		ty: mir::ty::Type) -> IntValue<'ctx>
	{
		let llvm_ty = match ty.kind() {
			TypeKind::Integer { bits } => self.lower_int_ty(*bits),
			_ => unreachable!()
		};

		llvm_ty.const_int(n, false)
	}

	fn lower_float_value(
		&self,
		n: f64,
		ty: mir::ty::Type) -> FloatValue<'ctx>
	{
		let llvm_ty = match ty.kind() {
			TypeKind::Float { bits } => self.lower_float_ty(*bits),
			_ => unreachable!()
		};

		llvm_ty.const_float(n)
	}

	fn lower_call_value(
		&self,
		function: &RValue,
		args: &Vec<RValue>,
		context: FunctionContext<'a, 'ctx>) -> BasicValueEnum<'ctx>
	{
		let args: Vec<BasicMetadataValueEnum>
			= args.iter()
				  .map(|arg| self.lower_rvalue(arg, context).into())
				  .collect_vec();

		match function.kind()
		{
			RValueKind::Function { is_extern, name } => {
				let function = self.module.get_function(name).unwrap();

				self.builder.build_call(function, &args, "call")
					.try_as_basic_value()
					.left()
					.unwrap_or_else(|| self.context.struct_type(&[], false).const_named_struct(&[]).into())
			}

			_ => {
				let llvm_function = self.lower_rvalue(function, context);

				if !llvm_function.is_pointer_value() {
					panic!()
				}

				let llvm_function = llvm_function.into_pointer_value();
				let callable_function = CallableValue::try_from(llvm_function).unwrap();

				self.builder.build_call(callable_function, &args, "call")
					.try_as_basic_value()
					.left()
					.unwrap_or_else(|| self.context.struct_type(&[], false).const_named_struct(&[]).into())
			}
		}
	}
}