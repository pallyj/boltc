use prelude::*;

use bolt_ast::{Attribute, FuncArg};

use crate::{Parse, Token, ParseError, Context, ParenCsl};

impl Parse for Attribute {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<crate::ParseError>> {
        let Some(start) = parser.consume_if_equal(Token::Punctuation("@".to_string())) else {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedAttribute(t).with_source(s));
		};

		let Some(name) = parser.consume_if_map(take_token_kind!(Token::Ident)) else {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedIdent(t).with_source(s));
		};

		// Parse params here (eventually)
		let args = into_result!(ParenCsl::<FuncArg>::parse(parser, ctx)).map(|args| args.unwrap().0).unwrap_or(vec![]);

		let end = parser.last_source();

		return Try::Some(Attribute::new(name, args).with_source(start.until(end)));
    }
}