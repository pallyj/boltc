use bolt_ast::{Expression, FuncArg, CodeBlock, CodeBlockItem, Statement};
use prelude::{Try, WithSource, HasSource, into_result, require, unwrap};

use crate::{ParseError, Token, ParenCsl, Context, operators::{OperatorPrecedence, OperatorFix}, Bracketed, Braced, BracketedCsl};

use super::Parse;

impl Parse for Expression {
    type Output = Self;

    fn parse(parser: &mut super::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		expression_inner(parser, ctx, OperatorPrecedence::None, false)
    }
}

pub fn expression_inner(parser: &mut super::Parser, ctx: &Context, precedence: OperatorPrecedence, before_brace: bool) -> Try<WithSource<Expression>, WithSource<ParseError>> {
	let mut expr = unwrap!(expression_atom(parser, ctx, before_brace));

	loop {
		// Parse a member
		if parser.consume_if_equal(Token::Punctuation(".".to_string())).is_some() {
			let Some(ident) = parser.consume_if_map(take_token_kind!(Token::Ident)) else {
				let (t, s) = parser.peek().clone().unwrap();

				return Try::Err(ParseError::ExpectedMemberName(t).with_source(s));
			};

			let source = expr.source().until(parser.last_source());

			expr = Expression::Member(Box::new(expr), ident).with_source(source);
		}
		// Parse an optional member
		else if parser.consume_if_equal(Token::Punctuation("?.".to_string())).is_some() {
			let Some(ident) = parser.consume_if_map(take_token_kind!(Token::Ident)) else {
				let (t, s) = parser.peek().clone().unwrap();

				return Try::Err(ParseError::ExpectedMemberName(t).with_source(s));
			};

			let source = expr.source().until(parser.last_source());

			expr = Expression::OptionalMember(Box::new(expr), ident).with_source(source);
		}
		// Parse an index
		else if let Ok(idx) = into_result!(Bracketed::<Expression>::parse(parser, ctx)) {
			let source = expr.source().until(idx.source().clone());

			expr = Expression::Index(Box::new(expr), Box::new(idx)).with_source(source);
		}
		// Parse a function call
		else if let Ok(args) = into_result!(ParenCsl::<FuncArg>::parse(parser, ctx)) {
			// TODO: Add trailing closures
			let args = args.unwrap().0;

			let source = expr.source().until(parser.last_source());

			expr = Expression::FuncCall(Box::new(expr), args).with_source(source);
		}
		// Parse an operator
		else if let Some(postfix_op) = ctx.factory().parse_postfix_operator(parser, precedence)  {
			if postfix_op.fix() == OperatorFix::Infix {
				let rhs = require!(expression_inner(parser, ctx, postfix_op.precedence(), before_brace));

				let source = expr.source().until(parser.last_source());

				expr = Expression::InfixOperator(Box::new(expr), postfix_op.name().clone(), Box::new(rhs)).with_source(source);
			} else {
				let source = expr.source().until(parser.last_source());

				expr = Expression::FixOperator(Box::new(expr), postfix_op.name().clone()).with_source(source);
			}
		}
		// Parse an apply
		else if parser.consume_if_equal(Token::Punctuation("->".to_string())).is_some() {
			let func = require!(expression_inner(parser, ctx, OperatorPrecedence::Prefix, before_brace));

			let source = expr.source().until(parser.last_source());

			expr = Expression::Apply {
				func: Box::new(func),
				to_expr: Box::new(expr),
			}.with_source(source);
		}
		
		else {
			break
		}
	}

	Try::Some(expr)
}

fn expression_atom(parser: &mut super::Parser, ctx: &Context, before_brace: bool) -> Try<WithSource<Expression>, WithSource<ParseError>> {
	// Check for an identifier
	if let Some(ident) = parser.consume_if_map(slice_token_kind!(Token::Ident)) {
		let (id, s) = ident.unwrap();
		return Try::Some(Expression::Named(id).with_source(s));
	}
	// Check for an integer literal
	else if let Some(token) = parser.consume_if_map(slice_token_kind!(Token::IntLit)) {
		let (t, s) = token.clone().unwrap();

		return Try::Some(Expression::IntLiteral(t).with_source(s))
	}
	// Check for a string literal
	else if let Some(string_lit) = parser.consume_if_map(slice_token_kind!(Token::StringLit)) {
		let (string, s) = string_lit.unwrap();
		return Try::Some(Expression::StringLiteral(string).with_source(s));
	}
	// Check for a float literal
	else if let Some(float_lit) = parser.consume_if_map(slice_token_kind!(Token::FloatLit)) {
		let (float, s) = float_lit.unwrap();
		return Try::Some(Expression::FloatLiteral(float).with_source(s));
	}
	// Check for a tuple or a parenthesized expression
	else if let Ok(csl) = into_result!(ParenCsl::<Expression>::parse(parser, ctx)) {
		match csl.value().len() {
			0 => { 
				let (t, s) = parser.peek().clone().unwrap();
				return Try::Err(ParseError::EmptyTuple(t).with_source(s)) }
			1 => { return Try::Some(csl.map(|v| v[0].value().clone())) }
			_ => {
				let (tuple, s) = csl.unwrap();
				return Try::Some(Expression::Tuple(tuple).with_source(s))
			}
		}
	}
	// Check for a try operator
	else if let Some(start) = parser.consume_if_equal(Token::Keyword("try".to_string())) {
		let operator = match parser.peek().value() {
			Token::Punctuation(ref bang) if bang.as_str() == "!" => "tryPanic",
			Token::Punctuation(ref question) if question.as_str() == "?" => "tryUnwrap",
			_ => "tryThrow"
		}.to_string();

		let expr = Box::new(require!(Expression::parse(parser, ctx)));

		let end = parser.last_source();

		return Try::Some(Expression::FixOperator(expr, operator).with_source(start.until(end)));
	}
	// Check for a prefix operator
	else if let Some(prefix_op) = ctx.factory().parse_prefix_operator(parser) {
		let start = parser.last_source();
		let unit = require!(expression_inner(parser, ctx, OperatorPrecedence::Prefix, before_brace)); // Parse with prefix precedence
		let source = start.until(parser.last_source());

		return Try::Some(Expression::FixOperator(Box::new(unit), prefix_op.name().clone()).with_source(source))
	}
	// Check for an enum variant
	else if let Some(start) = parser.consume_if_equal(Token::Punctuation(".".to_string())) {
		let Some(ident) = parser.consume_if_map(take_token_kind!(Token::Ident)) else {
			let (t, s) = parser.peek().clone().unwrap();

			return Try::Err(ParseError::ExpectedEnumVariantLiteral(t).with_source(s));
		};

		let source = start.until(parser.last_source());

		return Try::Some(Expression::Variant(ident).with_source(source));
	}
	// Check for a function
	
	// Check for an list or a record literal
	else if let Ok(items) = into_result!(BracketedCsl::<MultipleLiteralItem>::parse(parser, ctx)) {
		let is_collection = items.value().iter().all(|i| i.value().is_collection());
		let is_record = items.value().iter().all(|i| i.value().is_record());

		if !(is_collection || is_record) {
			// Mixed collection and record
			println!("Mixed");
		}

		if is_collection {
			let (items, source) = items.unwrap();

			let collection_items = items
				.into_iter()
				.map(|item| {
					let (item, source) = item.unwrap();
					match item {
						MultipleLiteralItem::Collection(expr) => expr.with_source(source),
						_ => panic!("Unexpected record literal")
					}
				})
				.collect();

			Try::Some(Expression::CollectionLiteral(collection_items).with_source(source))
		} else {
			let (items, source) = items.unwrap();

			let record_items = items
				.into_iter()
				.map(|item| {
					let item = item.unwrap().0;
					match item {
						MultipleLiteralItem::Record { key, value } => (key, value),
						_ => panic!("Unexpected collection literal")
					}
				})
				.collect();

			Try::Some(Expression::RecordLiteral(record_items).with_source(source))
		}
	}
	// Check for an if
	else if parser.consume_if_equal(Token::Keyword("if".to_string())).is_some() {
		return expression_if_body(parser, ctx);
	}
	// Return an error
	else {
		let (t, s) = parser.peek().clone().unwrap();
		return Try::None(ParseError::ExpectedExpr(t).with_source(s))
	}
}

impl Parse for FuncArg {
    type Output = FuncArg;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
		let start = parser.next_source();

        let label = match (parser.peek().value(), parser.peek_ahead(1).value()) {
			(Token::Ident(label), Token::Punctuation(colon)) if colon.as_str() == ":" => {
				let label = label.clone();
				parser.skip(2);
				Some(label)
			}
			_ => None
		};

		let arg = require!(Expression::parse(parser, ctx));

		let source = start.until(parser.last_source());

		Try::Some(FuncArg::new(label, arg).with_source(source))
    }
}

fn expression_if_body(parser: &mut super::Parser, ctx: &Context) -> Try<WithSource<Expression>, WithSource<ParseError>> {
	let start = parser.last_source();

	let condition = require!(expression_inner(parser, ctx, OperatorPrecedence::None, true));

	let positive = require!(Braced::<CodeBlock>::parse(parser, ctx));

	let negative = if parser.consume_if_equal(Token::Keyword("else".to_string())).is_some() {
		if parser.consume_if_equal(Token::Keyword("if".to_string())).is_some() {
			let expr = require!(expression_if_body(parser, ctx));
			let eval = expr.map(|expr| Statement::Expr(expr));

			let src = eval.source().clone();

			Some(CodeBlock::new(vec![ CodeBlockItem::new(eval, false) ]).with_source(src))
		} else {
			Some(require!(Braced::<CodeBlock>::parse(parser, ctx)))
		}
	} else {
		None
	};

	let source = start.until(parser.last_source());

	return Try::Some(Expression::If {
		condition: Box::new(condition),
		positive,
		negative,
	}.with_source(source));
}

enum MultipleLiteralItem {
	Collection(Expression),
	Record {
		key: WithSource<Expression>,
		value: WithSource<Expression>,
	}
}

impl MultipleLiteralItem {
	pub fn is_collection(&self) -> bool {
		match self {
			Self::Collection(_) => true,
			_ => false,
		}
	}

	pub fn is_record(&self) -> bool {
		match self {
			Self::Record { .. } => true,
			_ => false,
		}
	}
}


impl Parse for MultipleLiteralItem {
    type Output = Self;

    fn parse(parser: &mut crate::Parser, ctx: &Context) -> Try<WithSource<Self::Output>, WithSource<ParseError>> {
        let first = unwrap!(Expression::parse(parser, ctx));

		if parser.consume_if_equal(Token::Punctuation(":".to_string())).is_some() {
			let value = require!(Expression::parse(parser, ctx));

			let source = first.source().until(value.source().clone());

			return Try::Some(MultipleLiteralItem::Record {
				key: first,
				value,
			}.with_source(source));
		} else {
			return Try::Some(first.map(|first| MultipleLiteralItem::Collection(first)));
		}
    }
}