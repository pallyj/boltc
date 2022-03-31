use crate::{lexer::SyntaxKind, parse_error::ParseError};

use super::{Parser, file::ITEM_RECOVERY_SET, marker::Marker};

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_init(&mut self, marker: Marker) {
		assert!(self.check(SyntaxKind::InitKw));
		self.eat(SyntaxKind::InitKw);

		self.parse_paren_comma_seq(|parser| parser.parse_func_par());

		self.parse_codeblock();

		marker.complete(self, SyntaxKind::InitDef);
	}
	pub fn parse_func(&mut self, marker: Marker) {
		assert!(self.check(SyntaxKind::FuncKw));
		self.eat(SyntaxKind::FuncKw);

		self.node(SyntaxKind::FuncName, |parser| if !parser.eat(SyntaxKind::Ident) {
			parser.error_recover(ParseError::Expected(SyntaxKind::Ident), ITEM_RECOVERY_SET);
		});

		self.parse_paren_comma_seq(|parser| parser.parse_func_par());

		self.parse_ty_return();

		if self.check(SyntaxKind::OpenBrace) {
			self.parse_codeblock();
		}

		marker.complete(self, SyntaxKind::FuncDef);
	}

	pub fn parse_func_par(&mut self) {
		let marker = self.start();

		self.expect(SyntaxKind::Ident);
		self.expect(SyntaxKind::Colon);

		self.parse_ty();

		marker.complete(self, SyntaxKind::FuncPar);
	}
}