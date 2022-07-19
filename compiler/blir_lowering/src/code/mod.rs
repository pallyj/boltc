use blir::{code::{Statement, StatementKind, CodeBlock}, value::{ValueKind, match_::MatchValue, MatchBranch}, pattern::PatternKind, typ::TypeKind};
use errors::Span;
use mir::{val::RValue, instr::Terminator};
use patmat::{PatternMatrix};

use crate::{BlirLowerer, err::LoweringErrorKind};

mod func;

impl<'a, 'b> BlirLowerer<'a, 'b> {
	pub fn lower_code_block(
		&mut self,
		code: &CodeBlock) -> Option<RValue>
	{
		let mut last_value = None;

		for smt in code.statements() {
			last_value = self.lower_statement(smt);
			if smt.diverges() { break }
		}

		return last_value;
	}

	///
	/// Lowers a statement to MIR code
	/// 
	/// If the statement returns a value, return that value
	/// so it can be returned
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
						self.reporter.throw_diagnostic(LoweringErrorKind::NoPatternInLet.with_span(pattern.span));
						return None;
					};
					if rows.next().is_some() {
						self.reporter.throw_diagnostic(LoweringErrorKind::SplitPatternInLet.with_span(pattern.span));
						return None;
					}
					// Check that it is refutable
					for col in first_row.columns() {
						// The pattern is refutable
						if !col.matches_any() {
							self.reporter.throw_diagnostic(LoweringErrorKind::RefutablePatternInLet.with_span(pattern.span));

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
					let pattern_matrix = PatternMatrix::construct(ValueKind::Uninit.anon(pattern.match_type.clone()), vec![ pattern.clone() ]).expand();

					let mut rows = pattern_matrix.rows();
					let Some(first_row) = rows.next() else {
						self.reporter.throw_diagnostic(LoweringErrorKind::NoPatternInLet.with_span(pattern.span));
						return None;
					};
					if rows.next().is_some() {
						self.reporter.throw_diagnostic(LoweringErrorKind::SplitPatternInLet.with_span(pattern.span));
						return None;
					}
					// Check that it is refutable
					for col in first_row.columns() {
						// The pattern is refutable
						if !col.matches_any() {
							self.reporter.throw_diagnostic(LoweringErrorKind::RefutablePatternInLet.with_span(pattern.span));
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
			Break(value, label) => {
				if let Some(bb) = self.break_labels.get(label) {
					let bb = *bb;

					if let Some(value) = value {
						let value_to_assign = self.lower_rvalue(value);
						let place = self.loop_places.get(label).unwrap();
						

						self.builder.build_assign(place, value_to_assign);
					}

					self.builder.build_terminator(Terminator::goto(bb));
				} else {
					self.reporter.throw_diagnostic(LoweringErrorKind::LoopDoesNotExist.with_span(smt.span.unwrap_or_default()));

					self.builder.build_terminator(Terminator::panic());
				}
				
				None
			},
			Continue(label) => {
				if let Some(bb) = self.continue_labels.get(label) {
					self.builder.build_terminator(Terminator::goto(*bb));
				} else {
					self.reporter.throw_diagnostic(LoweringErrorKind::LoopDoesNotExist.with_span(smt.span.unwrap_or_default()));
					
					self.builder.build_terminator(Terminator::panic());
				}
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

			Panic => {
				self.builder.build_terminator(Terminator::panic());
				None
			}
		}
	}
}