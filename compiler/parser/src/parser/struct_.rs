use crate::lexer::SyntaxKind;

use super::Parser;

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_struct(&mut self, checkpoint: usize) -> bool {
		if !self.eat_and_start_node_at(SyntaxKind::StructKw, SyntaxKind::StructDef, checkpoint) {
			return false;
		}

		if !self.eat_and_start_node(SyntaxKind::Ident, SyntaxKind::FuncName) {
			// Recover
			self.bump();
		}
		self.finish_node();

		self.parse_delim(
			SyntaxKind::StructBody,
			SyntaxKind::OpenBrace,
			SyntaxKind::CloseBrace,
			|parser| parser.parse_struct_item());

		self.finish_node();

		return true;
	}

	pub fn parse_struct_item(&mut self) {
		let checkpoint = self.checkpoint();

		self.parse_visibility();

		self.eat(SyntaxKind::StaticKw);

		if self.parse_func(checkpoint) {

		} else if self.parse_var(checkpoint) {

		} else if self.parse_let(checkpoint) {

		} else if self.parse_init(checkpoint) {
			
		} else if self.parse_struct(checkpoint) {
			
		} else {
			self.bump();
		}
	}
}