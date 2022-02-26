use bolt_ast::{Decl, Visibility, Func, Attribute, Import, Let, Struct, Enum};
use prelude::{into_result, Try, HasSource, require};

use crate::{Parse, ParseError, List, Context};

impl Parse for Decl {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> prelude::Try<prelude::WithSource<Self::Output>, prelude::WithSource<crate::ParseError>> {
		if let Ok(import) = into_result!(Import::parse(parser, ctx)) {
			return Try::Some(import.map(|i| Decl::Import(i)));	
		}

		let (attributes, _) = require!(List::<Attribute>::parse(parser, ctx)).unwrap();
        let visibility = into_result!(Visibility::parse(parser, ctx)).ok();

		if let Ok(func) = into_result!(Func::parse(parser, ctx)) {
			Try::Some(func.map(|f| Decl::Func(f
				.with_visibility(visibility)
				.with_attributes(attributes))))
		} else if let Ok(let_decl) = into_result!(Let::parse(parser, ctx)) {
			Try::Some(let_decl.map(|l| Decl::Let(l
				.with_visibility(visibility)
				.with_attributes(attributes))))
		} else if let Ok(struct_decl) = into_result!(Struct::parse(parser, ctx)) {
			Try::Some(struct_decl.map(|s| Decl::Struct(s
				.with_visibility(visibility)
				.with_attributes(attributes))))
		} else if let Ok(enum_decl) = into_result!(Enum::parse(parser, ctx)) {
			Try::Some(enum_decl.map(|e| Decl::Enum(e
				.with_visibility(visibility)
				.with_attributes(attributes))))
		} else {
			let (t, s) = parser.peek().clone().unwrap();
			return Try::None(ParseError::ExpectedDecl(t).with_source(s));
		}
    }
}