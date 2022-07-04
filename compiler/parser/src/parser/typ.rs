use super::{marker::{CompletedMarker, Marker},
            Parser, expr::EXPR_RECOVERY_SET};
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

    pub fn parse_ty_tuple(&mut self) {
        let marker = self.start();

        if self.check(SyntaxKind::Ident) && self.check_ahead(1, SyntaxKind::Colon) {
            self.eat(SyntaxKind::Ident);
            self.eat(SyntaxKind::Colon);
        }

        self.parse_ty();

        marker.complete(self, SyntaxKind::FuncArg);
    }

    // parenthesized type CAN lower to tuple type
    pub fn parse_ty_unit(&mut self, marker: Marker) {
        let tuple_types_len = self.parse_paren_comma_seq(Self::parse_ty_tuple);

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

        let colon = self.check(SyntaxKind::Colon);
        let arrow = self.check(SyntaxKind::Arrow);

        let use_arrow_returns = feature_gate::has_feature("arrow_function");

        if arrow || colon {
            // todo: change it to a warning
            if arrow != use_arrow_returns {
                if use_arrow_returns {
                    //self.error("expected `->`");
                } else {
                    self.error("bolt uses the colon symbol for return types");
                }
            }

            self.bump();

            self.parse_ty();
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
        } else if self.check(SyntaxKind::Operator) {
            if self.lexemes[self.cursor].source == "<" {
                let generic = parent.precede(self);

                self.bump();

                let list = self.start();

                loop {
                    self.parse_ty();

                    if !self.eat(SyntaxKind::Comma) {
                        break
                    }
                }

                if self.check(SyntaxKind::Operator) {
                    if self.lexemes[self.cursor].source == ">" {
                        list.complete(self, SyntaxKind::CommaSeparatedList);
                        self.bump();

                        let completed_marker = generic.complete(self, SyntaxKind::GenericType);
                        return self.parse_ty_postfix(completed_marker);
                    }
                }

                self.error_recover("expected closing bracket `>`", EXPR_RECOVERY_SET);
            }
        } else if self.eat(SyntaxKind::OpenBracket) {
            let marker = parent.precede(self);

            if self.eat(SyntaxKind::CloseBracket) {
                // Its a slice
                marker.complete(self, SyntaxKind::SliceType);
                return;
            }

            if !self.check_expr() {
                self.error_recover("expected expression as array length", EXPR_RECOVERY_SET);
            }

            self.parse_expr();

            if !self.eat(SyntaxKind::CloseBracket) {
                self.error_recover("expected `]`", EXPR_RECOVERY_SET);
            }

            marker.complete(self, SyntaxKind::ArrayType);
        }
    }
}
