mod flow;

use std::sync::atomic::AtomicU32;

use blir::{value::{Value, ValueKind, Closure}, typ::{TypeKind, Type}, intrinsics::{UnaryIntrinsicFn as Unary, BinaryIntrinsicFn as Binary}};
use errors::Span;
use itertools::Itertools;
use mir::{val::{RValue, Place, SoloIntrinsic, DuoIntrinsic}};
use rand::Rng;

use crate::BlirLowerer;

impl<'a> BlirLowerer<'a> {
	pub fn lower_kind(&self, value: &Value) -> LowerKind {
		use ValueKind::*;
		
		match &value.kind {
			IntLiteral(_) => matches!(value.typ.kind(), TypeKind::Integer { .. }).then_some(LowerKind::Const).unwrap_or(LowerKind::Construct),
			BoolLiteral(_) => matches!(value.typ.kind(), TypeKind::Integer { bits: 1 }).then_some(LowerKind::Const).unwrap_or(LowerKind::Construct),
			FloatLiteral(_) => matches!(value.typ.kind(), TypeKind::Float { .. }).then_some(LowerKind::Const).unwrap_or(LowerKind::Construct),
			StringLiteral(_) => LowerKind::Construct,

			If(if_value) => if matches!(value.typ.kind(), TypeKind::Divergent | TypeKind::Void) || !self.has_else_covered(&if_value.negative) {
				LowerKind::Const
			} else { LowerKind::Construct }

			Match(_) => if matches!(value.typ.kind(), TypeKind::Divergent | TypeKind::Void) {
				LowerKind::Const
			} else { LowerKind::Construct }

			Loop { .. } => if matches!(value.typ.kind(), TypeKind::Divergent | TypeKind::Void) {
				LowerKind::Const
			} else { LowerKind::Construct }
			
			Unit => LowerKind::Const,
			StaticFunc(_) => LowerKind::Const,
			StaticMethod(_) => LowerKind::Const,
			Closure(_) => LowerKind::Const,

			FuncCall { function, .. } => match &function.kind {
				UnaryIntrinsicFn(Unary::RawPointerDeref) => LowerKind::Access,
				UnaryIntrinsicFn(Unary::RawPointerRef) => LowerKind::Const,
				BinaryIntrinsicFn(Binary::ArrayItem) => LowerKind::Access,
				_ => LowerKind::Construct,
			}
			Initializer(_, _) => LowerKind::Construct,
			Tuple(_) => LowerKind::Construct,
			EnumVariant { .. } => LowerKind::Construct,
			SequenceLiteral(_) => LowerKind::Construct,
			RepeatingLiteral {..} => LowerKind::Construct,

			LocalVariable(_, _, _) => LowerKind::Access,
			FunctionParam(_, _) => LowerKind::Access,
			SelfVal(_) => LowerKind::Access,
 			InstanceVariable { .. } => LowerKind::Access,
			TupleField(..) => LowerKind::Access,
			CastEnumToVariant { .. } => LowerKind::Access,
			GlobalVariable(_) => LowerKind::Access,
			
			Assign(_, _) => LowerKind::Const,

			_ => panic!("{value:?} is not defined")
		}
	}

	pub fn lower_assign(&mut self, place: &Place, value: &Value) {
		use LowerKind::*;

		let assign_value = match self.lower_kind(value) {
			Const => {
				self.lower_rvalue_inner(value)
			}

			Access => {
				self.lower_place_inner(value).copy(Self::span_of(value.span))
			}

			Construct => {
				return self.lower_assign_inner(place, value)
			}
		};

		self.builder.build_assign(place, assign_value);
	}

	pub fn lower_rvalue(&mut self, value: &Value) -> RValue {
		use LowerKind::*;

		match self.lower_kind(value) {
			Const => self.lower_rvalue_inner(value),

			Access => self.lower_place_inner(value).copy(Self::span_of(value.span)),

			Construct => {
				let ty = self.lower_ty(&value.typ);
				let place = self.builder.build_local(ty, false, Self::span_of(value.span));

				self.lower_assign_inner(&place, &value);

				place.copy(Self::span_of(value.span))
			}
		}
	}

	pub fn lower_place(&mut self, value: &Value) -> Place {
		use LowerKind::*;

		match self.lower_kind(value) {
			Const => {
				let rvalue = self.lower_rvalue(value);

				let constant_place = self.builder.build_local(rvalue.ty().clone(), false, Self::span_of(value.span));
				self.builder.build_assign(&constant_place, rvalue);

				constant_place
			}

			Access => self.lower_place_inner(value),

			Construct => {
				let ty = self.lower_ty(&value.typ);
				let place = self.builder.build_local(ty, false, Self::span_of(value.span));

				self.lower_assign_inner(&place, &value);

				place
			}
		}
	}

	fn lower_assign_inner(&mut self, place: &Place, value: &Value) {
		use ValueKind::*;

		match &value.kind {
			IntLiteral(n) => self.lower_int_literal(*n, &value.typ, place),
			BoolLiteral(b) => self.lower_bool_literal(*b, &value.typ, place),
			FloatLiteral(n) => self.lower_float_literal(*n, &value.typ, place),
			StringLiteral(s) => self.lower_string_literal(s.clone(), &value.typ, place),

			FuncCall { function, args } => {
				let args = args.args.iter().zip(&args.is_shared)
									.map(|(arg, is_shared)| {
										if *is_shared {
											if !arg.is_mutable() {
												println!("error: value is immutable")
											}
											self.lower_place(arg).get_ref(Self::span_of(arg.span))
										} else {
											self.lower_rvalue(arg)
										}
									})
									.collect_vec();					

				self.lower_function(function, args, place);
			}
			Tuple(items) => {
				for (i, item) in items.iter().enumerate() {
					self.lower_assign(&place.tuple_item(i, Self::span_of(value.span)), item);
				}
			}
			EnumVariant { variant, of_enum } => {
				let tag = variant.tag() as u64;
				let enum_repr = of_enum.repr_type();
				let enum_repr_ty = self.lower_ty(&enum_repr);

				self.builder.build_assign(&place.discriminant(enum_repr_ty.clone(), Span::empty()), RValue::const_int(tag, enum_repr_ty, Span::empty()));
			}

			If(if_value) => self.lower_if_value(if_value, Some(place)),
			Match(match_value) => self.lower_match(match_value, Some(place)),
			Loop { label, code } => {
				self.loop_places.insert(label.clone(), place.clone());
				self.lower_loop(code, &label);
				self.loop_places.remove(label);
			}
			SequenceLiteral(sequence) => {
				for (i, seq_item) in sequence.iter().enumerate() {
					let item_place = place.array_index(RValue::const_int(i as u64, mir::ty::Type::int(64), Self::span_of(value.span)), Self::span_of(seq_item.span));

					self.lower_assign(&item_place, seq_item);
				}
			}

			RepeatingLiteral { repeating, count } => {
				// todo: roll this loop
				for i in 0..count.unwrap() {
					let item_place = place.array_index(RValue::const_int(i, mir::ty::Type::int(64), Self::span_of(value.span)), Self::span_of(repeating.span));

					self.lower_assign(&item_place, &repeating);
				}
			}

			_ => {
				panic!("{value:?}");
			}
		}
	}

	fn lower_rvalue_inner(&mut self, value: &Value) -> RValue {
		use ValueKind::*;

		let ty = self.lower_ty(&value.typ);

		match &value.kind {
			IntLiteral(n) => RValue::const_int(*n, ty, Self::span_of(value.span)),
			BoolLiteral(b) => RValue::const_int(if *b { 1 } else { 0 }, ty, Self::span_of(value.span)),
			FloatLiteral(f) => RValue::const_float(*f, ty, Self::span_of(value.span)),
			Unit => RValue::tuple(vec![], Self::span_of(value.span)),

			StaticFunc(function) => {
				self.builder.build_function(function.borrow().info.link_name(), Self::span_of(value.span))
			},

			Assign(place, value) => {
				if !place.is_mutable() {
					println!("error: {place:?} is not mutable");
				}

				let place = self.lower_place(place);

				self.lower_assign(&place, value);

				RValue::tuple(vec![], Self::span_of(value.span))
			},

			If(if_value) => {
				self.lower_if_value(if_value, None);

				RValue::tuple(vec![], Self::span_of(value.span))
			}

			Match(match_value) => {
				self.lower_match(&match_value, None);

				RValue::tuple(vec![], Self::span_of(value.span))
			}

			Loop { code: loop_value, label } => {
				self.lower_loop(loop_value, label);

				RValue::tuple(vec![], Self::span_of(value.span))
			}

			Closure(closure) => self.lower_closure(closure, &value.typ),

			FuncCall { function, args } => {

				match &function.kind {
					ValueKind::UnaryIntrinsicFn(intrinsic) => match intrinsic {
						Unary::RawPointerRef => self.lower_place(&args.args[0]).get_ref(Self::span_of(value.span)),
						_ => unreachable!(),
					},
					ValueKind::BinaryIntrinsicFn(intrinsic) => match intrinsic {
						Binary::RawPointerAdd => todo!(),
						_ => unreachable!()
					},
					_ => unreachable!(),
				}
			}

			_ => unreachable!()
		}
	}

	fn lower_place_inner(&mut self, value: &Value) -> Place {
		use ValueKind::*;

		match &value.kind {
			LocalVariable(name, _, _) => self.function_ctx.get(name).unwrap().clone(),
			FunctionParam(name, _) => self.function_ctx.get(name).unwrap().clone(),
			SelfVal(_) => self.function_ctx.get("self").unwrap().clone(),

			TupleField(place, index) => self.lower_place(&place).tuple_item(*index, Self::span_of(value.span)),
			InstanceVariable { reciever, var } => {
				let parent = self.lower_place(reciever);

				let var_borrowed = var.borrow();
				let field_name = &var_borrowed.name;
				let field_ty = self.lower_ty(&var_borrowed.typ);

				parent.field(field_name, field_ty, Self::span_of(value.span))
			}

			GlobalVariable(global) => {
				let global_id = self.builder
									.global_id(&global.symbol())
									.unwrap();

				self.builder
					.global(global_id)
					.unwrap()
					.place(Self::span_of(value.span))
			}

			CastEnumToVariant { enum_value, variant } => {
				let enum_place = self.lower_place(enum_value);

				self.builder.build_cast_variant(&enum_place, variant.tag() as u64, variant.name(), Self::span_of(value.span))
			}

			FuncCall { function, args } => {
				match &function.kind {
					ValueKind::UnaryIntrinsicFn(intrinsic) => match intrinsic {
						Unary::RawPointerDeref => self.lower_rvalue(&args.args[0]).deref(Self::span_of(value.span)),
						_ => unreachable!(),
					},
					ValueKind::BinaryIntrinsicFn(Binary::ArrayItem) => {
						let place = self.lower_place(&args.args[0]);
						let index = match args.args[1].typ.kind() {
							TypeKind::Integer { bits: 64 } => { self.lower_rvalue(&args.args[0]) },
							TypeKind::Struct(struct_ref) if struct_ref.integer_repr() => {
								let borrowed_struct = struct_ref.borrow();
								let first = borrowed_struct.instance_vars.first().unwrap().borrow();
								let field_name = &first.name;
								let field_type = self.lower_ty(&first.typ);
								let span = Self::span_of(value.span);
								self.lower_place(&args.args[1])
									.field(&field_name, field_type, span)
									.copy(span)
							}

							_ => {
								panic!("error: cannot index into array with type {}", args.args[1].typ);
							}
						}; //self.lower_rvalue(&args.args[1]);

						place.array_index(index, Self::span_of(value.span))
					}
					_ => unreachable!(),
				}
			}

			_ => {
				let ty = self.lower_ty(&value.typ);
				let place = self.builder.build_local(ty, false, Self::span_of(value.span));

				self.lower_assign(&place, &value);

				return place;
			}
		}
	}

	fn lower_string_literal(
		&mut self,
		s: String,
		ty: &Type,
		place: &Place)
	{
        match ty.kind() {
            TypeKind::Struct(struct_ref) => {
                if struct_ref.string_repr() {
					let borrowed_struct = struct_ref.borrow();

					let borrowed_var_ptr = borrowed_struct.instance_vars[0].borrow();
					let borrowed_var_len = borrowed_struct.instance_vars[1].borrow();
			
					let field_ty_ptr_blir = borrowed_var_ptr.typ.clone();
					let field_ty_len_blir = borrowed_var_len.typ.clone();

					let field_ty_ptr = self.lower_ty(&field_ty_ptr_blir);
					let field_ty_len = self.lower_ty(&field_ty_len_blir);

					let pointer = RValue::const_string(&s, field_ty_ptr, place.span());
					let length = RValue::const_int(s.as_bytes().len() as u64, field_ty_len, place.span());

					let field_name_ptr = borrowed_var_ptr.name.clone();
					let field_name_len = borrowed_var_len.name.clone();
			
					let Some(init) = struct_ref.initializer(vec![Some(field_name_ptr), Some(field_name_len)], vec![field_ty_ptr_blir, field_ty_len_blir]) else {
						panic!();
					};
			
					let func_call = self.builder.build_function(init.borrow().info.link_name(), place.span()).call(vec![place.get_ref(place.span()), pointer, length], place.span());
					self.builder.build_eval(func_call);
                } else if struct_ref.char_repr() {			
					let borrowed_struct = struct_ref.borrow();
					let borrowed_var = borrowed_struct.instance_vars[0].borrow();

					let char_repr = s.chars().next().unwrap() as u32;
			
					let field_ty_blir = borrowed_var.typ.clone();
					let field_ty = self.lower_ty(&field_ty_blir);
					let literal = RValue::const_int(char_repr as u64, field_ty, place.span());
					let field_name = borrowed_var.name.clone();
			
					let Some(init) = struct_ref.initializer(vec![Some(field_name)], vec![field_ty_blir]) else {
						panic!();
					};
			
					let func_call = self.builder.build_function(init.borrow().info.link_name(), place.span()).call(vec![place.get_ref(place.span()), literal], place.span());
					self.builder.build_eval(func_call);
                } else {
                    unreachable!()
                }
            }
            _ => panic!("{ty:?} is not a string"),
        }
    }

	fn lower_int_literal(
		&mut self,
		n: u64,
		ty: &Type,
		place: &Place)
	{
		let TypeKind::Struct(struct_ref) = ty.kind() else {
			panic!("integer with type {ty:?}")
		};
		
		if !struct_ref.integer_repr() {
			panic!()
		}

		let borrowed_struct = struct_ref.borrow();
		let borrowed_var = borrowed_struct.instance_vars[0].borrow();

		let field_ty_blir = borrowed_var.typ.clone();
		let field_ty = self.lower_ty(&field_ty_blir);
		let literal = RValue::const_int(n, field_ty, place.span());
		let field_name = borrowed_var.name.clone();

		let Some(init) = struct_ref.initializer(vec![Some(field_name)], vec![field_ty_blir]) else {
			panic!();
		};

		let func_call = self.builder.build_function(init.borrow().info.link_name(), place.span()).call(vec![place.get_ref(place.span()), literal], place.span());
		self.builder.build_eval(func_call);
	}

	fn lower_bool_literal(
		&mut self,
		b: bool,
		ty: &Type,
		place: &Place)
	{
		let TypeKind::Struct(struct_ref) = ty.kind() else {
			panic!("bool with type {ty:?}")
		};
		
		if !struct_ref.bool_repr() {
			panic!()
		}

		let borrowed_struct = struct_ref.borrow();
		let borrowed_var = borrowed_struct.instance_vars[0].borrow();

		let field_ty_blir = borrowed_var.typ.clone();
		let field_ty = self.lower_ty(&field_ty_blir);
		let literal = RValue::const_int(if b { 1 } else { 0 }, field_ty, place.span());
		let field_name = borrowed_var.name.clone();

		let Some(init) = struct_ref.initializer(vec![Some(field_name)], vec![field_ty_blir]) else {
			panic!();
		};

		let func_call = self.builder.build_function(init.borrow().info.link_name(), place.span()).call(vec![place.get_ref(place.span()), literal], place.span());
		self.builder.build_eval(func_call);
	}

	fn lower_float_literal(
		&mut self,
		n: f64,
		ty: &Type,
		place: &Place)
	{
		let TypeKind::Struct(struct_ref) = ty.kind() else {
			panic!("float with type {ty:?}")
		};
		
		if !struct_ref.float_repr() {
			panic!()
		}

		let borrowed_struct = struct_ref.borrow();
		let borrowed_var = borrowed_struct.instance_vars[0].borrow();

		let field_ty_blir = borrowed_var.typ.clone();
		let field_ty = self.lower_ty(&field_ty_blir);
		let literal = RValue::const_float(n, field_ty, place.span());
		let field_name = borrowed_var.name.clone();

		let Some(init) = struct_ref.initializer(vec![Some(field_name)], vec![field_ty_blir]) else {
			panic!();
		};

		let func_call = self.builder.build_function(init.borrow().info.link_name(), place.span()).call(vec![place.get_ref(place.span()), literal], place.span());
		self.builder.build_eval(func_call);
	}

	fn lower_function(&mut self, function: &Value, args: Vec<RValue>, place: &Place) {
		use ValueKind::*;
		
		let value = match &function.kind {
			UnaryIntrinsicFn(intrinsic) => {
				let arg0 = args.into_iter().next().unwrap();

				RValue::intrinsic(lower_solo_intrinsic(*intrinsic), arg0, Self::span_of(function.span))
			}
			BinaryIntrinsicFn(intrinsic) => {
				let mut args_iter = args.into_iter();

				let arg0 = args_iter.next().unwrap();
				let arg1 = args_iter.next().unwrap();

				RValue::intrinsic2(lower_duo_intrinsic(*intrinsic), arg0, arg1, Self::span_of(function.span))
			}
			StaticFunc(func) => self.builder.build_function(func.borrow().info.link_name(), Self::span_of(function.span)).call(args, place.span()),
			StaticMethod(method) => self.builder.build_function(method.borrow().info.link_name(), Self::span_of(function.span)).call(args, place.span()),
			ExternFunc(extern_func) => self.builder.build_extern_function(extern_func.borrow().info.link_name(), Self::span_of(function.span)).call(args, place.span()),
			InstanceMethod { reciever, method } => {
				let recv = self.lower_place(&reciever);

				if method.is_mutating() && !reciever.is_mutable() {
					println!("error: mutating method called on immutable value");
				}

				// todo: if method takes self ref
				let first_arg = if method.is_mutating() {
					recv.get_ref(recv.span())
				} else {
					recv.copy(recv.span())
				};

				let all_args = std::iter::once(first_arg)
					.chain(args)
					.collect_vec();

				self.builder.build_function(method.borrow().info.link_name(), Self::span_of(function.span)).call(all_args, place.span())
			}
			Initializer(method, _) => {
				let all_args = std::iter::once(place.get_ref(place.span()))
					.chain(args)
					.collect_vec();

				let init_call = self.builder.build_function(method.borrow().info.link_name(), Self::span_of(function.span)).call(all_args, place.span());

				self.builder.build_eval(init_call);

				return
			}
			ValueKind::EnumVariant { of_enum, variant } => {
				// Set the discriminant
				let tag = variant.tag() as u64;
				let enum_repr = of_enum.repr_type();
				let enum_repr_ty = self.lower_ty(&enum_repr);

				self.builder.build_assign(&place.discriminant(enum_repr_ty.clone(), Span::empty()), RValue::const_int(tag, enum_repr_ty, Span::empty()));

				// Cast the enum to its variant
				let enum_tuple = self.builder.build_cast_variant(place, tag, variant.name(), Self::span_of(function.span));

				// Insert each arg in the tuple fields
				for (i, arg) in args.into_iter().enumerate() {
					let tuple_place = enum_tuple.tuple_item(i, arg.span());
					self.builder.build_assign(&tuple_place, arg);
				}

				return

            }

			_ => self.lower_rvalue(function).call(args, place.span()),
		};

		self.builder.build_assign(place, value);
	}

	fn lower_closure(
		&mut self,
		closure: &Closure,
		closure_type: &Type) -> RValue
	{

		static CLOSURE_COUNTER: AtomicU32 = AtomicU32::new(1);

		let closure_index: u32 = CLOSURE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let closure_random_number: u32 = rand::thread_rng().gen_range(0..=(1 << 16 - 1));
		let closure_type_mangled = closure_type.mangle();
		let enclosing_function = self.builder.current_function_mut().name();
        let closure_mangled_name = format!("Cn{enclosing_function}E{closure_type_mangled}_{closure_random_number:4x}{closure_index:x}");

        let mir::ty::TypeKind::Function { parameters, return_type } = self.lower_ty(closure_type).kind() else {
			panic!()
		};

		let function_id = self.builder.add_function(&closure_mangled_name, parameters.to_vec(), return_type.as_ref().clone());		

        self.closures.push((closure_mangled_name, closure.clone()));

		RValue::function(self.builder.get_function_by_id(function_id),
						 Self::span_of(closure.code.span().cloned()))
    }
}

fn lower_solo_intrinsic(op: Unary) -> SoloIntrinsic {
	use Unary::*;

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
		StrSliceLen => panic!(),

		RawPointerDeref => unreachable!(),
		RawPointerRef => unreachable!(),
		RawPointerFromAddr => SoloIntrinsic::AddrCnvPtr,
		RawPointerToAddr => SoloIntrinsic::PtrCnvAddr,
	}
}

fn lower_duo_intrinsic(op: Binary) -> DuoIntrinsic {
	use Binary::*;

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

		ArrayItem => unreachable!(),

		RawPointerAdd => DuoIntrinsic::PtrAdd,
	}
}

pub enum LowerKind {
	Const,
	Construct,
	Access,
}