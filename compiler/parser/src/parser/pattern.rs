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
		else if self.check(SyntaxKind::Period) {
			// Variant pat
			self.bump();
			if !self.eat(SyntaxKind::Ident) {
				self.error("expected variant name");
			}

			if self.check(SyntaxKind::OpenParen) {
				self.parse_paren_comma_seq(Self::parse_pattern_tuple);
			}

			pattern.complete(self, SyntaxKind::VariantPattern);
		}
		else if self.check(SyntaxKind::OpenParen) {
			self.parse_paren_comma_seq(Self::parse_pattern_tuple);

			pattern.complete(self, SyntaxKind::TuplePattern);
		}
		else if self.check_expr()
		{
			self.node(SyntaxKind::Literal, Self::parse_expr);
			
			pattern.complete(self, SyntaxKind::LiteralPattern);
		}
		else {
			self.error_recover("expected pattern", EXPR_RECOVERY_SET);
			pattern.complete(self, SyntaxKind::Error);
		}
	}

	fn parse_pattern_tuple(&mut self) {
        let marker = self.start();

        if self.check(SyntaxKind::Ident) && self.check_ahead(1, SyntaxKind::Colon) {
            self.eat(SyntaxKind::Ident);
            self.eat(SyntaxKind::Colon);
        }

        self.parse_pattern();

        marker.complete(self, SyntaxKind::FuncArg);
    }
}