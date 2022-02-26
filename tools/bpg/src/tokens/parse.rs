use std::marker::PhantomData;

use prelude::*;

use super::{ast::{TokenParam, TokenDef, Ast}, token::Token, error::ParseError};

pub type Parser = prelude::Parser<Token, ParseError>;


macro_rules! is_token_kind {
	($tt:path) => {
		|tok| match tok {
			$tt(_) => true,
			_ => false
		}
	};
}

macro_rules! take_token_kind {
	($tt:path) => {
		|tok| match tok.value() {
			$tt(v) => Some(v.clone()),
			_ => None
		}
	};
}

pub trait Parse {
	type Output;

	fn parse(parser: &mut Parser) -> Try<WithSource<Self::Output>, WithSource<ParseError>>;
}

pub struct ParenCsl<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for ParenCsl<T> {
    type Output = Vec<WithSource<T::Output>>;

    fn parse(parser: &mut Parser) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        if parser.consume_if_equal(Token::OpenParen).is_none() {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedOpenParen(t).with_source(s))
		}

		let mut items = vec![];

		loop {
			match T::parse(parser) {
				Try::Some(t) => items.push(t),
				Try::None(_) => {
					let (next_t, next_s) = parser.peek().clone().unwrap();
					return Try::Err(ParseError::ExpectedCloseParen(next_t).with_source(next_s));
				}
				Try::Err(err) => return Try::Err(err)
			}

			if let Some(source) = parser.consume_if_equal(Token::CloseParen) {
				return Try::Some(items.with_source(source))
			} 

			if parser.consume_if_equal(Token::Comma).is_none() {
				if let Some(source) = parser.consume_if_equal(Token::CloseParen) {
					return Try::Some(items.with_source(source))
				} else {
					let (next_tok, _) = parser.peek().clone().unwrap();

					return Try::Err(ParseError::ExpectedCloseParen(next_tok).with_source(parser.last_source()))
				}
			}
		}
    }
}

impl Parse for TokenParam {
    type Output = Self;

    fn parse(parser: &mut Parser) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        if let Some(next) = parser.consume_if(is_token_kind!(Token::Ident)).cloned() {
			let (next_t, next_s) = next.unwrap();

			match next_t {
				Token::Ident(id) if id.as_str() == "String" => Try::Some(TokenParam::String.with_source(next_s)),
				Token::Ident(id) if id.as_str() == "UInt" => Try::Some(TokenParam::UInt.with_source(next_s)),
				Token::Ident(id) if id.as_str() == "Int" => Try::Some(TokenParam::Int.with_source(next_s)),
				_ => {
					// Return error expected token type
					return Try::None(ParseError::ExpectedType(next_t).with_source(next_s))
				}

			}
		} else {
			// Return error expected token type, found EOF
			let next = parser.peek();
			let (next_t, next_s) = next.clone().unwrap();

			return Try::None(ParseError::ExpectedType(next_t).with_source(next_s))
		}
    }
}

impl Parse for TokenDef {
    type Output = Self;

    fn parse(parser: &mut Parser) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		let start = parser.next_source();
        if let Some(token_name) = parser.consume_if_map(take_token_kind!(Token::Ident)) {
			let (params_t, params_s) = into_result!(ParenCsl::<TokenParam>::parse(parser))
				.map(|p| p.unwrap())
				.unwrap_or((vec![], parser.next_source()));

			let source = start.until(params_s);

			return Try::Some(TokenDef::new(token_name.with_source(start), params_t).with_source(source));
		} else {
			// Return error expected token name
			let (next, _) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedIdent(next).with_source(start));
		}
    }
}

impl Parse for Ast {
    type Output = Self;

    fn parse(parser: &mut Parser) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		let start = parser.next_source();

        let mut defs = vec![];

		while !parser.is_at_eof() {
			defs.push(require!(TokenDef::parse(parser)));
		}

		let end = parser.last_source();

		Try::Some(Ast::new(defs).with_source(start.until(end)))
    }
}