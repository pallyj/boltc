use super::{event::Event, marker::Marker, Parser};
use crate::lexer::SyntaxKind;

pub const ITEM_RECOVERY_SET: &[SyntaxKind] = &[SyntaxKind::StaticKw,
                                               SyntaxKind::PublicKw,
                                               SyntaxKind::InternalKw,
                                               SyntaxKind::FilePrivateKw,
                                               SyntaxKind::PrivateKw,
                                               SyntaxKind::FuncKw,
                                               SyntaxKind::InitKw,
                                               SyntaxKind::StructKw,
                                               SyntaxKind::LetKw,
                                               SyntaxKind::VarKw,
                                               SyntaxKind::At,
                                               SyntaxKind::OperatorKw];

pub const INNER_ITEM_RECOVERY_SET: &[SyntaxKind] = &[SyntaxKind::StaticKw,
                                                     SyntaxKind::PublicKw,
                                                     SyntaxKind::InternalKw,
                                                     SyntaxKind::FilePrivateKw,
                                                     SyntaxKind::PrivateKw,
                                                     SyntaxKind::FuncKw,
                                                     SyntaxKind::InitKw,
                                                     SyntaxKind::StructKw,
                                                     SyntaxKind::LetKw,
                                                     SyntaxKind::VarKw,
                                                     SyntaxKind::OpenBrace,
                                                     SyntaxKind::CloseBrace,
                                                     SyntaxKind::At,
                                                     SyntaxKind::OperatorKw];

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_import(&mut self, marker: Marker) {
        assert!(self.check(SyntaxKind::ImportKw));
        self.eat(SyntaxKind::ImportKw);

        if !self.eat(SyntaxKind::Ident) {
            // Recover
        }

        marker.complete(self, SyntaxKind::Import);
    }

    // todo: whitespaces are parsed afterwards
    pub fn parse_comments(&mut self) {
        let marker = self.start();
        marker.complete(self, SyntaxKind::Docs);
    }

    pub fn parse_file_item(&mut self) {
        if self.eat(SyntaxKind::Semicolon) {
            let marker = self.start();
            self.eat(SyntaxKind::Semicolon);
            marker.complete(self, SyntaxKind::NoOp);
            return;
        }

        self.eat_trivia();

        let marker = self.start();
        self.parse_comments();
        self.parse_attributes();
        self.parse_visibility();

        match self.peek() {
            Some(SyntaxKind::FuncKw) => self.parse_func(marker, false),
            Some(SyntaxKind::LetKw) => self.parse_let(marker),
            Some(SyntaxKind::StructKw) => self.parse_struct(marker),
            Some(SyntaxKind::ImportKw) => self.parse_import(marker),
            Some(SyntaxKind::EnumKw) => self.parse_enum(marker),
            Some(SyntaxKind::TypeAliasKw) => self.parse_type_alias(marker),
            _ => {
                self.error_recover("expected item", ITEM_RECOVERY_SET);
                marker.complete(self, SyntaxKind::Error);
            }
        }
    }

    pub fn parse_file(mut self) -> Vec<Event<'input>> {
        let marker = self.start();

        let mut last_idx = usize::MAX;

        while (last_idx != self.cursor) && (self.cursor < self.lexemes.len()) {
            last_idx = self.cursor;
            self.parse_file_item();
            self.eat_trivia();
        }

        marker.complete(&mut self, SyntaxKind::Root);

        self.events
    }

    pub (crate) fn parse_test<F>(mut self, test: F) -> Vec<Event<'input>>
        where F: Fn(&mut Parser)
    {
        let marker = self.start();

        let mut last_idx = usize::MAX;

        while (last_idx != self.cursor) && (self.cursor < self.lexemes.len()) {
            last_idx = self.cursor;
            test(&mut self);
            self.eat_trivia();
        }

        marker.complete(&mut self, SyntaxKind::Root);

        self.events
    }
}
