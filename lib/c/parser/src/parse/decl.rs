use prelude::{unwrap, require, Try, HasSource};
use syntax::{Decl, TypeDecl, TypeDef, Variable, Function};

use crate::{Parse, Token, ParseError, ParenCsl, BracedSemicolonList};

use super::typ::StructItem;

impl Parse for Decl {
    type Output = Decl;

    fn parse(parser: &mut crate::Parser) -> prelude::Try<prelude::WithSource<Self::Output>, prelude::WithSource<crate::ParseError>> {
		if parser.consume_if_equal(Token::Keyword("typedef".to_string())) {
			let typedecl = require!(StructItem::parse(parser));

			if !parser.consume_if_equal(Token::Symbol(";".to_string())) {
				let (t, s) = parser.peek().clone().unwrap();
				return Try::Err(ParseError::MissingSemicolon(t).with_source(s))
			}

			return Try::Some(typedecl.map(|typedecl| {
				Decl::TypeDef(TypeDef {
					name: typedecl.name.unwrap(),
					typ: typedecl.typ,
				})
			}));
		}

        let (typedecl, _) = unwrap!(TypeDecl::parse(parser)).unwrap();

		let (peek, source) = parser.peek().clone().unwrap();
		match peek {
			Token::Symbol(s) if s.as_str() == ";" => {
				parser.consume();
				// TODO: Get the source right
				return Try::Some(Decl::Variable(Variable {
					name: typedecl.name,
					typ: typedecl.typ,
				})).with_source(source).take();
			}
			Token::Symbol(s) if s.as_str() == "(" => {
				let (pars, _) = require!(ParenCsl::<TypeDecl>::parse(parser)).unwrap();

				let code = require!(BracedSemicolonList::<TypeDecl>::parse(parser));

				if typedecl.name.is_none() {
					let (t, s) = parser.peek().clone().unwrap();
	
					return Try::Err(ParseError::ExpectedIdentInStruct(t).with_source(s))
				}

				return Try::Some(Decl::Function(Function {
					return_type: typedecl.typ,
					name: typedecl.name.unwrap(),
					params: pars,
					code: ()
				})).with_source(source).take()
			}
			_ => {
				let (t, s) = parser.peek().clone().unwrap();
				return Try::Err(ParseError::MissingSemicolon(t).with_source(s))
			}
		}
    }
}