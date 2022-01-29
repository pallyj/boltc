use prelude::{WithSource, Try, HasSource, require, unwrap, into_result, Source};
use syntax::{Type, TypeDecl};

use crate::{Token, ParseError, ParenCsl, BracedSemicolonList};

use super::{Parse, Parser};


// TODO: Enums
// TODO: [] after function returns it
// TODO: [] in function is an array

impl Parse for TypeDecl {
    type Output = Self;

    fn parse(parser: &mut super::Parser) -> Try<WithSource<TypeDecl>, WithSource<ParseError>> {
		let Some(start) = parser.next_source() else {
			return Try::None(ParseError::EOF.with_source(parser.last_source()))
		};
        let mut base = require!(type_pointer(parser));

		let mut name = String::new();

		let Some(t) = parser.peek() else {
			return Try::Some(base.map(|typ| {
				TypeDecl {
					name,
					typ,
				}
			}));
		};

		let (t, _) = t.clone().unwrap();

		match t {
			Token::Keyword(kw) if kw.as_str() == "const" => {
				parser.consume();
				let source = start.clone().until(parser.last_source());
				base = Type::Const(Box::new(base)).with_source(source);
			}
			Token::Ident(bind_name) => {
				parser.consume();
				name = bind_name.to_string();
				base = require!(make_array(parser, base, &start));

				return Try::Some(base.map(|typ| {
					TypeDecl {
						name,
						typ,
					}
				}));
			}
			Token::Symbol(sym) if sym.as_str() == "(" => {
				let decl = require!(function_pointer(parser, base));
				let source = start.clone().until(parser.last_source());
				base = decl.typ.with_source(source);
				name = decl.name;
			}
			_ => {}
		}

		Try::Some(base.map(|typ| {
			TypeDecl {
				name,
				typ,
			}
		}))
    }
}

fn make_array(parser: &mut super::Parser, mut base: WithSource<Type>, start: &Source) -> Try<WithSource<Type>, WithSource<ParseError>> {
	loop {
		if !parser.consume_if_equal(Token::Symbol("[".to_string())) {
			return Try::Some(base);
		}
		let next_int = parser.consume_if(is_token_kind!(Token::NumberLit));

		let Some(next_int) = next_int else {
			let (t, s) = parser.peek().unwrap().clone().unwrap();
			return Try::Err(ParseError::ExpectedArrayLen(t).with_source(parser.last_source()))
		};

		match next_int.value() {
			Token::NumberLit(n) => {
				let n = *n;
				if !parser.consume_if_equal(Token::Symbol("]".to_string())) {
					return Try::Err(ParseError::EOF.with_source(parser.last_source())) // TODO: Expected ]
				}
				let source = start.clone().until(parser.last_source());
				base = Type::Array(Box::new(base), n).with_source(source);
			}
			_ => {
				return Try::Err(ParseError::EOF.with_source(parser.last_source())) // TODO: Expected length bound
			}
		}
	}
}

fn function_pointer(parser: &mut super::Parser, base: WithSource<Type>) -> Try<TypeDecl, WithSource<ParseError>> {
	parser.consume();
	if !parser.consume_if_equal(Token::Operator("*".to_string())) {
		let (t, s) = parser.peek().unwrap().clone().unwrap(); // TODO: Errors can take none
		return Try::Err(ParseError::ExpectedFunctionPointerStar(t).with_source(s))
	}

	// Get the name
	let name = parser.consume_if_map(take_token_kind!(Token::Ident)).unwrap_or("".to_string());

	if !parser.consume_if_equal(Token::Symbol(")".to_string())) {
		let (t, s) = parser.peek().unwrap().clone().unwrap(); // TODO: Errors can take none
		return Try::Err(ParseError::ExpectedClosingParen(t).with_source(s))
	}

	let (pars, _) = require!(ParenCsl::<TypeDecl>::parse(parser)).unwrap();

	Try::Some(TypeDecl {
		name,
		typ: Type::FuncPtr(Box::new(base), pars.into_iter().map(|p| Box::new(p)).collect() ),
	})
}

fn type_pointer(parser: &mut super::Parser) -> Try<WithSource<Type>, WithSource<ParseError>> {
	let atom = unwrap!(type_atom(parser));

	let star = match parser.peek() {
		Some(star) => star,
		None => return Try::Some(atom),
	};

	match star.value() {
		Token::Operator(op) if op.as_str() == "*" => {
			// TODO: Find a better way to do this
			parser.consume();
			Try::Some(Type::Pointer(Box::new(atom)).with_source(parser.last_source()))
		}
		_ => Try::Some(atom)
	}

}

fn type_atom(parser: &mut super::Parser) -> Try<WithSource<Type>, WithSource<ParseError>> {
	parser.slice_map(|parser| -> Try<Type, ParseError> {
		// TODO: Don't unwrap the first time
		let (head, _) = match parser.consume() {
			Some(x) => x.clone(),
			None => return Try::None(ParseError::EOF),
		}.unwrap();

		match head {
			Token::Ident(id) => {
				Try::Some(Type::Named(id.clone()))
			}
			Token::Keyword(kw) => {
				Try::Some(match kw.as_str() {
					"const" => {
						let atom = type_atom(parser);
						Type::Const(Box::new(unwrap!(atom.discard_error_source())))
					}
					"struct" => {
						// TODO: We can have implicit definition
						require!(parse_struct(parser).discard_error_source())
					}
					"enum" => {
						// TODO: We can have implicit definition
						let enum_name = require!(ident_anon(parser));

						Type::StructRef(enum_name)
					}
					"union" => {
						// TODO: We can have implicit definition
						require!(parse_union(parser).discard_error_source())
					}
					"void" => {
						Type::Unit
					}
					intrinsic @ ("unsigned" | "signed" | "long" | "int" | "short" | "char" | "float" | "double") => {
						Type::Intrinsic(parse_intrinsic(parser, intrinsic))
					}
					_ => return Try::Err(ParseError::ExpectedType(Token::Keyword(kw.clone())))
				})
			}
			t => Try::Err(ParseError::UnexpectedToken(t.clone()))
		}
	}).take()
}

fn parse_struct(parser: &mut super::Parser) -> Try<Type, WithSource<ParseError>> {
	let name = parser.consume_if_map(take_token_kind!(Token::Ident));

	match BracedSemicolonList::<TypeDecl>::parse(parser) {
		Try::Some(items) => {
			let (items, _) = items.unwrap();
			Try::Some(Type::Struct(name, items.into_iter().map(|i| Box::new(i)).collect() ))
		}

		Try::None(_) => {
			// If name is (none), it is an error
			Try::Some(Type::StructRef(name.unwrap()))
		}

		Try::Err(e) => return Try::Err(e)
	}
}

fn parse_union(parser: &mut super::Parser) -> Try<Type, WithSource<ParseError>> {
	let name = parser.consume_if_map(take_token_kind!(Token::Ident));

	match BracedSemicolonList::<TypeDecl>::parse(parser) {
		Try::Some(items) => {
			let (items, _) = items.unwrap();
			Try::Some(Type::Union(name, items.into_iter().map(|i| Box::new(i)).collect() ))
		}

		Try::None(_) => {
			// If name is (none), it is an error
			Try::Some(Type::UnionRef(name.unwrap()))
		}

		Try::Err(e) => return Try::Err(e)
	}
}

fn parse_intrinsic(parser: &mut super::Parser, head: &str) -> String {
	let mut rest = head.to_string();
	
	while let Some(intrinsic) = parser.consume_if(is_intrinsic) {
		match intrinsic.value() {
			Token::Keyword(intrinsic) =>  {
				rest.push(' ');
				rest.push_str(intrinsic)
			}
			_ => {panic!()}
		}
	}

	rest
}

fn ident_anon(parser: &mut Parser) -> Try<String, ParseError> {
	if let Some(struct_name) = parser.peek() {
		let (struct_name, _) = struct_name.as_ref().unwrap();

		match struct_name {
			Token::Ident(id) => { let id = id.clone(); parser.consume(); Try::Some(id) },
			t => {
				// x is not a valid struct name
				Try::Err(ParseError::ExpectedIdentInStruct(t.clone()))
			}
		}
	} else {
		// Reached EOF
		Try::Err(ParseError::EOF)
	}
}

fn is_intrinsic(token: &Token) -> bool {
	match token {
		Token::Keyword(kw)
			if kw.as_str() == "unsigned" ||
			   kw.as_str() == "signed" ||
			   kw.as_str() == "char" ||
			   kw.as_str() == "short" ||
			   kw.as_str() == "int" ||
			   kw.as_str() == "long" ||
			   kw.as_str() == "float" ||
			   kw.as_str() == "double"
			  => true,
		_ => false,
	}
}