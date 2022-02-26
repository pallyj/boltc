use prelude::*;

use bolt_ast::{Statement, Expression, CodeBlock, CodeBlockItem, Type};

use crate::{ParseError, Token, Context, Braced, expr::expression_inner, operators::OperatorPrecedence};

use super::Parse;

impl Parse for Statement {
    type Output = Self;

    fn parse(parser: &mut super::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		let (next_token, next_source) = parser.peek().clone().unwrap();

        match next_token {
			Token::Keyword(kw) if kw.as_str() == "let" => {
				parser.consume();
				let Some(name) = parser.consume_if_map(take_token_kind!(Token::Ident)) else {
					let (t, s) = parser.peek().clone().unwrap();

					return Try::Err(ParseError::ExpectedIdent(t).with_source(s));
				};

				let typ = if parser.consume_if_equal(Token::Punctuation(":".to_string())).is_some() {
					Some(require!(Type::parse(parser, ctx)))
				} else {
					None
				};

				let expr = if parser.consume_if_equal(Token::Punctuation("=".to_string())).is_some() {
					Some(require!(Expression::parse(parser, ctx)))
				} else {
					None
				};

				let source = next_source.until(parser.last_source());

				return Try::Some(Statement::Bind { name, typ, expr }.with_source(source))
			}

			Token::Keyword(kw) if kw.as_str() == "return" => {
				parser.consume();
				let value = into_result!(Expression::parse(parser, ctx)).ok();
				let source = next_source.until(parser.last_source());

				return Try::Some(Statement::Return(value).with_source(source));
			}

			Token::Keyword(kw) if kw.as_str() == "throw" => {
				parser.consume();
				let error = require!(Expression::parse(parser, ctx));
				let source = next_source.until(parser.last_source());

				return Try::Some(Statement::Throw(error).with_source(source));
			}

			Token::Keyword(kw) if kw.as_str() == "break" => {
				parser.consume();
				let label = parser.consume_if_map(take_token_kind!(Token::Ident));
				let source = next_source.until(parser.last_source());

				return Try::Some(Statement::Break(label).with_source(source));
			}

			Token::Keyword(kw) if kw.as_str() == "continue" => {
				parser.consume();
				let label = parser.consume_if_map(take_token_kind!(Token::Ident));
				let source = next_source.until(parser.last_source());

				return Try::Some(Statement::Continue(label).with_source(source));
			}

			Token::Keyword(kw) if kw.as_str() == "while" => {
				let condition = require!(expression_inner(parser, ctx, OperatorPrecedence::None, true));

				let code = require!(Braced::<CodeBlock>::parse(parser, ctx)).unwrap().0;

				let source = next_source.until(parser.last_source());

				return Try::Some(Statement::While { condition, code }.with_source(source))
			}

			Token::Keyword(kw) if kw.as_str() == "repeat" => {
				let code = require!(Braced::<CodeBlock>::parse(parser, ctx)).unwrap().0;

				let source = next_source.until(parser.last_source());

				return Try::Some(Statement::Repeat { code }.with_source(source))
			}

			_ => {
				let (expr, expr_source) = match Expression::parse(parser, ctx) {
					Try::Some(expr) => expr,
					Try::None(err) => return Try::None(err.map(|_| ParseError::ExpectedStatement(next_token))),
					Try::Err(err) => return Try::Err(err),
				}.unwrap();

				return Try::Some(Statement::Expr(expr).with_source(expr_source))
			}
		}
    }
}

impl Parse for CodeBlock {
    type Output = Self;

    fn parse(parser: &mut super::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		let mut statements = vec![];

		let start = parser.next_source();

		loop {
			if parser.consume_if_equal(Token::Punctuation(";".to_string())).is_some() { continue }

			let smt = match Statement::parse(parser, ctx) {
				Try::Some(smt) => smt,
				Try::None(_) => break,
				Try::Err(err) => return Try::Err(err),
			};

			let is_escaped = parser.consume_if_equal(Token::Punctuation(";".to_string())).is_some();
			let has_value = smt.value().has_value();

			statements.push(CodeBlockItem::new(smt, !is_escaped && has_value));
		};

		let source = start.until(parser.last_source());

		Try::Some(CodeBlock::new(statements).with_source(source))
    }
}