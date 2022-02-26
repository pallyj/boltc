use std::{sync::Arc, collections::HashSet};
use prelude::*;

use super::{token::Token, error::LexError};

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

pub struct Lexer<'a> {
	file: Arc<SourceFile>,

	iter: CharIter<'a>,

	whitespaces: HashSet<usize>,

	pub tokens: Vec<WithSource<Token>>
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
		} else if c == '(' {
			return Some(Token::OpenParen)
		} else if c == ')' {
			return Some(Token::CloseParen)
		} else if c == ',' {
			return Some(Token::Comma)
		}

		return None
	}

	fn lex_ident(&mut self, head: char) -> Option<Token> {
		let ident = format!("{}{}", head, self.iter.advance_collect(is_ident_body));
	
		Some(Token::Ident(ident))
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