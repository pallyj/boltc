use super::Parser;
use crate::{lexer::SyntaxKind, parser::file::ITEM_RECOVERY_SET};

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_attribute(&mut self) {
        assert!(self.check(SyntaxKind::At));

        let marker = self.start();
        self.eat(SyntaxKind::At);

        self.name(ITEM_RECOVERY_SET);

        if self.check(SyntaxKind::OpenParen) {
            self.parse_paren_comma_seq(Self::parse_func_arg);
        }

        marker.complete(self, SyntaxKind::Attribute);
    }

    pub fn parse_attributes(&mut self) {
        let marker = self.start();

        while self.check(SyntaxKind::At) {
            self.parse_attribute();
        }

        marker.complete(self, SyntaxKind::Attributes);
    }
}
