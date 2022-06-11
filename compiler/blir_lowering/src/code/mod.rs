use blir::code::{Statement, StatementKind, CodeBlock};
use mir::{val::RValue, instr::Terminator};

use crate::BlirLowerer;

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
			Bind { name, value, typ } => {
				// Assign a value, if we can
				// todo: check if these are mutable
				if let Some(value) = value {
					let ty = self.lower_ty(&typ);
					let place = self.builder.build_local(ty, false, Self::span_of(smt.span().cloned()));
					self.lower_assign(&place, &value);
					self.function_ctx.insert(name.clone(), place);
				} else {
					let ty = self.lower_ty(&typ);
					let place = self.builder.build_local(ty, false, Self::span_of(smt.span().cloned()));
					self.function_ctx.insert(name.clone(), place);
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
		}
	}
}