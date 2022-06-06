use blir::{value::{Value, ValueKind}, typ::{TypeKind, Type}, intrinsics::{UnaryIntrinsicFn, BinaryIntrinsicFn}};
use itertools::Itertools;
use mir::{val::{RValue, Place, SoloIntrinsic, DuoIntrinsic}};

use crate::BlirLowerer;

impl<'a> BlirLowerer<'a> {
	pub fn is_place(&self, value: &Value) -> bool {
		use ValueKind::*;
		
		// true: its a place
		// false: its an rvalue
		match &value.kind {
			IntLiteral(_) => !matches!(value.typ.kind(), TypeKind::Integer { .. }),
			BoolLiteral(_) => !matches!(value.typ.kind(), TypeKind::Integer { bits: 1 }),
			FloatLiteral(_) => !matches!(value.typ.kind(), TypeKind::Float { .. }),
			Tuple(_) => false,
			Unit => true,

			FuncCall { .. } => true,
			StaticFunc(_) => false,
			StaticMethod(_) => false,

			LocalVariable(_) => true,
			FunctionParam(_) => true,
			SelfVal => true,
 			InstanceVariable { .. } => true,

			TupleField(..) => true,
			
			Assign(_, _) => false,

			_ => panic!("{value:?} is not defined")
		}
	}

	pub fn lower_rvalue(&mut self, value: &Value) -> RValue {
		use ValueKind::*;

		if self.is_place(value) {
			return self.lower_place(value).copy()
		}

		let ty = self.lower_ty(&value.typ);

		match &value.kind {
			IntLiteral(n) => RValue::const_int(*n, ty),
			BoolLiteral(b) => RValue::const_int(if *b { 1 } else { 0 }, ty),
			FloatLiteral(f) => RValue::const_float(*f, ty),
			Tuple(items) => RValue::tuple(items.iter().map(|item| self.lower_rvalue(item)).collect_vec()),
			Unit => RValue::tuple(vec![]),

			StaticFunc(function) => {
				self.builder.build_function(function.borrow().info.link_name())
			},

			Assign(place, value) => {
				let place = self.lower_place(place);
				let value = self.lower_rvalue(value);

				self.builder.build_assign(&place, value);

				RValue::tuple(vec![])
			},

			_ => unreachable!()
		}
	}

	pub fn lower_place(&mut self, value: &Value) -> Place {
		use ValueKind::*;

		if !self.is_place(value) {
			let rvalue = self.lower_rvalue(value);

			let constant_place = self.builder.build_local(rvalue.ty().clone());
			self.builder.build_assign(&constant_place, rvalue);

			return constant_place;
		}

		match &value.kind {
			IntLiteral(n) => self.lower_int_literal(*n, &value.typ),

			LocalVariable(name) => self.function_ctx.get(name).unwrap().clone(),
			FunctionParam(name) => self.function_ctx.get(name).unwrap().clone(),
			SelfVal => self.function_ctx.get("self").unwrap().clone(),

			FuncCall { function, args } => {
				let args = args.args.iter()
									.map(|arg| self.lower_rvalue(arg))
									.collect_vec();

				let value = self.lower_function(function, args);
				let place = self.builder.build_local(value.ty());
				self.builder.build_assign(&place, value);

				place
			}

			TupleField(place, index) => self.lower_place(&place).tuple_item(*index),
			InstanceVariable { reciever, var } => {
				let parent = self.lower_place(reciever);

				let var_borrowed = var.borrow();
				let field_name = &var_borrowed.name;
				let field_ty = self.lower_ty(&var_borrowed.typ);

				parent.field(field_name, field_ty)
			}

			_ => todo!()
		}
	}

	fn lower_int_literal(
		&mut self,
		n: u64,
		ty: &Type) -> Place
	{
		let mir_ty = self.lower_ty(ty);
		let place = self.builder.build_local(mir_ty);

		let TypeKind::Struct(struct_ref) = ty.kind() else {
			panic!()
		};
		
		if !struct_ref.integer_repr() {
			panic!()
		}

		let borrowed_struct = struct_ref.borrow();
		let borrowed_var = borrowed_struct.instance_vars[0].borrow();

		let field_ty_blir = borrowed_var.typ.clone();
		let field_ty = self.lower_ty(&field_ty_blir);
		let literal = RValue::const_int(n, field_ty);
		let field_name = borrowed_var.name.clone();

		let Some(init) = struct_ref.initializer(vec![Some(field_name)], vec![field_ty_blir]) else {
			panic!();
		};

		let func_call = self.builder.build_function(init.borrow().info.link_name()).call(vec![place.get_ref(), literal]);
		self.builder.build_eval(func_call);

		place
	}

	fn lower_function(&mut self, function: &Value, args: Vec<RValue>) -> RValue {
		use ValueKind::*;
		
		match &function.kind {
			UnaryIntrinsicFn(intrinsic) => {
				let arg0 = args.into_iter().next().unwrap();

				RValue::intrinsic(lower_solo_intrinsic(*intrinsic), arg0)
			}
			BinaryIntrinsicFn(intrinsic) => {
				let mut args_iter = args.into_iter();

				let arg0 = args_iter.next().unwrap();
				let arg1 = args_iter.next().unwrap();

				RValue::intrinsic2(lower_duo_intrinsic(*intrinsic), arg0, arg1)
			}
			StaticFunc(function) => self.builder.build_function(function.borrow().info.link_name()).call(args),
			StaticMethod(method) => self.builder.build_function(method.borrow().info.link_name()).call(args),
			InstanceMethod { reciever, method } => {
				let recv = self.lower_place(&reciever);

				let all_args = std::iter::once(recv.get_ref())
					.chain(args)
					.collect_vec();

				self.builder.build_function(method.borrow().info.link_name()).call(all_args)
			}

			_ => panic!("{function:?} not implemented")
		}
	}
}

fn lower_solo_intrinsic(op: UnaryIntrinsicFn) -> SoloIntrinsic {
	use UnaryIntrinsicFn::*;

	match op {
		IntegerNegate => SoloIntrinsic::INeg,
		IntegerInvert => SoloIntrinsic::IInv,
		IntegerExtZero16 => SoloIntrinsic::IZext16,
		IntegerExtZero32 => SoloIntrinsic::IZext32,
		IntegerExtZero64 => SoloIntrinsic::IZext64,
		IntegerExtSig16 => SoloIntrinsic::ISext16,
		IntegerExtSig32 => SoloIntrinsic::ISext32,
		IntegerExtSig64 => SoloIntrinsic::ISext64,
		IntegerTrunc8 => SoloIntrinsic::ITrunc8,
		IntegerTrunc16 => SoloIntrinsic::ITrunc16,
		IntegerTrunc32 => SoloIntrinsic::ITrunc32,
		FloatNegate => SoloIntrinsic::FNeg,
		FloatTrunc16 => SoloIntrinsic::FTrunc16,
		FloatTrunc32 => SoloIntrinsic::FTrunc32,
		FloatExt32 => SoloIntrinsic::FExt32,
		FloatExt64 => SoloIntrinsic::FExt64,
		IntegerFromFloat => SoloIntrinsic::FCnvI,
		IntegerFromFloatSig => SoloIntrinsic::FCnvI,
		Float16FromInteger => SoloIntrinsic::ICnvF16,
		Float32FromInteger => SoloIntrinsic::ICnvF32,
		Float64FromInteger => SoloIntrinsic::ICnvF64,
		Float16FromIntegerSig => SoloIntrinsic::ICnvF16Sig,
		Float32FromIntegerSig => SoloIntrinsic::ICnvF32Sig,
		Float64FromIntegerSig => SoloIntrinsic::ICnvF64Sig,
		StrSliceLen => todo!(),
	}
}

fn lower_duo_intrinsic(op: BinaryIntrinsicFn) -> DuoIntrinsic {
	use BinaryIntrinsicFn::*;

	match op {
		IntegerAdd => DuoIntrinsic::IAdd,
		IntegerSub => DuoIntrinsic::ISub,
		IntegerMul => DuoIntrinsic::IMul,
		IntegerDiv => DuoIntrinsic::IDiv,
		IntegerRem => DuoIntrinsic::IRem,
		IntegerDivSig => DuoIntrinsic::IDivSig,
		IntegerRemSig => DuoIntrinsic::IRemSig,
		IntegerOr => DuoIntrinsic::IOr,
		IntegerXor => DuoIntrinsic::IXor,
		IntegerAnd => DuoIntrinsic::IAnd,
		IntegerShl => DuoIntrinsic::IShl,
		IntegerShr => DuoIntrinsic::IShr,
		IntegerShrSig => DuoIntrinsic::IShrSig,
		IntegerCmpEq => DuoIntrinsic::ICmpEq,
		IntegerCmpNeq => DuoIntrinsic::ICmpNeq,
		IntegerCmpLt => DuoIntrinsic::ICmpLt,
		IntegerCmpGt => DuoIntrinsic::ICmpGt,
		IntegerCmpLte => DuoIntrinsic::ICmpLte,
		IntegerCmpGte => DuoIntrinsic::ICmpGte,
		IntegerCmpLtSig => DuoIntrinsic::ICmpLtSig,
		IntegerCmpGtSig => DuoIntrinsic::ICmpGtSig,
		IntegerCmpLteSig => DuoIntrinsic::ICmpLteSig,
		IntegerCmpGteSig => DuoIntrinsic::ICmpGteSig,
		FloatAdd => DuoIntrinsic::FAdd,
		FloatSub => DuoIntrinsic::FSub,
		FloatMul => DuoIntrinsic::FMul,
		FloatDiv => DuoIntrinsic::FDiv,
		FloatRem => DuoIntrinsic::FRem,
		FloatCmpEq => DuoIntrinsic::FCmpEq,
		FloatCmpNeq => DuoIntrinsic::FCmpNeq,
		FloatCmpGt => DuoIntrinsic::FCmpGt,
		FloatCmpGte => DuoIntrinsic::FCmpGte,
		FloatCmpLt => DuoIntrinsic::FCmpLt,
		FloatCmpLte => DuoIntrinsic::FCmpLte,
	}
}

/*

 there are two methods:
 
 copy gets an rvalue
 
 if the value is a constant, then copy just returns a constant value
 otherwise, evaluates a place and then moves it.
 
 ref gets a pointer rvalue
 
 
 place gets a place
 
 if the value is a constant, then create a place for it and call copy to get the value to put

 */