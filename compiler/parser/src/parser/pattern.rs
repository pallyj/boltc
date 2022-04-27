use crate::lexer::SyntaxKind;

use super::{Parser, expr::EXPR_RECOVERY_SET};

// _ => anonymous
// ident => named
// `(` => tuple

// variant => variant
// literal => literal

impl<'input, 'l> Parser<'input, 'l> {
	pub fn parse_pattern(&mut self)
	{
		let pattern = self.start();

		if self.check(SyntaxKind::UnderscoreKw) {
			// Wildcard
			self.bump();

			pattern.complete(self, SyntaxKind::WildcardPattern);
		}
		else if self.check(SyntaxKind::Ident) {
			// Named pat
			self.bump();

			pattern.complete(self, SyntaxKind::BindPattern);
		}
		else if self.check(SyntaxKind::LiteralDecInt)
			 || self.check(SyntaxKind::LiteralBinInt)
			 || self.check(SyntaxKind::LiteralOctInt)
			 || self.check(SyntaxKind::LiteralHexInt)
			 || self.check(SyntaxKind::LiteralDecFloat)
			 || self.check(SyntaxKind::LiteralTrue)
			 || self.check(SyntaxKind::LiteralFalse)
			 || self.check(SyntaxKind::StringLiteral)
		{
			self.bump();
			
			pattern.complete(self, SyntaxKind::LiteralPattern);
		}
		else if self.check(SyntaxKind::Period) {
			// Variant pat
			self.bump();
			if !self.eat(SyntaxKind::Ident) {
				self.error("expected variant name");
			}

			if self.check(SyntaxKind::OpenParen) {
				self.parse_paren_comma_seq(Self::parse_pattern);
			}

			pattern.complete(self, SyntaxKind::VariantPattern);
		}
		else if self.check(SyntaxKind::OpenParen) {
			self.parse_paren_comma_seq(Self::parse_pattern);

			pattern.complete(self, SyntaxKind::TuplePattern);
		}
		else {
			self.error_recover("expected pattern", EXPR_RECOVERY_SET);
			pattern.complete(self, SyntaxKind::Error);
		}
	}
}