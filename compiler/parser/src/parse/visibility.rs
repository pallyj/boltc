use bolt_ast::Visibility;
use prelude::*;

use crate::{Parse, Token, ParseError, Context};

impl Parse for Visibility {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, _ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<crate::ParseError>> {
        let (t, s) = parser.peek().clone().unwrap();

		match t {
			Token::Keyword(kw) if kw.as_str() == "public" => {
				parser.consume();
				return Try::Some(Visibility::Public.with_source(s))
			}
			Token::Keyword(kw) if kw.as_str() == "internal" => {
				parser.consume();
				return Try::Some(Visibility::Internal.with_source(s))
			}
			Token::Keyword(kw) if kw.as_str() == "fileprivate" => {
				parser.consume();
				return Try::Some(Visibility::Fileprivate.with_source(s))
			}
			Token::Keyword(kw) if kw.as_str() == "private" => {
				parser.consume();
				return Try::Some(Visibility::Private.with_source(s))
			}
			_ => return Try::None(ParseError::ExpectedVisibility(t).with_source(s))
		}
    }
}