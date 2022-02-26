use std::{sync::Arc, collections::HashSet};
use prelude::*;

use crate::{token::Token, LexError};

pub fn is_ident_head(c: char) -> bool {
	(c >= 'a' && c <= 'z') ||
	(c >= 'A' && c <= 'Z') ||
	c == '_' ||
	c == '$'
}

pub fn is_ident_body(c: char) -> bool {
	(c >= 'a' && c <= 'z') ||
	(c >= 'A' && c <= 'Z') ||
	(c >= '0' && c <= '9') ||
	c == '_' ||
	c == '$'
}

pub (crate) const SYMBOLS: [char; 11] = [
	'[', ']',
	'(', ')',
	'{', '}',
	',',
	';', ':',
	'`', '@'
];

pub (crate) const KEYWORDS: [&str; 41] = [
	"struct",
	"class",
	"enum",
	"case",
	"union",
	"protocol",
	"extension",
	"import",
	"alias",

	"func",
	"init",
	"let",
	"var",

	"static",

	"public",
	"private",
	"internal",
	"fileprivate",

	"if",
	"else",
	"guard",
	"match",

	"try",
	"catch",
	"throws",

	"async",
	"await",

	"throw",
	"return",
	"defer",

	"for",
	"in",

	"while",
	"repeat",
	
	"as",

	"while",
	"repeat",

	"break",
	"continue",

	"get",
	"set"
];

pub struct Lexer<'a> {
	file: Arc<SourceFile>,

	iter: CharIter<'a>,

	whitespaces: HashSet<usize>,

	tokens: Vec<WithSource<Token>>
}

impl<'a> Lexer<'a> {
	pub fn new(iter: CharIter<'a>) -> Self {
		Self {
			file: iter.file().clone(),
			iter: iter,
			whitespaces: HashSet::new(),
			tokens: vec![],
		}
	}

	pub fn is_at_end(&self) -> bool {
		self.iter.peek().is_none()
	}

	pub fn lex(&mut self) {
		while self.iter.peek().is_some() {
			let start = self.iter.index();
			if let Some(token) = self.lex_next() {
				let source = self.iter.since_last(start);
				self.tokens.push(token.with_source(source));
			}
		}
	}

	pub fn lex_next(&mut self) -> Option<Token> {
		let c = self.iter.next()?;

		if is_ident_head(c) {
			return self.lex_ident(c)
		} else if c.is_whitespace() {
			self.whitespaces.insert(self.iter.index());
			return None
		} else if c.is_decimal_digit() {
			return Some(self.lex_number(c))
		} else if c == '/' {
			return match self.iter.peek() {
				Some('/') => { self.iter.advance(); self.iter.advance_while(|c| !c.is_newline()); None },
				Some('*') => { self.iter.advance(); self.lex_long_comment() },
				_ => { Some(Token::Punctuation("/".to_string())) }
			}
		} else if SYMBOLS.contains(&c) {
			return Some(Token::Punctuation(c.to_string()))
		} else if is_op_char(c) {
			let sym = format!("{}{}", c, self.iter.advance_collect(is_op_char));

			return Some(Token::Punctuation(sym));
		}

		return None
	}

	fn lex_ident(&mut self, head: char) -> Option<Token> {
		let ident = format!("{}{}", head, self.iter.advance_collect(is_ident_body));
	
		if KEYWORDS.contains(&ident.as_str()) {
			Some(Token::Keyword(ident))
		} else {
			Some(Token::Ident(ident))
		}
	}


	pub (crate) fn lex_long_comment(&mut self) -> Option<Token> {
		let mut is_star = false;
		let mut comment = String::new();

		while let Some(c) = self.iter.next() {
			if c == '*' {
				is_star = true;
				continue;
			}
			if is_star && c == '/' {
				return None
			}
			if is_star {
				comment.push('*');
				is_star = false;
			}

			comment.push(c);
		}

		// error

		None
	}

	pub (crate) fn lex_number(&mut self, first_digit: char) -> Token {
		let mut acc = first_digit.to_digit(10).unwrap() as u64;
	
		self.iter.advance_while(|c| {
			if let Some(digit) = c.to_digit(10) {
				acc *= 10;
				acc += u64::from(digit);
				true
			} else {
				false
			}
		});
	
		Token::IntLit(acc)
	}
}

impl<'a> GenericLexer for Lexer<'a> {
    type Token = Token;
    type Error = LexError;

    fn into(self) -> (Arc<SourceFile>, Vec<WithSource<Self::Token>>, HashSet<usize>) {
        (self.file, self.tokens, self.whitespaces)
    }

    fn errors(&self) -> Option<Vec<Self::Error>> {
        todo!()
    }
}

fn is_op_char(c: char) -> bool {
	c == '+' ||
	c == '-' ||
	c == '*' ||
	c == '/' ||
	c == '%' ||
	c == '<' ||
	c == '>' ||
	c == '&' ||
	c == '|' ||
	c == '^' ||
	c == '=' ||
	c == '!' ||
	c == '?' ||
	c == '.'
}