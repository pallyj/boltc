use super::{marker::{CompletedMarker, Marker},
            Parser};
use crate::lexer::SyntaxKind;

const TYPE_RECOVERY_SET: &[SyntaxKind] = &[SyntaxKind::LetKw,
                                           SyntaxKind::ReturnKw,
                                           SyntaxKind::Equals,
                                           SyntaxKind::Colon,
                                           SyntaxKind::OpenBrace,
                                           SyntaxKind::Semicolon,
                                           SyntaxKind::Period];

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_ty(&mut self) {
        let ty = self.start();

        if self.eat(SyntaxKind::Ident) {
            let completed = ty.complete(self, SyntaxKind::NamedType);

            self.parse_ty_postfix(completed);
        } else if self.check(SyntaxKind::OpenParen) {
            self.parse_ty_unit(ty)
        } else if self.eat(SyntaxKind::FuncKw) {
            self.parse_ty_func(ty)
        } else if self.eat(SyntaxKind::UnderscoreKw) {
            ty.complete(self, SyntaxKind::InferType);
        } else {
            self.error_recover("expected type", TYPE_RECOVERY_SET);

            ty.complete(self, SyntaxKind::Error);
        }
    }

    pub fn parse_ty_unit(&mut self, marker: Marker) {
        let tuple_types_len = self.parse_paren_comma_seq(Self::parse_ty);

        match tuple_types_len {
            0 => marker.complete(self, SyntaxKind::UnitType),
            1 => marker.complete(self, SyntaxKind::ParenthesizedType),
            _ => marker.complete(self, SyntaxKind::TupleType),
        };
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

        if self.eat(SyntaxKind::Colon) {
            self.parse_ty()
        }

        marker.complete(self, SyntaxKind::FuncReturn);
    }

    pub fn parse_ty_postfix(&mut self, parent: CompletedMarker) {
        if self.eat(SyntaxKind::Period) {
            let marker = parent.precede(self);

            if !self.eat(SyntaxKind::Ident) {
                self.error_recover("expected member name", TYPE_RECOVERY_SET);
            }

            let completed_marker = marker.complete(self, SyntaxKind::MemberType);

            self.parse_ty_postfix(completed_marker);
        }
    }
}
