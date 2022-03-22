#![feature(let_else)]

#[cfg(test)]
mod tests;
mod variant;
mod table;

pub use table::TypeTable;

use std::{collections::HashMap};

use blir::{code::{CodeBlock, Statement, StatementKind}, typ::{Type, TypeKind}, value::{Value, ValueKind, IfValue, IfBranch}, scope::ScopeRef, Symbol};
use rusttyc::{VarlessTypeChecker, TcKey};
use variant::TypeVariant;

pub struct TypeInferCtx {
	checker: VarlessTypeChecker<TypeVariant>,
	infer_keys: HashMap<u64, TcKey>
}

impl TypeInferCtx {
	pub fn new() -> TypeInferCtx {
		TypeInferCtx {
			checker: VarlessTypeChecker::new(),
			infer_keys: HashMap::new()
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

	pub fn infer_codeblock(&mut self, block: &mut CodeBlock, ty: &Type, scope: &ScopeRef) {
		// Add a constraint between ty and the block's code
		let block_ty = block.typ();

		self.constrain_two_way(&block_ty, ty);

		for smt in block.statements_mut() {
			self.infer_smt(smt, scope)
		}
	}

	fn infer_smt(&mut self, smt: &mut Statement, scope: &ScopeRef) {
		match &mut smt.kind {
			StatementKind::Bind { name: _, typ, value } => {
				if let Some(value) = value.as_mut() {
					self.constrain_two_way(typ, &value.typ);
					self.infer_value(value, scope);
				}
			}

			StatementKind::Eval { value, escaped: _ } => {
				self.infer_value(value, scope);
			}

			_ => {}
		}
	}

	fn infer_value(&mut self, value: &mut Value, scope: &ScopeRef) {
		match &mut value.kind {
			ValueKind::BoolLiteral(_) => self.constrain_bool(&value.typ),
			ValueKind::IntLiteral(_) => self.constrain_integer(&value.typ),
			ValueKind::FloatLiteral(_) => self.constrain_float(&value.typ),

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
						self.constrain_one_way(&arg.typ, param_ty);
					}

					self.constrain_one_way(&value.typ, return_type.as_ref())
				}
			}

			ValueKind::Named(name) => {
				let Some(sym) = scope.lookup_symbol(name.as_str()) else {
					// ERROR: Could not find symbol
					return;
				};

				match sym.resolve() {
					Symbol::Value(resolved_value) => {
						value.set_kind(resolved_value.kind);
						value.set_type(resolved_value.typ);
					}
					Symbol::Function(function) => {
						value.set_type(function.take_typ());
						value.set_kind(ValueKind::StaticFunc(function));
					}
					_ => {
						// ERROR: Symbol isn't a value
						return;
					}
				}
			}

			ValueKind::If(if_value) => self.infer_if_value(if_value, scope, &value.typ),

			_ => {}
		}
	}

	fn infer_if_value(&mut self, if_value: &mut IfValue, scope: &ScopeRef, typ: &Type) {
		self.infer_value(if_value.condition.as_mut(), scope);

		self.infer_codeblock(&mut if_value.positive, typ, scope);

		if let Some(else_block) = if_value.negative.as_mut() {
			match else_block {
				IfBranch::CodeBlock(code_block) => self.infer_codeblock(code_block, typ, scope),
				IfBranch::Else(else_branch) => self.infer_if_value(else_branch, scope, typ),
			}
		}
	}

	fn constrain_two_way(&mut self, one: &Type, two: &Type) {
		if let Some((key1, key2)) = self.get_infer_key(one).zip(self.get_infer_key(two)) {
			self.checker
				.impose(key1.equate_with(key2))
				.unwrap();
		} else if let Some(key1) = self.get_infer_key(one) {
			if let Some(variant2) = self.get_variant(two) {
				self.checker
					.impose(key1.concretizes_explicit(variant2))
					.unwrap();
			}
		} else if let Some(key2) = self.get_infer_key(two) {
			if let Some(variant1) = self.get_variant(one) {
				self.checker
					.impose(key2.concretizes_explicit(variant1))
					.unwrap();
			}
		}
	}

	fn constrain_one_way(&mut self, constrain: &Type, absolute: &Type) {
		if let Some((c_key, a_key)) = self.get_infer_key(constrain).zip(self.get_infer_key(absolute)) {
			self.checker
				.impose(c_key.concretizes(a_key))
				.unwrap();
		} else if let Some(c_key) = self.get_infer_key(constrain) {
			if let Some(a_variant) = self.get_variant(absolute) {
				self.checker
					.impose(c_key.concretizes_explicit(a_variant))
					.unwrap();
			}
		}
	}

	fn constrain_integer(&mut self, ty: &Type) {
		if let Some(key) = self.get_infer_key(ty) {
			self.checker
				.impose(key.concretizes_explicit(TypeVariant::SomeInteger))
				.unwrap();
		}
	}

	fn constrain_bool(&mut self, ty: &Type) {
		if let Some(key) = self.get_infer_key(ty) {
			self.checker
				.impose(key.concretizes_explicit(TypeVariant::SomeBoolean))
				.unwrap();
		}
	}

	fn constrain_float(&mut self, ty: &Type) {
		if let Some(key) = self.get_infer_key(ty) {
			self.checker
				.impose(key.concretizes_explicit(TypeVariant::SomeFloat))
				.unwrap();
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
			TypeKind::Integer { bits } => TypeVariant::IntrinsicInteger { bits: *bits },
			TypeKind::Float { bits } => TypeVariant::IntrinsicFloat { bits: *bits },

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