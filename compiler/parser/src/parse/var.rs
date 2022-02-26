use bolt_ast::{Let, Type, Expression, Var};
use prelude::*;

use crate::{Parse, Token, ParseError, Context};

impl Parse for Let {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<crate::ParseError>> {
        let Some(start) = parser.consume_if_equal(Token::Keyword("let".to_string())) else {
			let (t, s) = parser.peek().clone().unwrap();

			return Try::None(ParseError::ExpectedLet(t).with_source(s));
		};

		let name = match parser.consume_if_map(take_token_kind!(Token::Ident)) {
			Some(i) => i,
			None => {
				let (t, s) = parser.peek().clone().unwrap();

				return Try::Err(ParseError::ExpectedIdent(t).with_source(s));
			}
		};

		let typ = if parser.consume_if_equal(Token::Punctuation(":".to_string())).is_some() {
			let typ = require!(Type::parse(parser, ctx));
			Some(typ)
		} else {
			None
		};

		let value = if parser.consume_if_equal(Token::Punctuation("=".to_string())).is_some() {
			let expr = require!(Expression::parse(parser, ctx));
			Some(expr)
		} else {
			None
		};

		parser.consume_if_equal(Token::Punctuation(";".to_string()));

		Try::Some(Let::new(name, typ, value).with_source(start.until(parser.last_source())))
    }
}


impl Parse for Var {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<crate::ParseError>> {
        let Some(start) = parser.consume_if_equal(Token::Keyword("var".to_string())) else {
			let (t, s) = parser.peek().clone().unwrap();

			return Try::None(ParseError::ExpectedLet(t).with_source(s));
		};

		let name = match parser.consume_if_map(take_token_kind!(Token::Ident)) {
			Some(i) => i,
			None => {
				let (t, s) = parser.peek().clone().unwrap();

				return Try::Err(ParseError::ExpectedIdent(t).with_source(s));
			}
		};

		let typ = if parser.consume_if_equal(Token::Punctuation(":".to_string())).is_some() {
			let typ = require!(Type::parse(parser, ctx));
			Some(typ)
		} else {
			None
		};

		let value = if parser.consume_if_equal(Token::Punctuation("=".to_string())).is_some() {
			let expr = require!(Expression::parse(parser, ctx));
			Some(expr)
		} else {
			None
		};

		parser.consume_if_equal(Token::Punctuation(";".to_string()));

		Try::Some(Var::new(name, typ, value).with_source(start.until(parser.last_source())))
    }
}