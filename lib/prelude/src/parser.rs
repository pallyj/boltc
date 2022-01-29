use std::{sync::Arc, collections::HashSet};

use crate::{BoltMessage, WithSource, SourceFile, Source, HasSource, GenericLexer, GenericToken};

pub struct Parser<T: GenericToken, E: BoltMessage> {
	/// The file we are parsing on
	file: Arc<SourceFile>,

	/// Indexes with a whitespace
	whitespaces: HashSet<usize>,

	/// The tokens produced by the lexer
	tokens: Vec<WithSource<T>>,

	/// The index to read at
	idx: usize,

	/// Messages outputted by the parser
	messages: Vec<E>,
}

impl<T: GenericToken, E: BoltMessage> Parser<T, E> {
	/// Creates a `Parser` from the tokens
	pub fn new<U: GenericLexer<Token = T>>(lexer: U) -> Self {
		let (file, tokens, whitespaces) = lexer.into();

		Self {
			file,
			whitespaces,
			tokens,
			idx: 0,
			messages: vec![],
		}
	}

	/// Emits an error in the `Parser`
	pub fn emit_error(&mut self, e: E) {
		self.messages.push(e);
	}

	/// Peeks at the next token
	pub fn peek(&self) -> Option<&WithSource<T>> {
		self.tokens.get(self.idx)
	}

	/// Requires a whitespace at the current location
	pub fn require_whitespace(&self) -> bool {
		self.whitespaces.contains(&self.idx)
	}

	/// Requires a newline at the current location
	pub fn require_newline(&self) -> bool {
		self.file.line_breaks.contains(&self.idx)
	}

	/// Requires no whitespace at the current location
	pub fn guard_whitespace(&self) -> bool {
		!self.whitespaces.contains(&self.idx)
	}

	/// Requires no newline at the current location
	pub fn guard_newline(&self) -> bool {
		!self.file.line_breaks.contains(&self.idx)
	}

	/// Consume the next token
	pub fn consume(&mut self) -> Option<&WithSource<T>> {
        if let Some(t) = self.tokens.get(self.idx) {
			self.idx += 1;
			return Some(t)
        }
		return None;
    }

	/// Consume the next token if it equals a token
	pub fn consume_if_equal(&mut self, equals: T) -> bool {
        if let Some(t) = self.tokens.get(self.idx) {
            if t.value() == &equals {
                self.idx += 1;
                return true;
            }
        }
		return false;
    }
    
	/// Consume the next token if it matches a predicate
    pub fn consume_if<F>(&mut self, condition: F) -> Option<&WithSource<T>>
		where F: FnOnce(&T) -> bool
	{   
        let t = self.tokens.get(self.idx)?;
        
        if condition(t.value()) {
            self.idx += 1;
            Some(t)
        } else {
            None
        }
    }

	pub fn consume_if_map<F, U>(&mut self, condition: F) -> Option<U>
		where F: FnOnce(&WithSource<T>) -> Option<U>
	{   
        let t = self.tokens.get(self.idx)?;


        
        if let Some(v) = condition(t) {
            self.idx += 1;
            Some(v)
        } else {
            None
        }
    }

	/// Runs a function in this context, and returns
	/// a value that was parsed.
	pub fn slice_map<F, U: HasSource>(&mut self, f: F) -> WithSource<U>
		where F: FnOnce(&mut Self) -> U
	{
		let start_idx = self.idx;
		let parsed = f(self);
		let end_idx = self.idx;

		let source = Source {
			file: self.file.clone(),
			char_index: start_idx,
			len: end_idx - start_idx,
		};

		parsed.with_source(source)
	}

	/// Runs a function in the context, and returns
	/// a value if it passed. If it doesn't pass, rewind the parser
	pub fn slice_if<F, U>(&mut self, f: F) -> Result<WithSource<U>, WithSource<E>>
		where F: FnOnce(&mut Self) -> Result<U, E>
	{
		let start_idx = self.idx;
		let parsed = f(self);
		let end_idx = self.idx;

		let source = Source {
			file: self.file.clone(),
			char_index: start_idx,
			len: end_idx - start_idx,
		};

		match parsed {
			Ok(ok) => Ok(ok.with_source(source)),
			Err(err) => Err(err.with_source(source)),
		}
	}

	pub fn last_source(&self) -> Source {
		self.tokens[self.idx - 1].source().clone()
	}

	pub fn next_source(&self) -> Option<Source> {
		self.tokens.get(self.idx)
			.map(|t| t.source().clone())
	}
}