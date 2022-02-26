use bolt_ast::Import;
use prelude::*;

use crate::{Parse, ParseError, Token, Context};

impl Parse for Import {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<crate::ParseError>> {
        let Some(start_span) = parser.consume_if_equal(Token::Keyword("import".to_string())) else {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedImport(t).with_source(s))
		};

		if let Some(ident) = parser.consume_if_map(take_token_kind!(Token::Ident)) {
			parser.consume_if_equal(Token::Punctuation(";".to_string()));
			Try::Some(Import::new(ident).with_source(start_span.until(parser.last_source())))
		} else {
			let (t, s) = parser.peek().clone().unwrap();
			Try::Err(ParseError::ExpectedIdent(t).with_source(s))
		}
    }
}