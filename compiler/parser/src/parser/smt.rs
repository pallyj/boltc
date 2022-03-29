use crate::lexer::SyntaxKind;

use super::Parser;

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_smt(&mut self) {
		if self.eat_and_start_node(SyntaxKind::ReturnKw, SyntaxKind::ReturnSmt) {
			if self.check_expr() {
				self.parse_expr();
			}

			self.finish_node()
		} else if self.eat_and_start_node(SyntaxKind::LetKw, SyntaxKind::LetSmt) {
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
		} else if self.eat_and_start_node(SyntaxKind::Semicolon, SyntaxKind::NoOp) {
			self.finish_node()
		} else {
			self.start_node(SyntaxKind::EvalSmt);
			
			self.parse_expr();

			self.eat(SyntaxKind::Semicolon);

			self.finish_node();
		}
	}

	pub fn parse_codeblock(&mut self) {
		self.parse_delim(
			SyntaxKind::CodeBlock,
			SyntaxKind::OpenBrace,
			SyntaxKind::CloseBrace,
			|parser| parser.parse_smt())
	}
}