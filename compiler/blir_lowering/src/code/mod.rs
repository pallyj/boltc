use blir::{code::{Statement, StatementKind, CodeBlock}, value::{ValueKind, match_::MatchValue, MatchBranch}, pattern::PatternKind, typ::TypeKind};
use errors::Span;
use mir::{val::RValue, instr::Terminator};
use patmat::{PatternMatrix, Maranget};

use crate::{BlirLowerer, val};

mod func;

impl<'a> BlirLowerer<'a> {
	pub fn lower_code_block(
		&mut self,
		code: &CodeBlock) -> Option<RValue>
	{
		let mut last_value = None;

		for smt in code.statements() {
			last_value = self.lower_statement(smt);
			if smt.diverges() { break }
		}

		// todo: and then add warnings

		return last_value;
	}

	///
	/// Lowers a statement to MIR code
	/// 
	pub fn lower_statement(
		&mut self,
		smt: &Statement) -> Option<RValue>
	{
		use StatementKind::*;

		match &smt.kind {
			Eval { value, escaped } => (!escaped).then_some(self.lower_rvalue(value)),
			Bind { pattern, value, typ } => {
				// Assign a value, if we can
				// todo: check if these are mutable

				if let Some(value) = value {
					let pattern_matrix = PatternMatrix::construct(value.clone(), vec![ pattern.clone() ]).expand();

					let mut rows = pattern_matrix.rows();
					let Some(first_row) = rows.next() else {
						println!("error: no row in pattern in irrefutable let");
						return None;
					};
					if rows.next().is_some() {
						println!("error: more than one row in pattern in irrefutable let");
						return None;
					}
					// Check that it is refutable
					for col in first_row.columns() {
						// The pattern is refutable
						if !col.matches_any() {
							println!("error: refutable pattern in let binding");

							// Create a binding to prevent errors
							for (bind_name, bind_value) in first_row.bindings() {
								let ty = self.lower_ty(&bind_value.typ);
								let place = self.builder.build_local(ty, false, Self::span_of(bind_value.span.clone()));
								self.function_ctx.insert(bind_name.clone(), place);
							}

							break;
						}
					}

					// Create a binding to each value
					for (bind_name, bind_value) in first_row.bindings() {
						let ty = self.lower_ty(&bind_value.typ);
						let place = self.builder.build_local(ty, false, Self::span_of(bind_value.span.clone()));
						self.lower_assign(&place, &bind_value); // todo: simplify the value
						self.function_ctx.insert(bind_name.clone(), place);
					}
				} else {
					let pattern_matrix = PatternMatrix::construct(ValueKind::Uninit.infer(), vec![ pattern.clone() ]).expand();

					let mut rows = pattern_matrix.rows();
					let Some(first_row) = rows.next() else {
						println!("error: no row in pattern in irrefutable let");
						return None;
					};
					if rows.next().is_some() {
						println!("error: more than one row in pattern in irrefutable let");
						return None;
					}
					// Check that it is refutable
					for col in first_row.columns() {
						// The pattern is refutable
						if !col.matches_any() {
							println!("error: refutable pattern in let binding");
							break;
						}
					}

					// Create a binding to each value
					for (bind_name, bind_value) in first_row.bindings() {
						let ty = self.lower_ty(&bind_value.typ);
						let place = self.builder.build_local(ty, false, Self::span_of(bind_value.span.clone()));
						self.function_ctx.insert(bind_name.clone(), place);
					}

				}

				None
			}
			Return { value } => {
				if let Some(value) = value {
					let value = self.lower_rvalue(value);

					self.builder.build_terminator(Terminator::returns(value));
				} else {
					self.builder.build_terminator(Terminator::return_void());
				}

				None
			},
			Break(label) => {
				let bb = *self.break_labels.get(label).unwrap();
				self.builder.build_terminator(Terminator::goto(bb));
				None
			},
			Continue(label) => {
				let bb = *self.continue_labels.get(label).unwrap();
				self.builder.build_terminator(Terminator::goto(bb));
				None
			}

			Guard { condition, otherwise } => {
				let empty_code_block = CodeBlock::new(vec![], smt.span.unwrap());
				let true_pattern = PatternKind::Integer { value: 1 }.with(Span::empty(), TypeKind::Integer { bits: 1 }.anon());
				let false_pattern = PatternKind::Wildcard.with(Span::empty(), condition.typ.clone());
				
				let guard_coerced_to_match = MatchValue {
					discriminant: condition.clone(),
					branches: vec![
						MatchBranch {
							pattern: true_pattern,
							code: empty_code_block,
						},
						MatchBranch {
							pattern: false_pattern,
							code: otherwise.clone(),
						}
					]
				};

				self.lower_match(&guard_coerced_to_match, None);

				None
			}

			GuardLet { pattern, value, otherwise } => {
				let empty_code_block = CodeBlock::new(vec![], smt.span.unwrap());
				let false_pattern = PatternKind::Wildcard.with(Span::empty(), value.typ.clone());
				
				let guard_coerced_to_match = MatchValue {
					discriminant: Box::new(value.clone()),
					branches: vec![
						MatchBranch {
							pattern: pattern.clone(),
							code: empty_code_block,
						},
						MatchBranch {
							pattern: false_pattern,
							code: otherwise.clone(),
						}
					]
				};

				self.lower_match(&guard_coerced_to_match, None);

				None
			}
		}
	}
}