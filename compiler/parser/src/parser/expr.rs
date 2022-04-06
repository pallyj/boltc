use super::{marker::{CompletedMarker, Marker},
            Parser};
use crate::lexer::SyntaxKind;

const EXPR_RECOVERY_SET: &[SyntaxKind] = &[SyntaxKind::LetKw,
                                           SyntaxKind::ReturnKw,
                                           SyntaxKind::OpenBrace,
                                           SyntaxKind::CloseBrace,
                                           SyntaxKind::Semicolon,
                                           SyntaxKind::OpenParen,
                                           SyntaxKind::Period];

impl<'input, 'l> Parser<'input, 'l> {
    pub fn parse_expr(&mut self) {
        self.parse_expr_raw(/*OperatorPrecedence::None*/)
    }

    pub fn parse_expr_raw(&mut self /* , in_precedence: OperatorPrecedence */) {
        let mut completed = self.parse_expr_atom();

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

				self.parse_paren_comma_seq(|parser| parser.parse_expr());

				completed = marker.complete(self, SyntaxKind::FuncCallExpr);
			}
			/*else if self.parse_expr_postfix(in_precedence, checkpoint) {

			}
			// Do trailing closures
			*/ else {
				break;
			}
        }
    }

    // pub fn parse_expr_postfix(
    // &mut self,
    // in_precedence: OperatorPrecedence,
    // checkpoint: Checkpoint) -> bool {
    // if self.check(SyntaxKind::Operator) {
    // let op_text = self.lexer.peek().unwrap().source;
    //
    // let Some(operator) = self.operators.get_postfix_op(op_text) else {
    // return false;
    // };
    //
    // if operator.fix() == OperatorFix::Postfix {
    // self.bump();
    // self.start_node(SyntaxKind::PostfixExpr);
    // self.finish_node();
    // } else {
    // let precedence = operator.precedence();
    // if precedence.shifts(in_precedence) {
    // Consume
    // self.bump();
    // self.start_node_at(SyntaxKind::InfixExpr, checkpoint);
    // self.parse_expr_raw(precedence);
    // self.finish_node();
    // } else {
    // Do nothing, finish the node
    // return false;
    // }
    // }
    //
    // true
    // } else {
    // false
    // }
    // }

    pub fn check_expr(&mut self) -> bool {
        self.check(SyntaxKind::Ident) ||
		self.check(SyntaxKind::LiteralDecInt) ||
		self.check(SyntaxKind::LiteralBinInt) ||
		self.check(SyntaxKind::LiteralOctInt) ||
		self.check(SyntaxKind::LiteralHexInt) ||
		self.check(SyntaxKind::LiteralDecFloat) ||
		//self.check(SyntaxKind::Operator) ||
		self.check(SyntaxKind::OpenParen) ||
		self.check(SyntaxKind::IfKw)
    }

    pub fn parse_expr_atom(&mut self) -> CompletedMarker {
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
		} /*else if self.eat_and_start_node(SyntaxKind::Operator, SyntaxKind::PrefixExpr) {
			// Change so we can have prefix precedence
			self.parse_expr_atom();
			self.finish_node();
		}*/ else if self.eat(SyntaxKind::IfKw) {
			self.parse_expr_if(marker)
		} else {
			// Try to do recovery
			self.error_recover("expected expression", EXPR_RECOVERY_SET)
		}
    }

    pub fn parse_expr_if(&mut self, marker: Marker) -> CompletedMarker {
        self.node(SyntaxKind::Condition, |parser| parser.parse_expr());

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
