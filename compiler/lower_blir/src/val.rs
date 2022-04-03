use std::panic;

use blir::{value::{Value, ValueKind, IfValue, IfBranch}, typ::{Type, TypeKind}, intrinsics::{BinaryIntrinsicFn, UnaryIntrinsicFn}, code::FunctionRef};
use blirssa::value::{LabelValue, BinaryIntrinsicFn as SsaBinaryIntrinsicFn, UnaryIntrinsicFn as SsaUnaryIntrinsicFn};

use crate::BlirLowerer;

impl BlirLowerer {
	pub (super) fn lower_value(&mut self, value: &Value) -> LabelValue {
		match &value.kind {
			ValueKind::IntLiteral(n) => self.lower_int_literal(*n, &value.typ),
			ValueKind::FloatLiteral(n) => self.lower_float_literal(*n, &value.typ),
			ValueKind::BoolLiteral(b) => self.lower_bool_literal(*b, &value.typ),

			ValueKind::FuncCall { function, args } => {
				let lowered_args = args.args.iter()
					.map(|arg| self.lower_value(arg))
					.collect();

				self.lower_func_call(function.as_ref(), lowered_args)
			}

			ValueKind::LocalVariable(local_var_name) => {
				self.context
					.lookup_var(local_var_name)
					.cloned()
					.unwrap()
			}

			ValueKind::FunctionParam(param_name) => {
				self.context
					.lookup_var(param_name)
					.cloned()
					.unwrap()
			}

			ValueKind::SelfVal => {
				self.context
					.lookup_var("self")
					.cloned()
					.unwrap()
			}

			ValueKind::If(if_value) => self.lower_if_value(if_value, &value.typ),

			ValueKind::InstanceVariable { reciever, var } => {
				self.lower_field_access(reciever.as_ref(), &var.borrow().name)
			}

			ValueKind::StaticFunc(func) => self.lower_static_func(func),

			_ => panic!("{value:?}"),
		}
	}

	fn lower_static_func(&mut self, func: &FunctionRef) -> LabelValue {
		let static_func = self.ssa_library()
			.get_function(&func.borrow().link_name)
			.cloned()
			.unwrap();

		let function = self.builder().build_function(&static_func);

		self.builder().build_function_pointer(function)
	}

	fn lower_int_literal(&mut self, n: u64, ty: &Type) -> LabelValue {
		match ty.kind() {
			TypeKind::Integer { bits } => self.builder().build_integer_literal(*bits as u32, n),
			TypeKind::Struct(r#struct) => {
				// TODO: Do this by insert value
				if !r#struct.integer_repr() {
					panic!()
				}

				let borrowed_struct = r#struct.borrow();
				let borrowed_var = borrowed_struct.instance_vars[0].borrow();

				let literal = self.lower_int_literal(n, &borrowed_var.typ);

				self.lower_init(ty, vec![literal])
			}
			other => panic!("{other:?}"),
		}
	}
	
	fn lower_float_literal(&mut self, n: f64, ty: &Type) -> LabelValue {
		match ty.kind() {
			TypeKind::Float { bits } => self.builder().build_float_literal(*bits as u32, n),
			TypeKind::Struct(r#struct) => {
				// TODO: Do this by insert value
				if !r#struct.float_repr() {
					panic!()
				}

				let borrowed_struct = r#struct.borrow();
				let borrowed_var = borrowed_struct.instance_vars[0].borrow();

				let literal = self.lower_float_literal(n, &borrowed_var.typ);

				self.lower_init(ty, vec![literal])
			}
			_ => panic!(),
		}
	}
	
	fn lower_bool_literal(&mut self, b: bool, ty: &Type) -> LabelValue {
		match ty.kind() {
			TypeKind::Integer { bits: 1 } => self.builder().build_integer_literal(1, if b { 1 } else { 0 }),
			TypeKind::Struct(r#struct) => {
				// TODO: Do this by insert value
				if !r#struct.bool_repr() {
					panic!()
				}

				let borrowed_struct = r#struct.borrow();
				let borrowed_var = borrowed_struct.instance_vars[0].borrow();

				let literal = self.lower_bool_literal(b, &borrowed_var.typ);

				self.lower_init(ty, vec![literal])
			}
			_ => panic!(),
		}
	}
	
	fn lower_func_call(&mut self, func: &Value, mut args: Vec<LabelValue>) -> LabelValue {
		match &func.kind {
			ValueKind::ExternFunc(extern_func) => {
				let extern_func = self.ssa_library()
					.get_extern_function(&extern_func.borrow().name)
					.cloned()
					.unwrap();

				let function = self.builder().build_extern_function(&extern_func);
				self.builder().build_call(function, args)
			}

			ValueKind::StaticFunc(function) => {
				let static_func = self.ssa_library()
					.get_function(&function.borrow().link_name)
					.cloned()
					.unwrap();

				let function = self.builder().build_function(&static_func);
				self.builder().build_call(function, args)
			}

			ValueKind::InstanceMethod { reciever, method } => {
				let func = self.ssa_library()
					.get_function(&method.borrow().link_name)
					.cloned()
					.unwrap();

				let function = self.builder().build_function(&func);
				let reciever = self.lower_value(&reciever);
				args.insert(0, reciever);
				self.builder().build_call(function, args)
			}

			ValueKind::StaticMethod(function) => {
				let static_func = self.ssa_library()
					.get_function(&function.borrow().link_name)
					.cloned()
					.unwrap();

				let function = self.builder().build_function(&static_func);
				self.builder().build_call(function, args)
			}

			ValueKind::Init(ty) => self.lower_init(ty, args),

			ValueKind::BinaryIntrinsicFn(intrinsic) => {
				let intrinsic = lower_binary_intrinsic(*intrinsic);

				self.builder().build_binary_intrinsic(intrinsic, args[0].clone(), args[1].clone())
			}

			ValueKind::UnaryIntrinsicFn(intrinsic) => {
				let intrinsic = lower_unary_intrinsic(*intrinsic);

				self.builder().build_unary_intrinsic(intrinsic, args[0].clone())
			}

			_ => {
				let function_value = self.lower_value(func);

				match function_value.typ_ref() {
					blirssa::typ::Type::Function { .. } => {},
					blirssa::typ::Type::Pointer { pointee } => match pointee.as_ref() {
						blirssa::typ::Type::Function { .. } => {},
						_ => panic!(),
					},
					_ => panic!(),
				}

				self.builder().build_call(function_value, args)
			}
		}
	}

	fn lower_if_value(&mut self, value: &IfValue, ty: &Type) -> LabelValue {
		if value.negative.is_none() {
			self.lower_if_value_inner(value, None);
			LabelValue::void()
		} else {
			let ty = self.lower_type(ty);

			let assign_val_ptr = self.builder().build_stack_alloc_undef(ty);

			self.lower_if_value_inner(value, Some(assign_val_ptr.clone()));

			let assign_val = self.builder().build_deref(assign_val_ptr);

			assign_val
		}
	}

	fn lower_if_value_inner(&mut self, value: &IfValue, yield_pointer: Option<LabelValue>) {
		let condition = self.lower_value(value.condition.as_ref());

		let positive_block = self.context.function().append_block("positive");

		let finally_block =
		if let Some(negative) = value.negative.as_ref() {
			let negative_block = self.context.function().append_block("negative");
			let finally_block = self.context.function().append_block("finally");

			// Branch to the positive branch if the condition is true
			self.builder().build_branch(condition, &positive_block, &negative_block);

			// Lower the negative branch
			self.builder().position_at_end(&negative_block);
			match &negative {
				IfBranch::CodeBlock(codeblock) => {
					let value_to_assign = self.lower_code_block(codeblock);

					if let Some((yield_pointer, value_to_assign)) = yield_pointer.clone().zip(value_to_assign) {
						self.builder().build_assign_ptr(yield_pointer, value_to_assign);
					}
				}
				IfBranch::Else(else_branch) => self.lower_if_value_inner(else_branch.as_ref(), yield_pointer.clone())
			};
			self.builder().build_always_branch(&finally_block);

			finally_block
		} else {
			let finally_block = self.context.function().append_block("finally");

			// Branch to the positive branch if the condition is true
			self.builder().build_branch(condition, &positive_block, &finally_block);

			finally_block
		};

		// Lower the positive branch
		self.builder().position_at_end(&positive_block);
		let value_to_assign = self.lower_code_block(&value.positive);
		if let Some((yield_pointer, value_to_assign)) = yield_pointer.zip(value_to_assign) {
			self.builder().build_assign_ptr(yield_pointer, value_to_assign);
		}
		self.builder().build_always_branch(&finally_block);

		// Position at the end of the if block
		self.builder().position_at_end(&finally_block);
	}

	fn lower_field_access(&mut self, parent: &Value, field: &str) -> LabelValue {
		let parent = self.lower_value(parent);

		self.builder().build_deref_struct_field(parent, field)
	}

	fn lower_init(&mut self, ty: &Type, args: Vec<LabelValue>) -> LabelValue {
		match ty.kind() {
			TypeKind::Struct(r#struct) => {
				let borrowed_struct = r#struct.borrow();

				let field_names = borrowed_struct.instance_vars.iter().map(|field| field.borrow().name.clone()).collect::<Vec<_>>();

				let struct_typ = self.lower_type(ty);
				let container_literal = self.builder().build_stack_alloc_undef(struct_typ);

				for (field, value) in field_names.iter().zip(args) {
					let field_ptr = self.builder().build_access_struct_field(container_literal.clone(), &field);
					self.builder().build_assign_ptr(field_ptr, value);
				}
		
				self.builder().build_deref(container_literal)
			}
			_ => {
				if args.len() == 1 {
					args[0].clone()
				} else {
					panic!()
				}
			}
		}
	}
}

fn lower_binary_intrinsic(intrinsic: BinaryIntrinsicFn) -> blirssa::value::BinaryIntrinsicFn {
	match intrinsic {
		BinaryIntrinsicFn::IntegerAdd => SsaBinaryIntrinsicFn::IntegerAdd,
		BinaryIntrinsicFn::IntegerSub => SsaBinaryIntrinsicFn::IntegerSub,
		BinaryIntrinsicFn::IntegerMul => SsaBinaryIntrinsicFn::IntegerMul,
		BinaryIntrinsicFn::IntegerDiv => SsaBinaryIntrinsicFn::IntegerDiv,
		BinaryIntrinsicFn::IntegerRem => SsaBinaryIntrinsicFn::IntegerRem,
		BinaryIntrinsicFn::IntegerDivSig => SsaBinaryIntrinsicFn::IntegerDivSig,
		BinaryIntrinsicFn::IntegerRemSig => SsaBinaryIntrinsicFn::IntegerRemSig,
		BinaryIntrinsicFn::IntegerOr => SsaBinaryIntrinsicFn::IntegerOr,
		BinaryIntrinsicFn::IntegerXor => SsaBinaryIntrinsicFn::IntegerXor,
		BinaryIntrinsicFn::IntegerAnd => SsaBinaryIntrinsicFn::IntegerAnd,
		BinaryIntrinsicFn::IntegerShl => SsaBinaryIntrinsicFn::IntegerShl,
		BinaryIntrinsicFn::IntegerShr => SsaBinaryIntrinsicFn::IntegerShr,
		BinaryIntrinsicFn::IntegerShrSig => SsaBinaryIntrinsicFn::IntegerShrSig,
		BinaryIntrinsicFn::IntegerCmpEq => SsaBinaryIntrinsicFn::IntegerCmpEq,
		BinaryIntrinsicFn::IntegerCmpNeq => SsaBinaryIntrinsicFn::IntegerCmpNeq,
		BinaryIntrinsicFn::IntegerCmpLt => SsaBinaryIntrinsicFn::IntegerCmpLt,
		BinaryIntrinsicFn::IntegerCmpGt => SsaBinaryIntrinsicFn::IntegerCmpGt,
		BinaryIntrinsicFn::IntegerCmpLte => SsaBinaryIntrinsicFn::IntegerCmpLte,
		BinaryIntrinsicFn::IntegerCmpGte => SsaBinaryIntrinsicFn::IntegerCmpGte,
		BinaryIntrinsicFn::IntegerCmpLtSig => SsaBinaryIntrinsicFn::IntegerCmpLtSig,
		BinaryIntrinsicFn::IntegerCmpGtSig => SsaBinaryIntrinsicFn::IntegerCmpGtSig,
		BinaryIntrinsicFn::IntegerCmpLteSig => SsaBinaryIntrinsicFn::IntegerCmpLteSig,
		BinaryIntrinsicFn::IntegerCmpGteSig => SsaBinaryIntrinsicFn::IntegerCmpGteSig,
		BinaryIntrinsicFn::FloatAdd => SsaBinaryIntrinsicFn::FloatAdd,
		BinaryIntrinsicFn::FloatSub => SsaBinaryIntrinsicFn::FloatSub,
		BinaryIntrinsicFn::FloatMul => SsaBinaryIntrinsicFn::FloatMul,
		BinaryIntrinsicFn::FloatDiv => SsaBinaryIntrinsicFn::FloatDiv,
		BinaryIntrinsicFn::FloatRem => SsaBinaryIntrinsicFn::FloatRem,
		BinaryIntrinsicFn::FloatCmpEq => SsaBinaryIntrinsicFn::FloatCmpEq,
		BinaryIntrinsicFn::FloatCmpNeq => SsaBinaryIntrinsicFn::FloatCmpNeq,
		BinaryIntrinsicFn::FloatCmpGt => SsaBinaryIntrinsicFn::FloatCmpGt,
		BinaryIntrinsicFn::FloatCmpGte => SsaBinaryIntrinsicFn::FloatCmpGte,
		BinaryIntrinsicFn::FloatCmpLt => SsaBinaryIntrinsicFn::FloatCmpLt,
		BinaryIntrinsicFn::FloatCmpLte => SsaBinaryIntrinsicFn::FloatCmpLte,
	}
}

fn lower_unary_intrinsic(intrinsic: UnaryIntrinsicFn) -> blirssa::value::UnaryIntrinsicFn {
	match intrinsic {
		UnaryIntrinsicFn::IntegerNegate => SsaUnaryIntrinsicFn::IntegerNegate,
		UnaryIntrinsicFn::IntegerInvert => SsaUnaryIntrinsicFn::IntegerInvert,
		UnaryIntrinsicFn::IntegerExtZero16 => SsaUnaryIntrinsicFn::IntegerExt16,
		UnaryIntrinsicFn::IntegerExtZero32 => SsaUnaryIntrinsicFn::IntegerExt32,
		UnaryIntrinsicFn::IntegerExtZero64 => SsaUnaryIntrinsicFn::IntegerExt64,
		UnaryIntrinsicFn::IntegerExtSig16 => SsaUnaryIntrinsicFn::IntegerExt16Sig,
		UnaryIntrinsicFn::IntegerExtSig32 => SsaUnaryIntrinsicFn::IntegerExt32Sig,
		UnaryIntrinsicFn::IntegerExtSig64 => SsaUnaryIntrinsicFn::IntegerExt64Sig,
		UnaryIntrinsicFn::IntegerTrunc8 => SsaUnaryIntrinsicFn::IntegerTrunc8,
		UnaryIntrinsicFn::IntegerTrunc16 => SsaUnaryIntrinsicFn::IntegerTrunc16,
		UnaryIntrinsicFn::IntegerTrunc32 => SsaUnaryIntrinsicFn::IntegerTrunc32,
		UnaryIntrinsicFn::FloatNegate => SsaUnaryIntrinsicFn::FloatNegate,
		UnaryIntrinsicFn::FloatTrunc16 => SsaUnaryIntrinsicFn::FloatTrunc16,
		UnaryIntrinsicFn::FloatTrunc32 => SsaUnaryIntrinsicFn::FloatTrunc32,
		UnaryIntrinsicFn::FloatExt32 => SsaUnaryIntrinsicFn::FloatExt32,
		UnaryIntrinsicFn::FloatExt64 => SsaUnaryIntrinsicFn::FloatExt64,
		UnaryIntrinsicFn::IntegerFromFloat => SsaUnaryIntrinsicFn::FloatToInt,
		UnaryIntrinsicFn::IntegerFromFloatSig => SsaUnaryIntrinsicFn::FloatToIntSig,
		UnaryIntrinsicFn::Float16FromInteger => SsaUnaryIntrinsicFn::IntegerToFloat16,
		UnaryIntrinsicFn::Float32FromInteger => SsaUnaryIntrinsicFn::IntegerToFloat32,
		UnaryIntrinsicFn::Float64FromInteger => SsaUnaryIntrinsicFn::IntegerToFloat64,
		UnaryIntrinsicFn::Float16FromIntegerSig => SsaUnaryIntrinsicFn::IntegerToFloat16Sig,
		UnaryIntrinsicFn::Float32FromIntegerSig => SsaUnaryIntrinsicFn::IntegerToFloat32Sig,
		UnaryIntrinsicFn::Float64FromIntegerSig => SsaUnaryIntrinsicFn::IntegerToFloat64Sig,
	}
}