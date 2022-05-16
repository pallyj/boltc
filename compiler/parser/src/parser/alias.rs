use crate::{lexer::SyntaxKind, parser::file::ITEM_RECOVERY_SET};

use super::{Parser, marker::Marker};

impl<'input, 'l> Parser<'input, 'l> {
	pub (crate) fn parse_type_alias(&mut self, marker: Marker) {
		assert!(self.check(SyntaxKind::TypeAliasKw));
        self.eat(SyntaxKind::TypeAliasKw);

		self.name(ITEM_RECOVERY_SET);

		if self.eat(SyntaxKind::Equals) {
			self.parse_ty();
		} else {
			// Throw an error and recover?
			self.error_recover("expected symbol `=`", ITEM_RECOVERY_SET);
		}

		marker.complete(self, SyntaxKind::TypeAlias);
	}
}