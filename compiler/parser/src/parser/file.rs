use crate::{lexer::SyntaxKind};

use super::{Parser, event::Event};

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_import(&mut self, checkpoint: usize) -> bool {
		if self.eat_and_start_node_at(SyntaxKind::ImportKw, SyntaxKind::Import, checkpoint) {
			if !self.eat(SyntaxKind::Ident) {
				// Recover
			}

			self.finish_node();

			return true;
		}

		return false;
	}
	pub fn parse_file_item(&mut self) {
		let checkpoint = self.checkpoint();

		self.parse_visibility();

		if self.parse_func(checkpoint) {

		} else if self.parse_struct(checkpoint) {

		} else if self.parse_import(checkpoint) {

		} else {
			self.bump();
		}
	}


	pub fn parse_file(mut self) -> Vec<Event<'input>> {
		self.start_node(SyntaxKind::Root);

		let mut last_idx = usize::MAX;

		

		while (last_idx != self.cursor) && (self.cursor < self.lexemes.len()) {
			last_idx = self.cursor;
			self.parse_file_item();
		}

		self.finish_node();

		self.events
	}
}