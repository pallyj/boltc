mod typ;
mod file;
mod expr;
mod smt;
mod func;
mod var;
mod struct_;
mod event;
mod sink;
mod marker;

use errors::{debugger::Debugger};

use crate::{lexer::{SyntaxKind, Lexer, Token}, operators::OperatorFactory, ast::{Parse, SyntaxNode}};

use self::{event::Event, sink::Sink, marker::{Marker, CompletedMarker}};

struct Parser<'input, 'l> {
	lexemes: &'l [Token<'input>],
	//operators: OperatorFactory,
	events: Vec<Event<'input>>,
	cursor: usize
}

impl<'input, 'l> Parser<'input, 'l> {
	pub fn new(lexemes: &'l [Token<'input>]) -> Self {
		Self {
			lexemes,
			events: Vec::new(),
			//operators: OperatorFactory::new(),
			cursor: 0,
		}
	}

	/*pub fn operator_factory(&mut self) -> &mut OperatorFactory {
		&mut self.operators
	}*/

	pub fn check(&mut self, token: SyntaxKind) -> bool {
		self.peek()
			.map(|next_token| next_token == token)
			.unwrap_or(false)
	}

	pub fn eat(&mut self, token: SyntaxKind) -> bool {
		if !self.check(token) {
			return false;
		}

		self.bump();
		true
	}

	pub fn bump(&mut self) {
		self.eat_trivia();
		if let Some(next) = self.lexemes.get(self.cursor) {
			self.events.push(Event::AddToken { kind: next.kind, text: next.source });
			self.cursor += 1;
		}
	}

	pub fn slice(&mut self) -> &str {
		self.lexemes
			.get(self.cursor)
			.unwrap().source
	}

	fn parse_delim(
		&mut self,
		node: SyntaxKind,
		bra: SyntaxKind,
		ket: SyntaxKind,
		mut f: impl FnMut(&mut Self))
	{
		let marker = self.start();
		if !self.eat(bra) {
			// Throw an error
			// Recover from this
			self.bump();
		} else {
			while !self.eat(ket) {
				f(self);
			}
		}
		marker.complete(self, node);
	}

	fn parse_delim_separated(
		&mut self,
		node: SyntaxKind,
		bra: SyntaxKind,
		ket: SyntaxKind,
		sep: SyntaxKind,
		mut f: impl FnMut(&mut Self))
	{
		let marker = self.start();

		if !self.eat(bra) {
			// Throw an error
			// Recover from this
			self.bump();
		} else {
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
		}

		marker.complete(self, node);
	}

	pub fn error(&mut self, error: &str) {
		let event = Event::Error(error.to_string());

		self.events.push(event);

		// Do error recovery
	}

	pub fn error_recover(&mut self, error: &str, recovery_set: &[SyntaxKind]) -> CompletedMarker {
		self.error(error);
		let err = self.start();
		if !self.peek().map(|peeked_token| recovery_set.contains(&peeked_token)).unwrap_or(false) {
			self.bump();
		}
		err.complete(self, SyntaxKind::Error)
	}

	pub fn start(&mut self) -> Marker {
		let pos = self.events.len();
		self.events.push(Event::Placeholder);

		Marker::new(pos)
	}

	pub fn node<F: FnOnce(&mut Self)>(&mut self, kind: SyntaxKind, f: F) {
		let node = self.start();

		f(self);

		node.complete(self, kind);
	}

	pub fn name(&mut self, recovery_set: &[SyntaxKind]) {
		if self.check(SyntaxKind::Ident) {
			let func_name = self.start();
			self.eat(SyntaxKind::Ident);
			func_name.complete(self, SyntaxKind::FuncName);
		} else {
			self.error_recover("expected name", recovery_set);
		}
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

	fn eat_trivia(&mut self) {
        while self.peek_raw().map(|peek| peek.is_trivia()).unwrap_or(false) {
            self.cursor += 1;
        }
    }

	fn peek(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_raw()
    }

	fn peek_raw(&self) -> Option<SyntaxKind> {
        self.lexemes
            .get(self.cursor)
            .map(|token| token.kind)
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

	pub fn parse_visibility(&mut self) {
		let marker = self.start();

		if self.eat(SyntaxKind::PublicKw) ||
		   self.eat(SyntaxKind::InternalKw) ||
		   self.eat(SyntaxKind::FilePrivateKw) ||
		   self.eat(SyntaxKind::PrivateKw) { }
		else { }

		marker.complete(self, SyntaxKind::Visibility);
	}
}

pub fn parse<'input>(input: &'input str, debugger: &'input mut Debugger, file: usize) -> Parse {
	let lexemes: Vec<_> = Lexer::new(input).collect();

	let parser = Parser::new(&lexemes);

	let events = parser.parse_file();
	let sink = Sink::new(events, &lexemes, file);

	Parse {
		root: SyntaxNode::new_root(sink.finish(debugger))
	}
}