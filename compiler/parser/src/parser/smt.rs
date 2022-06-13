use super::{Parser, expr::EXPR_RECOVERY_SET};
use crate::lexer::SyntaxKind;

const LET_RECOVERY_SET: &[SyntaxKind] = &[SyntaxKind::LetKw,
                                          SyntaxKind::Colon,
                                          SyntaxKind::Equals,
                                          SyntaxKind::Semicolon,
                                          SyntaxKind::ReturnKw,
                                          SyntaxKind::OpenParen];

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_smt(&mut self) {
        let marker = self.start();

        if self.eat(SyntaxKind::ReturnKw) {
            if self.check_expr() {
                self.parse_expr();
            }

            marker.complete(self, SyntaxKind::ReturnSmt);
        } else if self.eat(SyntaxKind::LetKw) {
            self.eat(SyntaxKind::VarKw);

            self.parse_pattern();

            self.node(SyntaxKind::BindType, |parser| {
                    if parser.eat(SyntaxKind::Colon) {
                        parser.parse_ty();
                    }
                });

            // Parse the default value
            self.node(SyntaxKind::AssignValue, |parser| {
                    if parser.eat(SyntaxKind::Equals) {
                        parser.parse_expr();
                    }
                });

            marker.complete(self, SyntaxKind::LetSmt);
        } else if self.eat(SyntaxKind::BreakKw) {
            // todo: add break value, break `label
            marker.complete(self, SyntaxKind::BreakSmt);
        } else if self.eat(SyntaxKind::ContinueKw) {
            // todo: add continue `label
            marker.complete(self, SyntaxKind::ContinueSmt);
        } else if self.eat(SyntaxKind::GuardKw) {
            if self.eat(SyntaxKind::LetKw) {
                self.parse_pattern();

                if !self.eat(SyntaxKind::Equals) {
                    self.error("expected `=`");
                }
        
                self.node(SyntaxKind::Condition, Self::parse_expr);

                if !self.eat(SyntaxKind::ElseKw) {
                    self.error_recover("expected `else` keyword", EXPR_RECOVERY_SET);
                }
        
                if !self.check(SyntaxKind::OpenBrace) {
                    self.error_recover("expected open brace", EXPR_RECOVERY_SET);
                    marker.complete(self, SyntaxKind::Error);
                    return
                }
        
                self.parse_codeblock();
        
                marker.complete(self, SyntaxKind::GuardLet);
            } else {
                self.node(SyntaxKind::Condition, Self::parse_expr);


                if !self.eat(SyntaxKind::ElseKw) {
                    self.error_recover("expected `else` keyword", EXPR_RECOVERY_SET);
                }

                if !self.check(SyntaxKind::OpenBrace) {
                    self.error_recover("expected open brace", EXPR_RECOVERY_SET);
                    marker.complete(self, SyntaxKind::Error);
                    return
                }

                self.parse_codeblock();
                marker.complete(self, SyntaxKind::Guard);
            }

        } else if self.eat(SyntaxKind::Semicolon) {
            marker.complete(self, SyntaxKind::NoOp);
        } else {
            self.parse_expr();

            // Parse the trailing semicolon for an expr
            self.eat(SyntaxKind::Semicolon);

            marker.complete(self, SyntaxKind::EvalSmt);
        }
    }

    pub fn parse_codeblock(&mut self) {
        self.parse_delim(SyntaxKind::CodeBlock,
                         SyntaxKind::OpenBrace,
                         SyntaxKind::CloseBrace,
                         |parser| parser.parse_smt())
    }
}
