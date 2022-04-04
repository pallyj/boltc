#![feature(let_else)]

#[cfg(test)]
mod tests;
mod variant;
mod table;

use errors::{debugger::Debugger, error::ErrorCode, Span};
use table::GuessTable;
pub use table::TypeTable;

use std::{collections::HashMap};

use blir::{code::{CodeBlock, Statement, StatementKind}, typ::{Type, TypeKind}, value::{Value, ValueKind, IfValue, IfBranch}, scope::{ScopeRef, ScopeType}, Symbol};
use rusttyc::{VarlessTypeChecker, TcKey, TcErr};
use variant::TypeVariant;

pub struct TypeInferCtx<'a, 'b> {
	checker: VarlessTypeChecker<TypeVariant>,
	infer_keys: HashMap<u64, TcKey>,
	guesses: GuessTable,
	debugger: &'a mut Debugger<'b>
}

impl<'a, 'b> TypeInferCtx<'a, 'b> {
	pub fn new(debugger: &'a mut Debugger<'b>) -> Self {
		TypeInferCtx {
			checker: VarlessTypeChecker::new(),
			infer_keys: HashMap::new(),
			guesses: GuessTable::new(),
			debugger
		}
	}

	pub fn finalize(self) -> TypeTable {
		let mut type_table = TypeTable::new();

		let Ok(p_type_table) = self.checker.type_check_preliminary() else {
			return type_table;
		};

		for (key, tc_key) in self.infer_keys {
			if let Some(lookup) = p_type_table.get(&tc_key) {
				type_table.insert_variant(key, lookup.variant.clone());
			}
		}

		type_table
	}

	pub fn infer_rel(&mut self, value: &mut Value, typ: &Type, scope: &ScopeRef) {
		self.infer_value(value, scope);

		self.constrain_value_two_way(&value, typ);
	}

	pub fn infer_codeblock(&mut self, block: &mut CodeBlock, ty: &Type, scope: &ScopeRef, span: &Option<Span>) {
		// Add a constraint between ty and the block's code
		let block_ty = block.typ();

		// TODO: Set the span to the value's span
		match self.constrain_two_way(&block_ty, ty) {
			Ok(()) => {},
			_ => {
				let expected = format!("{block_ty:?}");
				self.debugger.throw_single(ErrorCode::MismatchedType { expected }, span)
			}
		}

		let block_scope = ScopeRef::new(Some(scope), blir::scope::ScopeRelation::SameContainer, ScopeType::Code, false, false);

		for smt in block.statements_mut() {
			self.infer_smt(smt, &block_scope)
		}
	}

	fn infer_smt(&mut self, smt: &mut Statement, scope: &ScopeRef) {
		match &mut smt.kind {
			StatementKind::Bind { name: _, typ, value } => {
				if let Some(value) = value.as_mut() {
					self.constrain_value_two_way(&value, typ);
					self.infer_value(value, scope);
				}
			}

			StatementKind::Eval { value, escaped: _ } => {
				self.infer_value(value, scope);
			}

			StatementKind::Return { value } => {
				let Some(return_value) = &value else {
					return
				};

				let function_return_type = scope.scope_type("return")
					.expect("Compiler Error: Function return type is not defined");

				self.constrain_value_one_way(&return_value, &function_return_type);
			}
		}
	}

	fn infer_value(&mut self, value: &mut Value, scope: &ScopeRef) {
		match &mut value.kind {
			ValueKind::BoolLiteral(_) => match self.constrain_bool(&value.typ) {
				Err(_) => {
					self.debugger.throw_single(ErrorCode::TypeIsNotABool, &value.span);
				}
				_ => {}
			}
			ValueKind::IntLiteral(_) => {
				match self.constrain_integer(&value.typ) {
					Err(_) => {
						self.debugger.throw_single(ErrorCode::TypeIsNotAnInteger, &value.span);
					}
					_ => {}
				}
			}
			ValueKind::FloatLiteral(_) => {
				match self.constrain_float(&value.typ) {
					Err(_) => {
						self.debugger.throw_single(ErrorCode::TypeIsNotAFloat, &value.span);
					}
					_ => {}
				}
			}

			ValueKind::FuncCall { function, args } => {
				self.infer_value(function.as_mut(), scope);

				args.args
					.iter_mut()
					.for_each(|arg| self.infer_value(arg, scope));

				if let TypeKind::Function { return_type, params, labels: _ } = function.as_ref().typ.kind() {
					// Ensure params and args are the same length
					if params.len() != args.args.len() {
						return;
					}

					// Match function parameters against the args
					for (arg, param_ty) in args.args.iter().zip(params.iter()) {
						self.constrain_value_one_way(&arg, param_ty);
					}

					let return_ty = return_type.clone();

					self.constrain_value_one_way(&value, return_ty.as_ref())
				} else if let TypeKind::Method { reciever: _, return_type, params } = function.as_ref().typ.kind() {
					let return_ty = return_type.clone();

					// Ensure params and args are the same length
					if params.len() != args.args.len() {
						return;
					}

					// Match function parameters against the args
					for (arg, param_ty) in args.args.iter().zip(params.iter()) {
						self.constrain_value_one_way(arg, param_ty);
					}

					self.constrain_value_one_way(&value, return_ty.as_ref());
				} else if let TypeKind::Metatype(t) = function.as_ref().typ.kind() {
					let ty = t.clone().anon();

					let initializer = t.clone().anon().init_type().anon();

					let TypeKind::Function { return_type: _, params, labels: _ } = initializer.kind() else {
						return;
					};

					// Ensure params and args are the same length
					if params.len() != args.args.len() {
						return;
					}

					// Match function parameters against the args
					for (arg, param_ty) in args.args.iter().zip(params.iter()) {
						self.constrain_value_one_way(&arg, param_ty);
					}

					self.constrain_value_one_way(&function, &initializer);
					function.set_type(initializer);
					self.constrain_value_one_way(&value, &ty);
				}
			}

			ValueKind::Named(name) => {
				let name = name.clone();

				if let Some(sym) = scope.lookup_symbol(name.as_str()) {
					match sym.resolve() {
						Symbol::Type(ty) => {
							value.set_kind(ValueKind::Metatype(ty.clone()));
	
							self.constrain_value_one_way(&value, &TypeKind::Metatype(Box::new(ty.clone())).anon());
	
							value.typ.set_kind(TypeKind::Metatype(Box::new(ty)));
						}
						Symbol::Value(resolved_value) => {
							value.set_kind(resolved_value.kind);
							
							let typ = resolved_value.typ;
	
							self.constrain_value_one_way(&value, &typ);
							value.set_type(typ);
						}
						Symbol::Function(function) => {
							let typ = function.take_typ();
	
							self.constrain_value_one_way(&value, &typ);
							value.set_type(typ);
	
							value.set_kind(ValueKind::StaticFunc(function));
						}
						Symbol::ExternFunction(function) => {
							let typ = function.take_typ();
	
							self.constrain_value_one_way(&value, &typ);
							value.set_type(typ);
	
							value.set_kind(ValueKind::ExternFunc(function));
						}
						Symbol::InstanceVariable(var) => {
							if let Some(self_type) = scope.scope_type("self") {
								let myself = ValueKind::SelfVal.anon(self_type.clone());

								let typ = var.borrow().typ.clone();
		
								self.constrain_value_one_way(&value, &typ);
								value.set_type(typ);
		
								let kind = ValueKind::InstanceVariable {
									reciever: Box::new(myself),
									var: var.clone() };
								value.set_kind(kind);
							} else {
								println!("Compiler error: found instance variable in a context without self");
							}
						}
						_ => {
							self.debugger.throw_single(ErrorCode::SymNotAValue { name: name.clone() }, &value.span);
						}
					}
					return
				}

				if let Some(self_type) = scope.scope_type("self") {
					if let Some(static_symbol) = self_type.lookup_static_item(&name) {
						match static_symbol {
							Symbol::StaticMethod(method) => {
								let typ = method.take_typ();
		
								self.constrain_value_one_way(&value, &typ);
								value.set_type(typ);
		
								value.set_kind(ValueKind::StaticMethod(method));
							}

							_ => {
								self.debugger.throw_single(ErrorCode::SymNotAValue { name: name.clone() }, &value.span);
							}
						}
					}
				}

				// Error: can't find symbol
			}

			ValueKind::Member { parent, member } => {
				self.infer_value(parent.as_mut(), scope);

				let Some(parent_type) = (match parent.typ.kind() {
					TypeKind::Infer { key } => self.guesses.get(key),
					_ => Some(&parent.typ),
				}) else {
					self.debugger.throw_single(ErrorCode::AmbiguousTy, &parent.span);
					return;
				};

				let Some(sym) = parent_type.lookup_instance_item(member.as_str(), scope) else {
					self.debugger.throw_single(ErrorCode::MemberNotFound { name: member.clone() }, &value.span);
					return
				};

				match sym {
					Symbol::Type(ty) => {
						value.set_kind(ValueKind::Metatype(ty.clone()));
						let typ = TypeKind::Metatype(Box::new(ty)).anon();

						self.constrain_value_one_way(&value, &typ);
						value.set_type(typ);
					}
	
					Symbol::Value(res_val) => {
						value.set_kind(res_val.kind);
						let typ = res_val.typ.clone();

						self.constrain_value_one_way(&value, &typ);
						value.set_type(typ);

					}
	
					Symbol::StaticMethod(method) => {
						let typ = method.take_typ();

						self.constrain_value_one_way(&value, &typ);
						value.set_type(typ);

						value.set_kind(ValueKind::StaticMethod(method));
					}
	
					Symbol::InstanceMethod(method) => {
						let parent = std::mem::replace(parent.as_mut(), ValueKind::Unit.anon(TypeKind::Void.anon()));

						let typ = method.take_typ();

						self.constrain_value_one_way(&value, &typ);
						value.set_type(typ);

						let kind = ValueKind::InstanceMethod {
							reciever: Box::new(parent),
							method };
						value.set_kind(kind);
					}

					Symbol::InstanceVariable(var) => {
						let parent = std::mem::replace(parent.as_mut(), ValueKind::Unit.anon(TypeKind::Void.anon()));

						let typ = var.borrow().typ.clone();

						self.constrain_value_one_way(&value, &typ);
						value.set_type(typ);

						let kind = ValueKind::InstanceVariable {
							reciever: Box::new(parent),
							var: var.clone() };
						value.set_kind(kind);
					}

					Symbol::Constant(res_constant) => {
						let res_val = res_constant.borrow().value.clone();

						value.set_kind(res_val.kind);
						let typ = res_val.typ.clone();

						self.constrain_value_one_way(&value, &typ);
						value.set_type(typ);

					}
	
					_ => {
						self.debugger.throw_single(ErrorCode::MemberNotAVal { name: member.clone() }, &value.span);
					}
				}
			}

			ValueKind::If(if_value) => self.infer_if_value(if_value, scope, &value.typ),

			_ => {}
		}
	}

	fn infer_if_value(&mut self, if_value: &mut IfValue, scope: &ScopeRef, typ: &Type) {
		self.infer_value(if_value.condition.as_mut(), scope);

		let span = if_value.positive.span().cloned();

		self.infer_codeblock(&mut if_value.positive, typ, scope, &span);

		if let Some(else_block) = if_value.negative.as_mut() {
			match else_block {
				IfBranch::CodeBlock(code_block) => {
					let span = code_block.span().cloned();
					self.infer_codeblock(code_block, typ, scope, &span)
				}
				IfBranch::Else(else_branch) => self.infer_if_value(else_branch, scope, typ),
			}
		}
	}

	fn constrain_value_two_way(&mut self, val: &Value, typ: &Type) {
		match self.constrain_two_way(&val.typ, typ) {
			Ok(()) => {}
			_ => {
				let expected = format!("{typ:?}");
				self.debugger.throw_single(ErrorCode::MismatchedType { expected }, &val.span)
			}
		}
	}

	fn constrain_two_way(&mut self, one: &Type, two: &Type) -> Result<(), TcErr<TypeVariant>> {
		//println!("{one:?} <=> {two:?}");
		if let Some((key1, key2)) = self.get_infer_key(one).zip(self.get_infer_key(two)) {
			self.checker
				.impose(key1.equate_with(key2))?;
		} else if let Some(key1) = self.get_infer_key(one) {
			if let Some(variant2) = self.get_variant(two) {
				self.checker
					.impose(key1.concretizes_explicit(variant2))?;
			}
		} else if let Some(key2) = self.get_infer_key(two) {
			if let Some(variant1) = self.get_variant(one) {
				self.checker
					.impose(key2.concretizes_explicit(variant1))?;
			}
		}

		// Add it to the guess table
		if let (TypeKind::Infer { key: key1 }, TypeKind::Infer { key: key2 })
			= (one.kind(), two.kind())
		{
			// If either is unresolved, set it to the other
			let (one_t, two_t) = (self.guesses.get(key1), self.guesses.get(key2));

			match (one_t, two_t) {
				(Some(one_t), None) => {
					let one_t = one_t.clone();
					self.guesses.insert(*key2, one_t);
				}

				(None, Some(two_t)) => {
					let two_t = two_t.clone();
					self.guesses.insert(*key1, two_t);
				}

				(None, None) => {
					// Do something
				}

				(Some(_), Some(_)) => {
					// Do nothing
				}
			}
		} else if let TypeKind::Infer { key: key1 } = one.kind() {
			if let TypeKind::Infer { key: abs_key } = two.kind() {
				self.guesses.get(&abs_key)
					.cloned()
					.map(|absolute| self.guesses.insert(*key1, absolute));
			} else {
				self.guesses.insert(*key1, two.clone())
			}
		} else if let TypeKind::Infer { key: key2 } = two.kind() {
			if let TypeKind::Infer { key: abs_key } = one.kind() {
				self.guesses.get(abs_key)
					.cloned()
					.map(|absolute| self.guesses.insert(*key2, absolute));
			} else {
				self.guesses.insert(*key2, one.clone())
			}
		}

		return Ok(())
	}

	fn constrain_value_one_way(&mut self, value: &Value, absolute: &Type) {
		match self.constrain_one_way(&value.typ, absolute) {
			Err(_) => {
				let expected = format!("{absolute:?}");
				self.debugger.throw_single(ErrorCode::MismatchedType { expected }, &value.span);
			}
			_ => {}
		}
	}

	fn constrain_one_way(&mut self, constrain: &Type, absolute: &Type) -> Result<(), TcErr<TypeVariant>> {
		//println!("{constrain:?} <- {absolute:?}");
		if let Some((c_key, a_key)) = self.get_infer_key(constrain).zip(self.get_infer_key(absolute)) {
			self.checker
				.impose(c_key.concretizes(a_key))?
		} else if let Some(c_key) = self.get_infer_key(constrain) {
			if let Some(a_variant) = self.get_variant(absolute) {
				self.checker
					.impose(c_key.concretizes_explicit(a_variant))?
			}
		}

		// Now add it to the guess table
		if let TypeKind::Infer { key } = constrain.kind() {
			if let TypeKind::Infer { key: abs_key } = absolute.kind() {
				self.guesses.get(abs_key)
					.cloned()
					.map(|absolute| self.guesses.insert(*key, absolute));
			} else {
				self.guesses.insert(*key, absolute.clone())
			}
		}

		Ok(())
	}

	fn constrain_integer(&mut self, ty: &Type) -> Result<(), TcErr<TypeVariant>> {
		//println!("{ty:?} <- some Integer");
		if let Some(key) = self.get_infer_key(ty) {
			self.checker
				.impose(key.concretizes_explicit(TypeVariant::SomeInteger))
		} else {
			Ok(())
		}
	}

	fn constrain_bool(&mut self, ty: &Type) -> Result<(), TcErr<TypeVariant>> {
		//println!("{ty:?} <- some Bool");
		if let Some(key) = self.get_infer_key(ty) {
			self.checker
				.impose(key.concretizes_explicit(TypeVariant::SomeBoolean))
		} else {
			Ok(())
		}
	}

	fn constrain_float(&mut self, ty: &Type) -> Result<(), TcErr<TypeVariant>> {
		//println!("{ty:?} <- some Float");
		if let Some(key) = self.get_infer_key(ty) {
			self.checker
				.impose(key.concretizes_explicit(TypeVariant::SomeFloat))
		} else {
			Ok(())
		}
	}

	fn get_infer_key(&mut self, ty: &Type) -> Option<TcKey> {
		let TypeKind::Infer { key } = ty.kind() else {
			return None;
		};

		if let Some(tc_key) = self.infer_keys.get(&key) {
			return Some(*tc_key)
		} else {
			let tc_key = self.checker.new_term_key();

			self.infer_keys.insert(*key, tc_key);

			Some(tc_key)
		}
	}

	fn get_variant(&self, ty: &Type) -> Option<TypeVariant> {
		Some(match ty.kind() {
			TypeKind::Divergent => TypeVariant::Diverges,
			TypeKind::Void => TypeVariant::Void,

			TypeKind::Integer { bits: 1 } => TypeVariant::IntrinsicBool,
			TypeKind::Integer { bits } => TypeVariant::IntrinsicInteger { bits: *bits },
			TypeKind::Float { bits } => TypeVariant::IntrinsicFloat { bits: *bits },

			TypeKind::Struct(r#struct) => TypeVariant::Struct(r#struct.clone()),

			_ => return None
		})
	}
}



// Rules

// let id: infer1 = (expr: infer 2)
// infer1 <-> infer2 (asymmetric)

// apply func(ty1, ty2): ty3 (val1, val2): ty4
// t(val1) <- ty1
// t(val2) <- ty2
// t(val3) <- ty3