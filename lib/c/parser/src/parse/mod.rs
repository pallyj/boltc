use std::marker::PhantomData;

use prelude::{Try, WithSource, HasSource, require};

use crate::{Token, ParseError};

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

mod typ;
mod decl;

pub type Parser = prelude::Parser<Token, ParseError>;

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
        if !parser.consume_if_equal(Token::Symbol("(".to_string())) {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedOpeningParen(t).with_source(s))
		}

		let mut items = vec![];

		loop {
			let next = parser.peek();
			if parser.consume_if_equal(Token::Symbol(")".to_string())) {
				return Try::Some(items.with_source(parser.last_source()))
			}

			items.push(require!(T::parse(parser)));

			if !parser.consume_if_equal(Token::Symbol(",".to_string())) {
				break;
			}
		}

		if parser.consume_if_equal(Token::Symbol(")".to_string())) {
			return Try::Some(items.with_source(parser.last_source()))
		}

		//

		return Try::Err(ParseError::EOF.with_source(parser.last_source())) // TODO: NotFound
    }
}


pub struct BracedSemicolonList<T: Parse> {
	_phantom: PhantomData<T>
}

impl<T: Parse> Parse for BracedSemicolonList<T> {
    type Output = Vec<WithSource<T::Output>>;

    fn parse(parser: &mut Parser) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        if !parser.consume_if_equal(Token::Symbol("{".to_string())) {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedOpeningBrace(t).with_source(s))
		}

		let mut items = vec![];

		while !parser.is_at_eof() {
			if parser.consume_if_equal(Token::Symbol("}".to_string())) {
				return Try::Some(items.with_source(parser.last_source()))
			}

			items.push(require!(T::parse(parser)));

			if !parser.consume_if_equal(Token::Symbol(";".to_string())) {
				let (t, s) = parser.peek().clone().unwrap();
				return Try::Err(ParseError::MissingSemicolon(t).with_source(s))
			}
		}

		if parser.consume_if_equal(Token::Symbol("}".to_string())) {
			return Try::Some(items.with_source(parser.last_source()))
		}

		return Try::Err(ParseError::EOF.with_source(parser.last_source())) // TODO: NotFound
    }
}