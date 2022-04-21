use super::{marker::{CompletedMarker, Marker},
            Parser};
use crate::{lexer::SyntaxKind, operators::{OperatorPrecedence, OperatorFix}};

const EXPR_RECOVERY_SET: &[SyntaxKind] = &[SyntaxKind::LetKw,
                                           SyntaxKind::ReturnKw,
                                           SyntaxKind::OpenBrace,
                                           SyntaxKind::CloseBrace,
                                           SyntaxKind::Semicolon,
                                           SyntaxKind::OpenParen,
                                           SyntaxKind::Period];

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_expr(&mut self) {
        self.parse_expr_raw(OperatorPrecedence::None, false)
    }

    pub fn parse_expr_before_brace(&mut self) {
        self.parse_expr_raw(OperatorPrecedence::None, true)
    }

    pub fn parse_expr_raw(
        &mut self,
        in_precedence: OperatorPrecedence,
        is_before_brace: bool) {
        let mut completed = self.parse_expr_atom(is_before_brace);

        let mut cur = usize::MAX;

        while cur != self.cursor {
            cur = self.cursor;
            if self.eat(SyntaxKind::Period) {
				if self.check(SyntaxKind::Ident) {
					let marker = completed.precede(self);
					self.bump();
					completed = marker.complete(self, SyntaxKind::MemberExpr);
				} else {
					self.error_recover("expected member name", EXPR_RECOVERY_SET);
				}
			} else if self.check(SyntaxKind::OpenParen) {
				let marker = completed.precede(self);

				self.parse_paren_comma_seq(Self::parse_func_arg);

				completed = marker.complete(self, SyntaxKind::FuncCallExpr);
			} else if self.check(SyntaxKind::Operator) {
                if let Some(next) = self.parse_expr_postfix(in_precedence, is_before_brace, completed) {
                    completed = next;
                } else {
                    break
                }
			} else if self.check(SyntaxKind::OpenBrace) {
                if is_before_brace { break; }

                completed = self.parse_trailing_closure(completed);
            }
			// Do trailing closures
			else {
				break;
			}
        }
    }

    pub fn parse_func_arg(&mut self) {
        let marker = self.start();

        if self.check(SyntaxKind::Ident) && self.check_ahead(1, SyntaxKind::Colon) {
            // The func arg has a label

            self.eat(SyntaxKind::Ident);
            self.eat(SyntaxKind::Colon);
        }

        self.parse_expr();

        marker.complete(self, SyntaxKind::FuncArg);
    }

    pub fn parse_expr_postfix(
        &mut self,
        in_precedence: OperatorPrecedence,
        is_before_brace: bool,
        completed: CompletedMarker) -> Option<CompletedMarker>
    {
        assert!(self.check(SyntaxKind::Operator));

        let operator_symbol = self.lexemes[self.cursor].source;

        let Some(operator) = self.operators.get_postfix_op(operator_symbol) else {
            // Error: this isn't a recognized operator
            let marker = completed.precede(self);
            self.error_recover(&format!("operator `{operator_symbol}` is not recognized"), EXPR_RECOVERY_SET);
            return Some(marker.complete(self, SyntaxKind::Error))
        };

        if operator.fix() == OperatorFix::Postfix {
            let marker = completed.precede(self);
            self.bump();
            return Some(marker.complete(self, SyntaxKind::PostfixExpr));
        }

        let operator_precedence = operator.precedence();

        if !operator_precedence.shifts(in_precedence) {
            // Do nothing
            return None
        }

        // Parse as an infix operator
        let marker = completed.precede(self);
        self.bump(); // Eat the operator
        self.parse_expr_raw(operator_precedence, is_before_brace);

        Some(marker.complete(self, SyntaxKind::InfixExpr))
    }

    pub fn parse_trailing_closure(&mut self, marker: CompletedMarker) -> CompletedMarker {
        let marker = marker.precede(self);

        let closure_marker = self.start();
        self.parse_closure(closure_marker);

        marker.complete(self, SyntaxKind::TrailingClosure)
    }

    pub fn parse_closure(&mut self, marker: Marker) -> CompletedMarker {
        assert!(self.check(SyntaxKind::OpenBrace));
        self.eat(SyntaxKind::OpenBrace);
        
        self.parse_closure_args();
        self.parse_closure_body();

        marker.complete(self, SyntaxKind::Closure)
    }

    fn parse_closure_body(&mut self) {
        self.parse_delim_end(SyntaxKind::CodeBlock,
            SyntaxKind::CloseBrace,
            |parser| parser.parse_smt())
    }

    const CLOSURE_YIELD_SYMBOL: SyntaxKind = SyntaxKind::BigArrow;

    fn parse_closure_args(&mut self) {
        // Determine if the closure takes arguments
        let mut peek_ahead = 0;
        let mut takes_parameters = false;
        loop {
            if self.check_ahead(peek_ahead, Self::CLOSURE_YIELD_SYMBOL) {
                takes_parameters = true;
                break;
            }

            if !self.check_ahead(peek_ahead, SyntaxKind::Ident) {
                break;
            }

            peek_ahead += 1;

            if self.check_ahead(peek_ahead, SyntaxKind::Colon) {
                takes_parameters = true;
                break;
            }

            if self.check_ahead(peek_ahead, SyntaxKind::Comma) {
                peek_ahead += 1;
            }
        }

        if !takes_parameters {
            return;
        }

        let marker = self.start();

        loop {
            if self.eat(Self::CLOSURE_YIELD_SYMBOL) {
                break;
            }

            self.parse_closure_param();

            if self.eat(Self::CLOSURE_YIELD_SYMBOL) {
                break;
            }

            if !self.eat(SyntaxKind::Comma) {
                self.error("expected `,`");
            }
        }

        marker.complete(self, SyntaxKind::CommaSeparatedList);
    }

    fn parse_closure_param(&mut self) {
        let marker = self.start();

        if !self.eat(SyntaxKind::Ident) {
            self.error_recover("expected closure param", &[SyntaxKind::Comma,
                                                            SyntaxKind::CloseBrace,
                                                            SyntaxKind::BigArrow]);
        }

        if self.eat(SyntaxKind::Colon) {
            self.parse_ty();
        }

        marker.complete(self, SyntaxKind::FuncPar);
    }

    pub fn check_expr(&mut self) -> bool {
        self.check(SyntaxKind::Ident) ||
		self.check(SyntaxKind::LiteralDecInt) ||
		self.check(SyntaxKind::LiteralBinInt) ||
		self.check(SyntaxKind::LiteralOctInt) ||
		self.check(SyntaxKind::LiteralHexInt) ||
		self.check(SyntaxKind::LiteralDecFloat) ||
		self.check(SyntaxKind::Operator) ||
		self.check(SyntaxKind::OpenParen) ||
		self.check(SyntaxKind::IfKw)
    }

    pub fn parse_expr_atom(&mut self, is_before_brace: bool) -> CompletedMarker {
        let marker = self.start();

        if self.eat(SyntaxKind::Ident) {
			marker.complete(self, SyntaxKind::NamedExpr)
		} else if
		   self.eat(SyntaxKind::LiteralDecInt) ||
		   self.eat(SyntaxKind::LiteralBinInt) ||
		   self.eat(SyntaxKind::LiteralOctInt) ||
		   self.eat(SyntaxKind::LiteralHexInt) ||
		   self.eat(SyntaxKind::LiteralDecFloat) ||
		   self.eat(SyntaxKind::LiteralTrue) ||
		   self.eat(SyntaxKind::LiteralFalse) {
			   marker.complete(self, SyntaxKind::Literal)
		   }
		else if self.eat(SyntaxKind::OpenParen) {
			if self.eat(SyntaxKind::CloseParen) {
				marker.complete(self, SyntaxKind::UnitExpr)
			} else {
				self.parse_expr();

				if !self.eat(SyntaxKind::CloseParen) {
					// Error:
					self.bump();
				}
				marker.complete(self, SyntaxKind::ParenthesizedExpr)
			}
		} else if self.check(SyntaxKind::Operator) {
            let operator_symbol = self.lexemes[self.cursor].source;
            let is_prefix_operator = self.operators.get_prefix_op(operator_symbol).is_some();

            self.bump();

            self.parse_expr_raw(OperatorPrecedence::Prefix, is_before_brace);

            if is_prefix_operator {
                marker.complete(self, SyntaxKind::PrefixExpr)
            } else {
                marker.complete(self, SyntaxKind::Error)
            }
        } else if self.eat(SyntaxKind::IfKw) {
			self.parse_expr_if(marker)
		} else if self.check(SyntaxKind::OpenBrace) {
            self.parse_closure(marker)
        } else {
			// Try to do recovery
			self.error_recover("expected expression", EXPR_RECOVERY_SET);
            marker.complete(self, SyntaxKind::Error)
		}
    }

    pub fn parse_expr_if(&mut self, marker: Marker) -> CompletedMarker {
        self.node(SyntaxKind::Condition, |parser| parser.parse_expr_before_brace());

        self.node(SyntaxKind::Positive, |parser| parser.parse_codeblock());

        if self.eat(SyntaxKind::ElseKw) {
            self.node(SyntaxKind::Negative, |parser| {
                    if parser.check(SyntaxKind::IfKw) {
                        let marker = parser.start();

                        parser.eat(SyntaxKind::IfKw);

                        parser.parse_expr_if(marker);
                    } else if parser.check(SyntaxKind::OpenBrace) {
                        parser.parse_codeblock();
                    } else {
                        parser.error_recover("expected code block or if statement", EXPR_RECOVERY_SET);
                    }
                });
        }

        marker.complete(self, SyntaxKind::IfExpr)
    }
}
