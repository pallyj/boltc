use bolt_ast::{Enum, Type, EnumVariant, Expression};
use prelude::{require, Try, HasSource, WithSource, into_result};

use crate::{Parse, Token, BracedList, ParseError, Csl, ParenCsl, Context};

impl Parse for Enum {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> prelude::Try<prelude::WithSource<Self::Output>, prelude::WithSource<crate::ParseError>> {
        let Some(kw) = parser.consume_if_equal(Token::Keyword("enum".to_string())) else {
			let (t, s) = parser.peek().clone().unwrap();

			return Try::None(ParseError::ExpectedEnum(t).with_source(s));
		};

		let ident = parser.consume_if_map(slice_token_kind!(Token::Ident));

		let repr = if parser.consume_if_equal(Token::Punctuation(":".to_string())).is_some() {
			Some(require!(Type::parse(parser, ctx)))
		} else {
			None
		};

		let (variants, _) = require!(BracedList::<Case>::parse(parser, ctx)).unwrap();

		let variants = variants.into_iter().flat_map(|x| x.unwrap().0).collect();

		let enumeration = Enum::new(ident, repr, variants);

		let source = kw.until(parser.last_source());

		Try::Some(enumeration.with_source(source))
    }
}

struct Case {}

impl Parse for Case {
    type Output = Vec<WithSource<EnumVariant>>;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<prelude::WithSource<Self::Output>, prelude::WithSource<crate::ParseError>> {
        let Some(start) = parser.consume_if_equal(Token::Keyword("case".to_string())) else {
			// error
			let (t, s) = parser.peek().clone().unwrap();

			return Try::None(ParseError::ExpectedEnumItem(t).with_source(s));
		};

		let variants = require!(Csl::<EnumVariant>::parse(parser, ctx));

		let source = start.until(parser.last_source());

		Try::Some(variants)
    }
}

impl Parse for EnumVariant {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<crate::ParseError>> {
		let Some(name) = parser.consume_if_map(slice_token_kind!(Token::Ident)) else {
			let (t, s) = parser.peek().clone().unwrap();

			return Try::None(ParseError::ExpectedEnumVariant(t).with_source(s))
		};

		let name_src = name.source().clone();

		let associated = into_result!(ParenCsl::<Type>::parse(parser, ctx))
			.map(|m| m.unwrap().0)
			.unwrap_or(vec![]);

        let mut variant = EnumVariant::new(name, associated);

		if parser.consume_if_equal(Token::Punctuation("=".to_string())).is_some() {
			variant.assign_value(require!(Expression::parse(parser, ctx)));
		}

		let source = name_src.until(parser.last_source());

		Try::Some(variant.with_source(source))
    }
}