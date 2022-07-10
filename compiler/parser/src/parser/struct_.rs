use super::{file::ITEM_RECOVERY_SET, marker::Marker, Parser};
use crate::lexer::SyntaxKind;

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_struct(&mut self, marker: Marker) {
        debug_assert!(self.check(SyntaxKind::StructKw));
        self.eat(SyntaxKind::StructKw);

        self.name(ITEM_RECOVERY_SET);

        self.parse_delim(SyntaxKind::StructBody,
                         SyntaxKind::OpenBrace,
                         SyntaxKind::CloseBrace,
                         |parser| parser.parse_struct_item());

        marker.complete(self, SyntaxKind::StructDef);
    }

    pub fn parse_struct_item(&mut self) {
        if self.eat(SyntaxKind::Semicolon) {
            let marker = self.start();
            self.eat(SyntaxKind::Semicolon);
            marker.complete(self, SyntaxKind::NoOp);
            return;
        }

        let marker = self.start();
        self.parse_comments();
        self.parse_attributes();
        self.parse_visibility();
        self.eat(SyntaxKind::StaticKw);

        match self.peek() {
            Some(SyntaxKind::FuncKw) |
            Some(SyntaxKind::MutatingKw) => self.parse_func(marker, true),
            Some(SyntaxKind::OperatorKw) => self.parse_operator_func(marker),
            Some(SyntaxKind::VarKw) => self.parse_var(marker),
            Some(SyntaxKind::LetKw) => self.parse_let(marker),
            Some(SyntaxKind::InitKw) => self.parse_init(marker),
            Some(SyntaxKind::EnumKw) => self.parse_enum(marker),
            Some(SyntaxKind::StructKw) => self.parse_struct(marker),
            Some(SyntaxKind::TypeAliasKw) => self.parse_type_alias(marker),
            _ => {
                // Error
                self.error_recover("expected struct item", ITEM_RECOVERY_SET);
                marker.complete(self, SyntaxKind::Error);
            }
        }
    }
}
