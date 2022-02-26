use bolt_ast::{Struct, Type, StructItem, Attribute, Visibility, Func, Var, Let};
use prelude::{Try, HasSource, require, into_result};

use crate::{Parse, Token, ParseError, BracedList, List, Context};

impl Parse for Struct {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> prelude::Try<prelude::WithSource<Self::Output>, prelude::WithSource<crate::ParseError>> {
        let Some(struct_kw) = parser.consume_if_equal(Token::Keyword("struct".to_string())) else {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedStruct(t).with_source(s));
		};

		let name = parser.consume_if_map(take_token_kind!(Token::Ident));

		let mut implements = vec![];

		if parser.consume_if_equal(Token::Punctuation(":".to_string())).is_some() {
			while let Ok(imp) = into_result!(Type::parse(parser, ctx)) {
				implements.push(imp);
			}
		}

		let (items, _) = require!(BracedList::<StructItem>::parse(parser, ctx)).unwrap();

		let struct_def = Struct::new(
			name,
			implements,
			items,
		);

		let source = struct_kw.until(parser.last_source());

		Try::Some(struct_def.with_source(source))
    }
}

impl Parse for StructItem {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<prelude::WithSource<Self::Output>, prelude::WithSource<ParseError>> {
        let (attributes, _) = require!(List::<Attribute>::parse(parser, ctx)).unwrap();
        let visibility = into_result!(Visibility::parse(parser, ctx)).ok();

		if let Ok(func) = into_result!(Func::parse(parser, ctx)) {
			Try::Some(func.map(|f| StructItem::Method(f
				.with_visibility(visibility)
				.with_attributes(attributes))))
		} else if let Ok(substruct) = into_result!(Struct::parse(parser, ctx)) {
			Try::Some(substruct.map(|s| StructItem::SubStruct(s
				.with_visibility(visibility)
				.with_attributes(attributes))))
		} else if let Ok(v) = into_result!(Var::parse(parser, ctx)) {
			Try::Some(v.map(|v| StructItem::Variable(v
				.with_visibility(visibility) 
				.with_attributes(attributes))))
		} else if let Ok(l) = into_result!(Let::parse(parser, ctx)) {
			Try::Some(l.map(|l| StructItem::Let(l
				.with_visibility(visibility)
				.with_attributes(attributes))))
		} else {
			let (t, s) = parser.peek().clone().unwrap();

			if visibility.is_some() || attributes.len() > 0 {
				return Try::Err(ParseError::ExpectedStructItem(t).with_source(s));
			}

			return Try::None(ParseError::ExpectedStructItem(t).with_source(s));
		}
    }
}