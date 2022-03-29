use rowan::Checkpoint;

use crate::lexer::SyntaxKind;

use super::Parser;

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_var(&mut self, checkpoint: usize) -> bool {
		if !self.eat_and_start_node_at(SyntaxKind::VarKw, SyntaxKind::VarDef, checkpoint) {
			return false
		}


		if !self.eat(SyntaxKind::Ident) {
			// Recover
			self.bump();
		}

		self.start_node(SyntaxKind::BindType);

		if self.eat(SyntaxKind::Colon) {
			self.parse_ty();
		}

		self.finish_node();


		// Parse the default value
		self.start_node(SyntaxKind::AssignValue);

		if self.eat(SyntaxKind::Equals) {
			self.parse_expr();
		}

		self.finish_node();

		self.finish_node();

		return true;
	}

	pub fn parse_let(&mut self, checkpoint: usize) -> bool {
		if !self.eat_and_start_node_at(SyntaxKind::LetKw, SyntaxKind::LetDef, checkpoint) {
			return false
		}


		if !self.eat(SyntaxKind::Ident) {
			// Recover
			self.bump();
		}

		self.start_node(SyntaxKind::BindType);

		if self.eat(SyntaxKind::Colon) {
			self.parse_ty();
		}

		self.finish_node();


		// Parse the default value
		self.start_node(SyntaxKind::AssignValue);

		if self.eat(SyntaxKind::Equals) {
			self.parse_expr();
		}

		self.finish_node();

		self.finish_node();

		return true;
	}
}