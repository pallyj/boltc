use crate::lexer::SyntaxKind;
use super::{Parser, marker::{Marker, CompletedMarker}};

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_ty(&mut self) {
		let ty = self.start();

		if self.eat(SyntaxKind::Ident) {
			let completed = ty.complete(self, SyntaxKind::NamedType);

			self.parse_ty_postfix(completed);
		} else if self.eat(SyntaxKind::OpenParen) {
			self.parse_ty_unit(ty)
		} else if self.eat(SyntaxKind::FuncKw) {
			self.parse_ty_func(ty)
		} else if self.eat(SyntaxKind::UnderscoreKw) {
			ty.complete(self, SyntaxKind::InferType);
		} else {
			ty.complete(self, SyntaxKind::Error);
			// Recover from an error
		}
	}

	pub fn parse_ty_unit(&mut self, marker: Marker) {
		if self.eat(SyntaxKind::CloseParen) {
			marker.complete(self, SyntaxKind::UnitType);
		} else {
			// Check for a CloseParen token ahead
			// Continue parsing without a )
			// Throw expected ) found ...

			// For now, don't throw an error, just continue
			//self.error(error);

			marker.complete(self, SyntaxKind::Error);
		}
	}

	pub fn parse_ty_func(&mut self, marker: Marker) {
		// Parse a type list
		self.parse_paren_comma_seq(|parser| {
			parser.parse_ty();
		});

		self.parse_ty_return();

		marker.complete(self, SyntaxKind::FuncType);
	}

	pub fn parse_ty_return(&mut self) {
		let marker = self.start();

		if self.eat(SyntaxKind::Colon) { self.parse_ty() }

		marker.complete(self, SyntaxKind::FuncReturn);
	}

	pub fn parse_ty_postfix(&mut self, parent: CompletedMarker) {
		if self.eat(SyntaxKind::Period) {
			let marker = parent.precede(self);

			if !self.eat(SyntaxKind::Ident) {
				// Error
			}

			let completed_marker = marker.complete(self, SyntaxKind::MemberType);

			self.parse_ty_postfix(completed_marker);
		}
	}
}