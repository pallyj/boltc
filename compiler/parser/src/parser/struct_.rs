use rowan::Checkpoint;

use crate::lexer::SyntaxKind;

use super::Parser;

impl<'a> Parser<'a> {
	pub fn parse_struct(&mut self, checkpoint: Checkpoint) -> bool {
		if !self.eat_and_start_node_at(SyntaxKind::StructKw, SyntaxKind::StructDef, checkpoint) {
			return false;
		}

		if !self.eat(SyntaxKind::Ident) {
			// Recover
			self.bump()
		}

		self.parse_delim(
			SyntaxKind::StructBody,
			SyntaxKind::OpenBrace,
			SyntaxKind::CloseBrace,
			|parser| parser.parse_struct_item());

		self.finish_node();

		return true;
	}

	pub fn parse_struct_item(&mut self) {
		let checkpoint = self.builder.checkpoint();

		self.parse_visibility();

		if self.parse_func(checkpoint) {

		} else if self.parse_var(checkpoint) {

		} else if self.parse_let(checkpoint) {

		} else if self.parse_struct(checkpoint) {
			
		} else {
			self.bump();
		}
	}
}