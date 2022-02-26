use std::{sync::Arc, collections::HashSet};

use crate::{BoltMessage, WithSource, SourceFile, Source, HasSource, GenericLexer, GenericToken, Try};

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
	messages: Vec<WithSource<E>>,

	pub eof: WithSource<T>,
}

impl<T: GenericToken, E: BoltMessage> Parser<T, E> {
	/// Creates a `Parser` from the tokens
	pub fn new<U: GenericLexer<Token = T>>(lexer: U) -> Self {
		let (file, tokens, whitespaces) = lexer.into();

		let last_span = tokens.last().map(|t| t.source().clone()).unwrap_or(Source {
			file: file.clone(),
			char_index: 0,
			len: 0,
		});

		Self {
			file,
			whitespaces,
			tokens,
			idx: 0,
			messages: vec![],
			eof: T::eof().with_source(last_span),
		}
	}

	/// Emits an error in the `Parser`
	pub fn emit_error(&mut self, e: WithSource<E>) {
		self.messages.push(e);
	}

	/// Peeks at the next token
	pub fn peek(&self) -> &WithSource<T> {
		if let Some(t) = self.tokens.get(self.idx) {
			t
		} else {
			&self.eof
		}
	}

	pub fn peek_ahead(&self, offset: usize) -> &WithSource<T> {
		if let Some(t) = self.tokens.get(self.idx + offset) {
			t
		} else {
			&self.eof
		}
	}

	pub fn skip(&mut self, n: usize) {
		self.idx += n;
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
	pub fn consume_if_equal(&mut self, equals: T) -> Option<Source> {
        if let Some(t) = self.tokens.get(self.idx) {
            if t.value() == &equals {
                self.idx += 1;
                return Some(t.source().clone());
            }
        }
		return None;
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
	pub fn slice_map<F, U: HasSource>(&mut self, f: F) -> Try<WithSource<U>, WithSource<E>>
		where F: FnOnce(&mut Self) -> Try<U, WithSource<E>>
	{
		let start_idx = self.idx;
		let parsed = f(self);
		let end_idx = self.idx;

		let start = self.tokens.get(start_idx).map(|t| t.source().clone()).unwrap_or_else(|| self.eof.source().clone() );
		let end = self.tokens.get(end_idx).map(|t| t.source().clone()).unwrap_or_else(|| self.eof.source().clone() );

		match parsed {
			Try::Some(s) => Try::Some(s.with_source(start.until(end))),
			Try::None(e) => Try::None(e),
			Try::Err(e) => Try::Err(e),
		}
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
			char_index: self.tokens[start_idx].source().char_index,
			len: self.tokens[end_idx].source().char_index.checked_sub(self.tokens[start_idx].source().char_index).unwrap_or(0),
		};

		match parsed {
			Ok(ok) => Ok(ok.with_source(source)),
			Err(err) => Err(err.with_source(source)),
		}
	}

	pub fn last_source(&self) -> Source {
		if self.idx == 0 {
			return self.tokens[0].source().clone()
		}
		self.tokens[self.idx - 1].source().clone()
	}

	pub fn next_source(&self) -> Source {
		if let Some(t) = self.tokens.get(self.idx) {
			t.source().clone()
		} else {
			self.eof.source().clone()
		}
	}

	/*pub fn get_slice_since(&self, start: usize) -> Option<Source> {
		if start < self.tokens.l

		let source = Source {
			file: self.file.clone(),
			char_index: start,
			len: self.idx - start,
		};
	}*/

	pub fn is_at_eof(&self) -> bool {
		self.idx >= self.tokens.len()
	}

	pub fn messages(&self) -> &Vec<WithSource<E>> {
		&self.messages
	}

	pub fn file(&self) -> Arc<SourceFile> {
		self.file.clone()
	}
}