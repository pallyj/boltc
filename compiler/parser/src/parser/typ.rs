use crate::lexer::SyntaxKind;
use super::Parser;

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_ty(&mut self) {
		let ty_start = self.checkpoint();

		if self.eat_and_start_node(SyntaxKind::Ident, SyntaxKind::NamedType) {
			// Check paths
			self.finish_node();

			self.parse_ty_postfix(ty_start);
		} else if self.eat_and_start_node(SyntaxKind::OpenParen, SyntaxKind::UnitType) {
			self.parse_ty_unit()
		} else if self.eat_and_start_node(SyntaxKind::FuncKw, SyntaxKind::FuncType) {
			self.parse_ty_func()
		} else if self.eat_and_start_node(SyntaxKind::UnderscoreKw, SyntaxKind::InferType) {
			self.finish_node()
		} else {
			self.start_node(SyntaxKind::Error);
			self.finish_node();
			// Recover
		}

		
	}

	pub fn parse_ty_unit(&mut self) {
		if self.eat(SyntaxKind::CloseParen) {
			self.finish_node();
		} else {
			// Check for a CloseParen token ahead
			// Continue parsing without a )
			// Throw expected ) found ...

			// For now, don't throw an error, just continue
			self.finish_node()
		}
	}

	pub fn parse_ty_func(&mut self) {
		// Parse a type list
		self.parse_paren_comma_seq(|parser| {
			parser.parse_ty();
		});

		self.parse_ty_return();

		self.finish_node();
	}

	pub fn parse_ty_return(&mut self) {
		self.start_node(SyntaxKind::FuncReturn);

		if self.eat(SyntaxKind::Colon) {
			self.parse_ty()
		}

		self.finish_node();
	}

	pub fn parse_ty_postfix(&mut self, checkpoint: usize) {
		if self.eat(SyntaxKind::Period) {
			self.start_node_at(SyntaxKind::MemberType, checkpoint);

			if self.eat(SyntaxKind::Ident) {
				self.finish_node();
			} else {
				// Expected ident, recover
				self.bump();
				self.finish_node()
			}

			self.parse_ty_postfix(checkpoint);
		}
	}
}