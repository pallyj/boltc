use rowan::Checkpoint;

use crate::lexer::SyntaxKind;

use super::{Parser, marker::Marker};

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_let(&mut self, marker: Marker) {
		debug_assert!(self.check(SyntaxKind::LetKw));
		self.eat(SyntaxKind::LetKw);

		self.expect(SyntaxKind::Ident);

		let bind_type = self.start();
		if self.eat(SyntaxKind::Colon) { self.parse_ty(); }
		bind_type.complete(self, SyntaxKind::BindType);

		let assign_value = self.start();
		if self.eat(SyntaxKind::Equals) { self.parse_expr(); }
		assign_value.complete(self, SyntaxKind::AssignValue);

		marker.complete(self, SyntaxKind::LetDef);
	}
	pub fn parse_var(&mut self, marker: Marker) {
		debug_assert!(self.check(SyntaxKind::VarKw));
		self.eat(SyntaxKind::VarKw);

		self.expect(SyntaxKind::Ident);

		let bind_type = self.start();
		if self.eat(SyntaxKind::Colon) { self.parse_ty(); }
		bind_type.complete(self, SyntaxKind::BindType);

		let assign_value = self.start();
		if self.eat(SyntaxKind::Equals) { self.parse_expr(); }
		assign_value.complete(self, SyntaxKind::AssignValue);

		marker.complete(self, SyntaxKind::VarDef);
	}
}