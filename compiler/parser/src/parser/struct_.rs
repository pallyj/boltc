use crate::lexer::SyntaxKind;

use super::{Parser, marker::Marker};

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_struct(&mut self, marker: Marker) {
		debug_assert!(self.check(SyntaxKind::StructKw));
		self.eat(SyntaxKind::StructKw);

		let func_name = self.start();
		if !self.eat(SyntaxKind::Ident) {

		}
		func_name.complete(self, SyntaxKind::FuncName);

		self.parse_delim(
			SyntaxKind::StructBody,
			SyntaxKind::OpenBrace,
			SyntaxKind::CloseBrace,
			|parser| parser.parse_struct_item());

		marker.complete(self, SyntaxKind::StructDef);
	}

	pub fn parse_struct_item(&mut self) {
		let marker = self.start();

		self.parse_visibility();
		self.eat(SyntaxKind::StaticKw);

		match self.peek() {
			Some(SyntaxKind::FuncKw) => self.parse_func(marker),
			Some(SyntaxKind::VarKw) => self.parse_var(marker),
			Some(SyntaxKind::LetKw) => self.parse_let(marker),
			Some(SyntaxKind::InitKw) => self.parse_init(marker),
			Some(SyntaxKind::StructKw) => self.parse_struct(marker),
			_ => {
				// Error

			}
		}
	}
}