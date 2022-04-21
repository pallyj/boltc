use super::{marker::Marker, Parser};
use crate::{lexer::SyntaxKind,
            parser::file::{INNER_ITEM_RECOVERY_SET, ITEM_RECOVERY_SET}};

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_init(&mut self, marker: Marker) {
        assert!(self.check(SyntaxKind::InitKw));
        self.eat(SyntaxKind::InitKw);

        self.parse_paren_comma_seq(|parser| parser.parse_func_par());

        if self.check(SyntaxKind::OpenBrace) {
            self.parse_codeblock();
        } else {
            self.error("expected code block");
        }

        marker.complete(self, SyntaxKind::FuncDef);
    }

    pub fn parse_func(&mut self, marker: Marker) {
        assert!(self.check(SyntaxKind::FuncKw));
        self.eat(SyntaxKind::FuncKw);

        self.name(INNER_ITEM_RECOVERY_SET);

        if self.check(SyntaxKind::OpenParen) {
            self.parse_paren_comma_seq(|parser| parser.parse_func_par());
        } else {
            self.error("expected function arguments");
        }

        self.parse_ty_return();

        if self.check(SyntaxKind::OpenBrace) {
            self.parse_codeblock();
        }

        marker.complete(self, SyntaxKind::FuncDef);
    }

    pub fn parse_operator_func(&mut self, marker: Marker) {
        assert!(self.check(SyntaxKind::OperatorKw));
        self.eat(SyntaxKind::OperatorKw);

        if self.check(SyntaxKind::FuncKw) {
            self.parse_func(marker);
        } else {
            self.error_recover("", ITEM_RECOVERY_SET);
        }
    }

    pub fn parse_func_par(&mut self) {
        let marker = self.start();

        if !self.eat(SyntaxKind::Ident) {
            self.error_recover("expected function param", &[SyntaxKind::Comma,
                                                            SyntaxKind::CloseParen]);
        }

        if !self.check(SyntaxKind::Colon) {
            self.name(&[SyntaxKind::Comma, SyntaxKind::CloseParen]);
        }

        if !self.eat(SyntaxKind::Colon) {
            self.error_recover("expected function param", &[SyntaxKind::Comma,
                                                            SyntaxKind::CloseParen]);
        }

        self.parse_ty();

        marker.complete(self, SyntaxKind::FuncPar);
    }
}
