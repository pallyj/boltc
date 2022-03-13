mod typ;
mod file;

use std::{iter::Peekable};

use cstree::{GreenNodeBuilder, Language};

use crate::{lexer::{SyntaxKind, Lexer}, ast::BoltLanguage};

pub struct Parser<'a> {
	lexer: Peekable<Lexer<'a>>,
	builder: GreenNodeBuilder<'static, 'static>,
}

impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			lexer: Lexer::new(input).peekable(),
			builder: GreenNodeBuilder::new(),
		}
	}

	pub fn check(&mut self, token: SyntaxKind) -> bool {
		self.lexer.peek()
			.map(|next_token| next_token.kind == token)
			.unwrap_or(false)
	}

	pub fn eat(&mut self, token: SyntaxKind) -> bool {
		if !self.check(token) {
			return false;
		}

		self.bump();
		true
	}

	pub fn eat_and_start_node(&mut self, token: SyntaxKind, node: SyntaxKind) -> bool {
		if !self.check(token) {
			return false;
		}

		self.start_node(node);

		self.bump();
		true
	}

	pub fn bump(&mut self) {
		if let Some(next) = self.lexer.next() {
			self.builder.token(BoltLanguage::kind_to_raw(next.kind), next.source)
		}
	}

	fn parse_delim_separated(
		&mut self,
		node: SyntaxKind,
		bra: SyntaxKind,
		ket: SyntaxKind,
		sep: SyntaxKind,
		mut f: impl FnMut(&mut Self))
	{
		self.start_node(node);
		if !self.eat(bra) {
			// Throw an error
			// Recover from this
			return
		}

		while !self.eat(ket) {
			f(self);

			if !self.eat(sep) {
				// End of list
				if !self.eat(ket) {
					// Recover from missing separator
				}
				break
			}
		}
		self.finish_node();
	}

	pub fn parse_paren_comma_seq(
		&mut self,
		f: impl FnMut(&mut Self))
	{
		self.parse_delim_separated(
			SyntaxKind::CommaSeparatedList,
			SyntaxKind::OpenParen, 
			SyntaxKind::CloseParen, 
			SyntaxKind::Comma,
			f)
	}

	fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(BoltLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
}