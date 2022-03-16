use rowan::Checkpoint;

use crate::{lexer::SyntaxKind, ast::{SyntaxNode, Parse}};

use super::Parser;

impl<'a> Parser<'a> {
	pub fn parse_import(&mut self, checkpoint: Checkpoint) -> bool {
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
		let checkpoint = self.builder.checkpoint();

		self.parse_visibility();

		if self.parse_func(checkpoint) {

		} else if self.parse_struct(checkpoint) {

		} else if self.parse_import(checkpoint) {

		} else {
			self.bump();
		}
	}


	pub fn parse_file(mut self) -> Parse {
		self.start_node(SyntaxKind::Root);

		let mut last_idx = usize::MAX;

		while (last_idx != self.index) && self.lexer.peek().is_some() {
			last_idx = self.index;
			self.parse_file_item();
		}

		self.finish_node();

		let green = self.builder.finish();

		Parse {
			root: SyntaxNode::new_root(green)
		}
	}
}