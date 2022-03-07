use std::marker::PhantomData;

use prelude::{Try, WithSource, HasSource, require};

use crate::{Token, ParseError, Context};

macro_rules! take_token_kind {
	($tt:path) => {
		|tok| match tok.value() {
			$tt(v) => Some(v.clone()),
			_ => None
		}
	};
}

macro_rules! slice_token_kind {
	($tt:path) => {
		|tok| match tok.value() {
			$tt(v) => Some(v.clone().with_source(tok.source().clone())),
			_ => None
		}
	};
}

pub mod expr;
pub mod statement;
pub mod func;
pub mod typ;
pub mod visibility;
pub mod decl;
pub mod attribute;
pub mod asttree;
pub mod import;
pub mod var;
pub mod struct_;
pub mod enum_;
pub mod operators;

pub type Parser = prelude::Parser<Token, ParseError>;

pub trait Parse {
	type Output;

	fn parse(parser: &mut Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>>;
}

pub struct Csl<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for Csl<T> {
    type Output = Vec<WithSource<T::Output>>;

    fn parse(parser: &mut Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		let mut items = vec![];

		let start = parser.next_source();

		loop {
			items.push(require!(T::parse(parser, ctx)));

			if parser.consume_if_equal(Token::Punctuation(",".to_string())).is_none() {
				break;
			}
		}

		let source = start.until(parser.last_source());

		Try::Some(items.with_source(source))
    }
}

pub struct ParenCsl<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for ParenCsl<T> {
    type Output = Vec<WithSource<T::Output>>;

    fn parse(parser: &mut Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        if parser.consume_if_equal(Token::Punctuation("(".to_string())).is_none() {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedOpenParen(t).with_source(s))
		}

		let mut items = vec![];

		loop {
			if let Some(source) = parser.consume_if_equal(Token::Punctuation(")".to_string())) {
				return Try::Some(items.with_source(source))
			}

			items.push(require!(T::parse(parser, ctx)));

			if parser.consume_if_equal(Token::Punctuation(",".to_string())).is_none() {
				break;
			}
		}

		if let Some(source) = parser.consume_if_equal(Token::Punctuation(")".to_string())) {
			return Try::Some(items.with_source(source))
		}

		let (next_tok, _) = parser.peek().clone().unwrap();

		return Try::Err(ParseError::ExpectedCloseParen(next_tok).with_source(parser.last_source())) // TODO: NotFound
    }
}

pub struct BracketedCsl<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for BracketedCsl<T> {
    type Output = Vec<WithSource<T::Output>>;

    fn parse(parser: &mut Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        if parser.consume_if_equal(Token::Punctuation("[".to_string())).is_none() {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedOpenParen(t).with_source(s))
		}

		let mut items = vec![];

		loop {
			if let Some(source) = parser.consume_if_equal(Token::Punctuation("]".to_string())) {
				return Try::Some(items.with_source(source))
			}

			items.push(require!(T::parse(parser, ctx)));

			if parser.consume_if_equal(Token::Punctuation(",".to_string())).is_none() {
				break;
			}
		}

		if let Some(source) = parser.consume_if_equal(Token::Punctuation("]".to_string())) {
			return Try::Some(items.with_source(source))
		}

		let (next_tok, _) = parser.peek().clone().unwrap();

		return Try::Err(ParseError::ExpectedCloseParen(next_tok).with_source(parser.last_source())) // TODO: NotFound
    }
}

pub struct Bracketed<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for Bracketed<T> {
    type Output = T::Output;

    fn parse(parser: &mut Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        if parser.consume_if_equal(Token::Punctuation("[".to_string())).is_none() {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedOpenBracket(t).with_source(s))
		}

		let item = require!(T::parse(parser, ctx));

		if parser.consume_if_equal(Token::Punctuation("]".to_string())).is_none() {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::Err(ParseError::ExpectedCloseBracket(t).with_source(s))
		}

		return Try::Some(item);
    }
}

pub struct Braced<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for Braced<T> {
    type Output = T::Output;

    fn parse(parser: &mut Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        if parser.consume_if_equal(Token::Punctuation("{".to_string())).is_none() {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedOpenBrace(t).with_source(s))
		}

		let item = require!(T::parse(parser, ctx));

		if parser.consume_if_equal(Token::Punctuation("}".to_string())).is_none() {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::Err(ParseError::ExpectedCloseBrace(t).with_source(s))
		}

		return Try::Some(item);
    }
}


pub struct BracedList<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for BracedList<T> {
    type Output = Vec<WithSource<T::Output>>;

    fn parse(parser: &mut Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        let Some(start_source) = parser.consume_if_equal(Token::Punctuation("{".to_string())) else {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedOpenBrace(t).with_source(s))
		};

		let mut items = vec![];

		while !parser.is_at_eof() {
			if let Some(source) = parser.consume_if_equal(Token::Punctuation("}".to_string())) {
				return Try::Some(items.with_source(source))
			}

			items.push(require!(T::parse(parser, ctx)));
		}

		if let Some(source) = parser.consume_if_equal(Token::Punctuation("}".to_string())) {
			return Try::Some(items.with_source(source))
		}

		return Try::Err(ParseError::ExpectedCloseBrace(parser.peek().value().clone()).with_source(start_source.until(parser.last_source()))) // TODO: NotFound
    }
}

pub struct List<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for List<T> {
    type Output = Vec<WithSource<T::Output>>;

    fn parse(parser: &mut Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		let mut items = vec![];

		let start = parser.next_source();

		loop {
			match T::parse(parser, ctx) {
				Try::Some(item) => items.push(item),
				Try::None(_) => return Try::Some(items.with_source(start.until(parser.last_source()))),
				Try::Err(e) => {
					return Try::Err(e)
				}
			}
		}
    }
}