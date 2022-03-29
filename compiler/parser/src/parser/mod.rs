mod typ;
mod file;
mod expr;
mod smt;
mod func;
mod var;
mod struct_;
mod event;
mod sink;

use errors::{debugger::Debugger, error::ErrorCode, Span};

use crate::{lexer::{SyntaxKind, Lexer, Token}, operators::OperatorFactory, ast::{Parse, SyntaxNode}};

use self::{event::Event, sink::Sink};

struct Parser<'input, 'l> {
	lexemes: &'l [Token<'input>],
	debugger: &'input mut Debugger,
	operators: OperatorFactory,
	events: Vec<Event<'input>>,
	cursor: usize
}

impl<'input, 'l> Parser<'input, 'l> {
	pub fn new(lexemes: &'l [Token<'input>], debugger: &'input mut Debugger) -> Self {
		Self {
			lexemes,
			debugger,
			events: Vec::new(),
			operators: OperatorFactory::new(),
			cursor: 0,
		}
	}

	pub fn operator_factory(&mut self) -> &mut OperatorFactory {
		&mut self.operators
	}

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

	pub fn eat_and_start_node(&mut self, token: SyntaxKind, node: SyntaxKind) -> bool {
		if !self.check(token) {
			return false;
		}

		self.start_node(node);

		self.bump();
		true
	}

	pub fn eat_and_start_node_at(&mut self, token: SyntaxKind, node: SyntaxKind, checkpoint: usize) -> bool {
		if !self.check(token) {
			return false;
		}

		self.start_node_at(node, checkpoint);

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

	pub fn checkpoint(&self) -> usize {
		self.events.len()
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

	pub fn error(&mut self, code: ErrorCode, spans: Vec<Span>) {
		self.bump();

		self.debugger.throw(code, spans);
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

	fn start_node(&mut self, kind: SyntaxKind) {
		self.events.push(Event::StartNode { kind });
    }

	fn start_node_at(&mut self, kind: SyntaxKind, checkpoint: usize) {
		self.events.push(Event::StartNodeAt { kind, at: checkpoint });
    }

    fn finish_node(&mut self) {
		self.events.push(Event::FinishNode);
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

pub fn parse<'input>(input: &'input str, debugger: &'input mut Debugger) -> Parse {
	let lexemes: Vec<_> = Lexer::new(input).collect();

	let parser = Parser::new(&lexemes, debugger);

	let events = parser.parse_file();
	let sink = Sink::new(events, &lexemes);

	Parse {
		root: SyntaxNode::new_root(sink.finish())
	}
}