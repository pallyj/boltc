mod typ;
mod file;
mod expr;
mod smt;
mod func;
mod var;
mod struct_;

use std::{iter::Peekable};

use rowan::{GreenNodeBuilder, Language, Checkpoint};

use crate::{lexer::{SyntaxKind, Lexer}, ast::BoltLanguage, operators::OperatorFactory};

pub struct Parser<'a> {
	lexer: Peekable<Lexer<'a>>,
	operators: OperatorFactory,
	builder: GreenNodeBuilder<'static>,
	index: usize
}

impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Self {
		Self {
			lexer: Lexer::new(input).peekable(),
			builder: GreenNodeBuilder::new(),
			operators: OperatorFactory::new(),
			index: 0,
		}
	}

	pub fn operator_factory(&mut self) -> &mut OperatorFactory {
		&mut self.operators
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

	pub fn eat_and_start_node_at(&mut self, token: SyntaxKind, node: SyntaxKind, checkpoint: Checkpoint) -> bool {
		if !self.check(token) {
			return false;
		}

		self.start_node_at(node, checkpoint);

		self.bump();
		true
	}

	pub fn bump(&mut self) {
		if let Some(next) = self.lexer.next() {
			self.builder.token(BoltLanguage::kind_to_raw(next.kind), next.source);
			self.index += 1;
		}
	}

	pub fn slice(&mut self) -> &str {
		self.lexer.peek().unwrap().source
	}

	fn parse_delim(
		&mut self,
		node: SyntaxKind,
		bra: SyntaxKind,
		ket: SyntaxKind,
		mut f: impl FnMut(&mut Self))
	{
		self.start_node(node);
		if !self.eat(bra) {
			// Throw an error
			// Recover from this
			self.bump();
			return
		}

		while !self.eat(ket) {
			f(self);
		}
		self.finish_node();
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
			self.bump();
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

	/*
	fn parse_delim_separated_at(
		&mut self,
		node: SyntaxKind,
		bra: SyntaxKind,
		ket: SyntaxKind,
		sep: SyntaxKind,
		chk: Checkpoint,
		mut f: impl FnMut(&mut Self))
	{
		self.start_node_at(node, chk);
		if !self.eat(bra) {
			// Throw an error
			// Recover from this
			self.bump();
			self.finish_node();
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
	}*/

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

	fn start_node_at(&mut self, kind: SyntaxKind, checkpoint: Checkpoint) {
        self.builder.start_node_at(checkpoint, BoltLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

	pub fn parse_visibility(&mut self) {
		self.start_node(SyntaxKind::Visibility);

		if self.eat(SyntaxKind::PublicKw) ||
		   self.eat(SyntaxKind::InternalKw) ||
		   self.eat(SyntaxKind::FilePrivateKw) ||
		   self.eat(SyntaxKind::PrivateKw) { }
		else { }

		self.finish_node();
	}
}