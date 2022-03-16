use rowan::{Language, /*Checkpoint*/};

use crate::{lexer::SyntaxKind, ast::BoltLanguage, /*operators::{OperatorPrecedence, OperatorFix}*/};

use super::Parser;

impl<'a> Parser<'a> {
	pub fn parse_expr(&mut self) {
		self.parse_expr_raw(/*OperatorPrecedence::None*/)
	}
	pub fn parse_expr_raw(&mut self/*, in_precedence: OperatorPrecedence*/) {
		let checkpoint = self.builder.checkpoint();

		self.parse_expr_atom();

		loop {
			if self.eat_and_start_node_at(SyntaxKind::Period, SyntaxKind::MemberExpr, checkpoint) {
				if self.eat(SyntaxKind::Ident) {
					self.finish_node()
				} else {
					// Recover
					self.bump();
					self.finish_node()
				}
			} else if self.check(SyntaxKind::OpenParen) {
				self.start_node_at(SyntaxKind::FuncCallExpr, checkpoint);
				self.parse_paren_comma_seq(|parser| parser.parse_expr());
				self.finish_node();
			}
			/*else if self.parse_expr_postfix(in_precedence, checkpoint) {

			}
			*/ else {
				break;
			}
		}
	}

	/*pub fn parse_expr_postfix(
		&mut self,
		in_precedence: OperatorPrecedence,
		checkpoint: Checkpoint) -> bool {
		if self.check(SyntaxKind::Operator) {
			let op_text = self.lexer.peek().unwrap().source;

			let Some(operator) = self.operators.get_postfix_op(op_text) else {
				return false;
			};

			if operator.fix() == OperatorFix::Postfix {
				self.bump();
				self.start_node(SyntaxKind::PostfixExpr);
				self.finish_node();
			} else {
				let precedence = operator.precedence();
				if precedence.shifts(in_precedence) {
					// Consume
					self.bump();
					self.start_node_at(SyntaxKind::InfixExpr, checkpoint);
					self.parse_expr_raw(precedence);
					self.finish_node();
				} else {
					// Do nothing, finish the node
					return false;
				}
			}

			true
		} else {
			false
		}
	}*/

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

	pub fn parse_expr_atom(&mut self) {
		if self.eat_and_start_node(SyntaxKind::Ident, SyntaxKind::NamedExpr) {
			self.finish_node();
		} else if 
		   self.eat_and_start_node(SyntaxKind::LiteralDecInt, SyntaxKind::Literal) ||
		   self.eat_and_start_node(SyntaxKind::LiteralBinInt, SyntaxKind::Literal) ||
		   self.eat_and_start_node(SyntaxKind::LiteralOctInt, SyntaxKind::Literal) ||
		   self.eat_and_start_node(SyntaxKind::LiteralHexInt, SyntaxKind::Literal) ||
		   self.eat_and_start_node(SyntaxKind::LiteralDecFloat, SyntaxKind::Literal) ||
		   self.eat_and_start_node(SyntaxKind::LiteralTrue, SyntaxKind::Literal) ||
		   self.eat_and_start_node(SyntaxKind::LiteralFalse, SyntaxKind::Literal)  {
			   self.finish_node();
		   }
		else if self.check(SyntaxKind::OpenParen) {
			let checkpoint = self.builder.checkpoint();

			self.bump();

			if self.eat(SyntaxKind::CloseParen) {
				self.builder.start_node_at(checkpoint, BoltLanguage::kind_to_raw(SyntaxKind::UnitExpr));
				self.builder.finish_node();
			} else {
				self.builder.start_node_at(checkpoint, BoltLanguage::kind_to_raw(SyntaxKind::ParenthesizedExpr));

				self.parse_expr();

				if !self.eat(SyntaxKind::CloseParen) {
					// Recover from errors
					self.bump();
				}
				self.builder.finish_node();


			}
		} /*else if self.eat_and_start_node(SyntaxKind::Operator, SyntaxKind::PrefixExpr) {
			// Change so we can have prefix precedence
			self.parse_expr_atom();
			self.finish_node();
		}*/ else if self.eat_and_start_node(SyntaxKind::IfKw, SyntaxKind::IfExpr) {
			self.parse_expr_if();
		} else {
			// Try to do recovery
			self.start_node(SyntaxKind::Error);
			self.finish_node();
		}
	}

	pub fn parse_expr_if(&mut self) {
		self.start_node(SyntaxKind::Condition);
		// Parse condition
		self.parse_expr();
		self.finish_node();

		self.start_node(SyntaxKind::Positive);
		// Parse codeblock
		self.parse_codeblock();
		self.finish_node();

		if self.eat(SyntaxKind::ElseKw) {
			self.start_node(SyntaxKind::Negative);
			if self.eat_and_start_node(SyntaxKind::IfKw, SyntaxKind::IfExpr) {
				self.parse_expr_if();
			} else if self.check(SyntaxKind::OpenBrace) {
				self.parse_codeblock();
			} else {
				self.bump();
				// Recover
			}
			self.finish_node();
		}

		self.finish_node();
	}
}