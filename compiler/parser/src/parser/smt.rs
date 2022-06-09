use super::Parser;
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

            if !self.eat(SyntaxKind::Ident) {
                self.error_recover("expected name", LET_RECOVERY_SET);
            }

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
