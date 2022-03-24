use rowan::Checkpoint;

use crate::{lexer::SyntaxKind};

use super::Parser;

impl<'a> Parser<'a> {
	pub fn parse_init(&mut self, checkpoint: Checkpoint) -> bool {
		if !self.eat_and_start_node_at(SyntaxKind::InitKw, SyntaxKind::InitDef, checkpoint) {
			return false;
		}

		println!("InitKW");

		self.parse_paren_comma_seq(|parser| parser.parse_func_par());

		self.parse_codeblock();

		self.finish_node();

		return true
	}
	pub fn parse_func(&mut self, checkpoint: Checkpoint) -> bool {
		if !self.eat_and_start_node_at(SyntaxKind::FuncKw, SyntaxKind::FuncDef, checkpoint) {
			return false;
		}

		if !self.eat_and_start_node(SyntaxKind::Ident, SyntaxKind::FuncName) {
			// Recover
			self.bump();
		}
		self.finish_node();

		self.parse_paren_comma_seq(|parser| parser.parse_func_par());

		self.parse_ty_return();

		if self.check(SyntaxKind::OpenBrace) {
			self.parse_codeblock();
		}

		self.finish_node();

		return true
	}

	pub fn parse_func_par(&mut self) {
		self.start_node(SyntaxKind::FuncPar);

		if !self.eat(SyntaxKind::Ident) {
			// Recover
			self.bump();
		}

		if !self.eat(SyntaxKind::Colon) {
			// Recover
			self.bump();
		}

		self.parse_ty();

		self.finish_node();
	}
}