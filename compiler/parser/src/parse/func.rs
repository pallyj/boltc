use bolt_ast::{Func, CodeBlock, FuncPar, Type};
use prelude::*;

use crate::{Token, ParseError, Context, Braced};

use super::{Parse, ParenCsl};

impl Parse for Func {
    type Output = Self;

    fn parse(parser: &mut super::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<crate::ParseError>> {
		let (start_tok, start_span) = parser.peek().clone().unwrap();

		let is_static = parser.consume_if_equal(Token::Keyword("static".to_string())).is_some();
		let is_mutating = parser.consume_if_equal(Token::Keyword("mutating".to_string())).is_some();

		let name = if parser.consume_if_equal(Token::Keyword("func".to_string())).is_some() {
			parser.consume_if_map(take_token_kind!(Token::Ident))
		} else if parser.consume_if_equal(Token::Keyword("init".to_string())).is_some() {
			Some("init".to_string())
		} else {
			return Try::None(ParseError::ExpectedFunc(start_tok).with_source(start_span));
		};

		let (pars, _) = require!(ParenCsl::<FuncPar>::parse(parser, ctx)).unwrap();

		let return_typ = if parser.consume_if_equal(Token::Punctuation(":".to_string())).is_some() {
			Some(require!(Type::parse(parser, ctx)))
		} else {
			None
		};

		let code = into_result!(Braced::<CodeBlock>::parse(parser, ctx)).ok();

		let span = start_span.until(parser.last_source());

		if let Some(code) = code {
			return Try::Some(Func::new(is_static, is_mutating, name, pars, return_typ, code).with_source(span));
		} else {
			return Try::Some(Func::new_extern(name, pars, return_typ).with_source(span));
		}
    }
}

impl Parse for FuncPar {
    type Output = Self;

    fn parse(parser: &mut super::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		let (start_tok, start_source) = parser.peek().clone().unwrap();

        let Some(first) = parser.consume_if_map(take_token_kind!(Token::Ident)) else {
			return Try::None(ParseError::ExpectedIdent(start_tok).with_source(start_source));
		};

		let label: Option<String>;
		let bind: String;

		if let Some(second) = parser.consume_if_map(take_token_kind!(Token::Ident)) {
			label = Some(first);
			bind = second;
		} else {
			label = None;
			bind = first;
		}

		if parser.consume_if_equal(Token::Punctuation(":".to_string())).is_none() {
			let (colon_tok, colon_source) = parser.peek().clone().unwrap();

			return Try::Err(ParseError::ExpectedColon(colon_tok).with_source(colon_source));
		}

		let typ = require!(Type::parse(parser, ctx));

		let last_source = parser.last_source();

		return Try::Some(FuncPar::new(label, bind, typ).with_source(start_source.until(last_source)));
    }
}