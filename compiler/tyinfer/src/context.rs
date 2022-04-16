use std::collections::HashMap;

use blir::{code::{CodeBlock, Statement, StatementKind}, typ::{Type, TypeKind}, scope::ScopeRef, value::{Value, ValueKind, IfValue, IfBranch}, BlirContext};
use errors::{debugger::Debugger, error::ErrorCode};
use rusttyc::{VarlessTypeChecker, TcKey, TcErr};

use crate::{variant::TypeVariant, replace::TypeReplaceContext};

pub struct TypeInferContext<'a, 'b> {
	checker: 	VarlessTypeChecker<TypeVariant>,
	infer_keys: HashMap<u64, TcKey>,
	debugger: 	&'a mut Debugger<'b>,
	context:	&'a BlirContext,
}

impl<'a, 'b> TypeInferContext<'a, 'b> {
	pub fn new(debugger: &'a mut Debugger<'b>, context: &'a BlirContext) -> Self {
		Self { checker:    VarlessTypeChecker::new(),
			   infer_keys: HashMap::new(),
			   debugger,
			   context }
	}

	pub fn replace<'c>(&'c mut self) -> TypeReplaceContext<'c, 'b> {
		let Ok(constraint_table) = self.checker.clone().type_check_preliminary() else {
			panic!()
		};

		TypeReplaceContext {
			constraint_table,
			infer_keys: &self.infer_keys,
			context: self.context,
			debugger: self.debugger,
			is_final_run: false
		}
	}

	pub fn finish<'c>(&'c mut self) -> TypeReplaceContext<'c, 'b> {
		let Ok(constraint_table) = self.checker.clone().type_check_preliminary() else {
			panic!()
		};

		TypeReplaceContext {
			constraint_table,
			infer_keys: &self.infer_keys,
			context: self.context,
			debugger: self.debugger,
			is_final_run: true
		}
	}

	fn constrain_value(
		&mut self,
		value: &Value,
		scope: &ScopeRef,
	) {
		/*
		Rules


		*/

		//println!("Constraining {value:?}");

		match &value.kind {
			ValueKind::BoolLiteral(_) => self.constrain_bool(&value),
			ValueKind::IntLiteral(_) => self.constrain_int(&value),
			ValueKind::FloatLiteral(_) => self.constrain_float(&value),

			ValueKind::FuncCall { function, args } => {
				let function_type = &function.typ;

				match function_type.kind() {
					TypeKind::Function { params, .. } |
					TypeKind::Method { params, .. } => {
						if self.fully_constrained(&params) {
							for (param, arg) in params.iter().zip(&args.args) {
								self.constrain_value(arg, scope);

								self.constrain_one_way( &arg.typ, param );
							}
						} else {
							// Constrain params to args
							for (param, arg) in params.iter().zip(&args.args) {
								self.constrain_value(arg, scope);

								self.constrain_one_way( param, &arg.typ );
							}
						}
					}

					_ => panic!("{function_type:?}"),
				}
			}

			ValueKind::Member { parent, .. } => {
				println!("Member {parent:?}");
				self.constrain_value(&parent, scope);
			}

			ValueKind::InstanceVariable { reciever, .. } => {
				self.constrain_value(&reciever, scope);
			}

			ValueKind::If(if_value) => self.constrain_if_value(if_value, &value.typ, scope),

			ValueKind::Closure(closure) => {
				// Constrain the closure's return type to the return type of its code
				let closure_return_type = match value.typ.kind() {
					TypeKind::Function { return_type, .. } => return_type,
					_ => panic!(),
				};

				// Infer the code of the codeblock
				// TODO: Make a new scope
				self.infer_codeblock(&closure.code, closure_return_type, scope);
			}

			_ => { }
		}
	}

	fn constrain_if_value(
		&mut self,
		if_value: &IfValue,
		if_type: &Type,
		scope: &ScopeRef)
	{

		self.constrain_value(&if_value.condition, scope);
		self.constrain_bool(&if_value.condition);
		
		self.infer_codeblock(&if_value.positive, if_type, scope);

		match &if_value.negative {
			Some(IfBranch::CodeBlock(else_block)) => {
				self.infer_codeblock(else_block, if_type, scope);
			}
			Some(IfBranch::Else(else_if_block)) => {
				self.constrain_if_value(else_if_block, if_type, scope);
			}
			None => {}
		}
	}

	fn infer_smt(
		&mut self,
		smt: &Statement,
		scope: &ScopeRef)
	{
		match &smt.kind {
			StatementKind::Bind { typ, value, .. } => {
				if let Some(value) = value.as_ref() {
					self.constrain_value(value, scope);

                    self.constrain_two_way(&value.typ, typ);
                }
			}

			StatementKind::Eval { value, .. } => self.constrain_value(value, scope),

			StatementKind::Return { value } => {
				if let Some(return_value) = value.as_ref() {
					let function_return_type =
						scope
							.scope_type("return")
							.expect("Compiler Error: Not in a function scope");

					self.constrain_value(return_value, scope);

					self.constrain_one_way(&return_value.typ, &function_return_type)
				}
			}
		}
	}

	pub fn infer_codeblock(
		&mut self,
		block: &CodeBlock,
		codeblock_type: &Type,
		scope: &ScopeRef)
	{
		let block_implicit_type = block.typ();	

		self.constrain_two_way(&block_implicit_type, codeblock_type);

		// I don't think we need to make a block scope

		for smt in block.statements() {
			self.infer_smt(smt, scope);
		}
	}

	fn constrain_bool(&mut self, value: &Value) {
		//println!("{value:?} <- some Bool");
		if let Some(infer_key) = self.infer_key(&value.typ) {
			let constraint =
				self.checker
					.impose(infer_key.concretizes_explicit(TypeVariant::SomeBool));

			// Match constraint for errors
		}
	}

	fn constrain_int(&mut self, value: &Value) {
		//println!("{value:?} <- some Int");
		if let Some(infer_key) = self.infer_key(&value.typ) {
			let constraint =
				self.checker
					.impose(infer_key.concretizes_explicit(TypeVariant::SomeInteger));

			if constraint.is_ok() {
				return
			}

			self.debugger.throw_single(ErrorCode::TypeIsNotAnInteger, &value.span);

			match constraint.err().unwrap() {
				TcErr::KeyEquation(key1, key2, error) => {
					println!("Incompatible types");
					
				}

				_ => {}
			}

			// Match constraint for errors
		}
	}

	fn constrain_float(&mut self, value: &Value) {
		//println!("{value:?} <- some Float");
		if let Some(infer_key) = self.infer_key(&value.typ) {
			let constraint =
				self.checker
					.impose(infer_key.concretizes_explicit(TypeVariant::SomeFloat));

			// Match constraint for errors
		}
	}

	fn constrain_one_way(
		&mut self,
		constrain: &Type,
		absolute: &Type
	) {
		//println!("{constrain:?} <- {absolute:?}");
		
		if let (TypeKind::Function { return_type: return_type_1, params: params_1, .. },
			TypeKind::Function { return_type: return_type_2, params: params_2, .. }) = (constrain.kind(), absolute.kind())
		{
			self.constrain_one_way(return_type_1, return_type_2);

			for (param1, param2) in params_1.iter().zip(params_2) {
				self.constrain_one_way(param1, param2);
			}
			return
		}

		let Some(constrain_key) = self.infer_key(constrain) else {
			return
		};

		let constraint = if let Some(absolute_key) = self.infer_key(absolute) {
			self.checker.impose(constrain_key.concretizes(absolute_key))
		} else {
			let bound = self.variant(absolute);

			self.checker.impose(constrain_key.concretizes_explicit(bound))
		};

		// Match against error
	}

	fn constrain_two_way(&mut self, ty1: &Type, ty2: &Type) {
		//println!("{ty1:?} <-> {ty2:?}");
		let constraint = match ( self.infer_key(ty1), self.infer_key(ty2) ) {
			(Some(key1), Some(key2))
				=> self.checker.impose(key1.equate_with(key2)),
			(Some(key1), None)
				=> {
					let variant = self.variant(ty2);
					self.checker.impose(key1.concretizes_explicit(variant))
				}
			(None, Some(key2)) => {
				let variant = self.variant(ty1);
				self.checker.impose(key2.concretizes_explicit(variant))
			}
			(None, None) => {
				if let (TypeKind::Function { return_type: return_type_1, params: params_1, .. },
						TypeKind::Function { return_type: return_type_2, params: params_2, .. }) = (ty1.kind(), ty2.kind())
				{
					self.constrain_two_way(return_type_1, return_type_2);

					for (param1, param2) in params_1.iter().zip(params_2) {
						self.constrain_two_way(param1, param2);
					}
				}

				return
			}
		};

		match constraint.err() {
			Some(TcErr::CyclicGraph) => {},

			_ => {}
		}
	}

	fn variant(&self, ty: &Type) -> TypeVariant {
		match ty.kind() {
			TypeKind::Divergent => TypeVariant::Diverges,
			TypeKind::Void => TypeVariant::Void,

			TypeKind::Integer { bits: 1 } => TypeVariant::LlvmBool,
			TypeKind::Integer { bits } => TypeVariant::LlvmInt { bits: *bits as u32 },
			TypeKind::Float { bits } => TypeVariant::LlvmFloat { bits: *bits as u32 },

			TypeKind::Struct(r#struct) => TypeVariant::Struct(r#struct.clone()),

			TypeKind::Function { ..  } => TypeVariant::Function,

			TypeKind::Error => TypeVariant::Error,

			_ => panic!()
		}
	}

	fn infer_key(&mut self, ty: &Type) -> Option<TcKey> {
		let TypeKind::Infer { key } = ty.kind() else {
			return None;
		};

		if let Some(tc_key) = self.infer_keys.get(key) {
			Some(*tc_key)
		} else {
			let new_tc_key = self.checker.new_term_key();

			self.infer_keys.insert(*key, new_tc_key);

			Some(new_tc_key)
		}
	}

	fn fully_constrained(&self, types: &[Type]) -> bool {
		!types.iter()
			.any(|ty| matches!(ty.kind, TypeKind::Infer { .. }))
	}
}