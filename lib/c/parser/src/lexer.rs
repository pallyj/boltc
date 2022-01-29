use prelude::*;

use std::{sync::Arc, collections::HashSet};
use crate::{token::Token, LexError};

pub struct Lexer<'a> {
	file: Arc<SourceFile>,

	iter: CharIter<'a>,

	/*errors: Vec<LexerMessage>,*/

	whitespaces: HashSet<usize>,

	tokens: Vec<WithSource<Token>>,
}

impl<'a> Lexer<'a> {
	pub fn new(iter: CharIter<'a>) -> Self {
		Self {
			file: iter.file().clone(),
			iter: iter,

			whitespaces: HashSet::new(),
			tokens: vec![],
			//preprocessor: Preprocessor::new(),
		}
	}

	pub fn is_not_at_end(&self) -> bool {
		self.iter.peek().is_some()
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

	pub fn tokens(&self) -> &Vec<WithSource<Token>> {
		&self.tokens
	}

	fn lex_next(&mut self) -> Option<Token> {
		let c = self.iter.next()?;
	
		if is_ident_head(c) {
			self.lex_ident( c)
			// Definitions
		} else if c == '0' {
			Some(lex_non_decimal(&mut self.iter))
		} else if c.is_decimal_digit() {
			Some(lex_number(&mut self.iter, c))
		} else if c == '"' {
			Some(lex_string(&mut self.iter))
		} else if c == '/' {
			match self.iter.peek() {
				Some('/') => { self.iter.advance(); self.iter.advance_while(|c| !c.is_newline()); None },
				Some('*') => { self.iter.advance(); lex_long_comment(&mut self.iter) },
				_ => { Some(Token::Operator("/".to_string())) }
			}
		} else if c.is_whitespace() {
			self.whitespaces.insert(self.iter.index());
			None
		} else if SYMBOLS.contains(&c) {
			Some(Token::Symbol(c.to_string()))
		} else if OP_CHARS.contains(&c) {
			Some(lex_op(&mut self.iter, c))
		} else if c == '#' {
			self.lex_macro()
		}
		else {
			println!("None");
			None
		}
	}

	fn lex_ident(&mut self, head: char) -> Option<Token> {
		let ident = format!("{}{}", head, self.iter.advance_collect(is_ident_body));
	
		if KEYWORDS.contains(&ident.as_str()) {
			Some(Token::Keyword(ident))
		} /*else if self.preprocessor.has_macro(ident.as_str()) {
			self.tokens.extend(self.preprocessor.expand_define(ident.as_str(), &mut self.iter));

			None
		}*/ else {
			Some(Token::Ident(ident))
		}
	}

	fn lex_macro(&mut self) -> Option<Token> {
		let macro_name = self.iter.advance_collect(is_ident_body);

		Some(Token::Directive(macro_name))
	}

	
	pub fn iter_mut(&mut self) -> &mut CharIter<'a> {
		&mut self.iter
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

pub fn is_ident_head(c: char) -> bool {
	(c >= 'a' && c <= 'z') ||
	(c >= 'A' && c <= 'Z') ||
	c == '_'
}

pub fn is_ident_body(c: char) -> bool {
	(c >= 'a' && c <= 'z') ||
	(c >= 'A' && c <= 'Z') ||
	(c >= '0' && c <= '9') ||
	c == '_'
}

pub (crate) const SYMBOLS: [char; 11] = [
	'[', ']',
	'(', ')',
	'{', '}',
	'.', ',',
	';', ':',
	'\\'
];

/*const OPERATORS: [&'static str; 35] = [
	"+", "-", "*", "/",
	"%", "++", "--",
	"==", "!=",
	">", "<",
	">=", "<=",
	"&&", "||", "!",
	"&", "|", "^", "~", "<<", ">>",
	"=",
	"+=", "-=", "*=", "/=", "%=",
	"<<=", ">>=", "&=", "^=", "|=",
	"?", "->"
];*/

pub (crate) const OP_CHARS: [char; 16] = [
	'+', '-', '*', '/',
	'%',
	'=', '!',
	'>', '<',
	'&', '|', '!',
	'^', '~',
	'=',
	'?'
];

pub (crate) const KEYWORDS: [&'static str; 32] = [
	"auto",
	"double",
	"int",
	"struct",
	"break",
	"else",
	"long",
	"switch",
	"case",
	"enum",
	"register",
	"typedef",
	"char",
	"extern",
	"return",
	"union",
	"const",
	"short",
	"float",
	"unsigned",
	"continue",
	"for",
	"signed",
	"void",
	"default",
	"goto",
	"sizeof",
	"volatile",
	"do",
	"if",
	"static",
	"while",
];

pub (crate) fn lex_string(iter: &mut CharIter<'_>) -> Token {
	// To make strings end at the end of the line
	// there needs to be some way to emit errors
	let string = iter.advance_collect(|c| !(c == '"' || c == '\n'));

	match iter.next() {
		Some('\n') => {} // Error
		Some('"') => {},
		None => {} // Reached EOF
		_ => {} // Not possible
	}

	Token::StringLit(string)
}

pub (crate) fn lex_long_comment(iter: &mut CharIter<'_>) -> Option<Token> {
	let mut is_star = false;
	let mut comment = String::new();

	while let Some(c) = iter.next() {
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

pub (crate) fn lex_number(iter: &mut CharIter<'_>, first_digit: char) -> Token {
	let mut acc = first_digit.to_digit(10).unwrap() as u64;

	iter
		.advance_while(|c| {
			if let Some(digit) = c.to_digit(10) {
				acc *= 10;
				acc += u64::from(digit);
				true
			} else {
				false
			}
		});

	Token::NumberLit(acc)
}

pub (crate) fn lex_non_decimal(iter: &mut CharIter<'_>) -> Token {
	if let Some(c) = iter.peek() {
		if c == 'x' {
			iter.next();
			lex_hex(iter)
		} else if c.is_octal_digit() {
			lex_oct(iter)
		} else {
			Token::NumberLit(0)
		}
	} else {
		Token::NumberLit(0)
	}
}

pub (crate) fn lex_hex(iter: &mut CharIter<'_>) -> Token {
	let mut acc = 0;

	iter
		.advance_while(|c| {
			if let Some(digit) = c.to_digit(16) {
				acc *= 16;
				acc += u64::from(digit);
				true
			} else {
				false
			}
		});

	Token::NumberLit(acc)
}

pub (crate) fn lex_oct(iter: &mut CharIter<'_>) -> Token {
	let mut acc = 0;

	iter
		.advance_while(|c| {
			if let Some(digit) = c.to_digit(8) {
				acc *= 8;
				acc += u64::from(digit);
				true
			} else {
				false
			}
		});

	Token::NumberLit(acc)
}

pub (crate) fn lex_op(iter: &mut CharIter<'_>, first_digit: char) -> Token {
	let mut acc = String::new();

	acc.push(first_digit);

	iter
		.advance_while(|c| {
			if OP_CHARS.contains(&c) {
				acc.push(c);
				true
			} else {
				false
			}
		});

	

	Token::Operator(acc)
}
