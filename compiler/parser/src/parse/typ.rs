use bolt_ast::Type;
use prelude::*;

use crate::{Token, ParseError, Context};

use super::Parse;

impl Parse for Type {
    type Output = Self;

    fn parse(parser: &mut super::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<crate::ParseError>> {
		// Check for a named type
        if let Some(token) = parser.consume_if_map(take_token_kind!(Token::Ident)) {
			let last_source = parser.last_source();

			return Try::Some(Type::Named(token).with_source(last_source))
		}

		else {
			let (t, s) = parser.peek().clone().unwrap();
			Try::None(ParseError::ExpectedType(t).with_source(s))
		}
    }
}