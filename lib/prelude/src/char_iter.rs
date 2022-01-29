use std::sync::Arc;

use crate::{WithSource, Source, HasSource, SourceFile};

pub struct CharIter<'a> {
	/// The file this iter operates on
	pub (crate) file: Arc<SourceFile>,

	/// The char buffer
	pub (crate) buffer: &'a Vec<char>,

	/// The index to read on
	pub (crate) idx: usize,
}

impl<'a> CharIter<'a> {
	pub fn file(&self) -> &Arc<SourceFile> {
		&self.file
	}

	pub fn index(&self) -> usize {
		self.idx
	}

	/// Gets the next character in the stream, advancing it by 1
	/// 
	/// If the iter is at its end, returns None
	pub fn next(&mut self) -> Option<char> {
		if self.idx < self.buffer.len() {
			let val = Some(self.buffer[self.idx]);
			self.idx += 1;
			val
		} else {
			None
		}
	}

	/// Returns the next character in the iter
	/// 
	/// If the iter is at the end of the file,
	/// returns None
	/// 
	pub fn peek(&self) -> Option<char> {
		if self.idx < self.buffer.len() {
			Some(self.buffer[self.idx])
		} else {
			None
		}
	}

	/// Advances the iter by 1
	pub fn advance(&mut self) {
		self.idx += 1;
	}

	/// Advances the iter by `n`
	pub fn advance_by(&mut self, n: usize) {
		self.idx += n;
	}

	/// Checks if the next character matches a predicate,
	/// and advances the iter by 1 if it does. Returns whether
	/// the next character matched it.
	pub fn advance_if<F>(&mut self, mut f: F) -> Option<char>
		where F: FnMut(char) -> bool
	{
		self.peek()
			.and_then(|c| {
				if f(c) { self.idx += 1; Some(c) }
				else { None }
			})
	}

	/// Advance the iter while the next character matches a predicate
	pub fn advance_while<F>(&mut self, mut f: F)
		where F: FnMut(char) -> bool
	{
		while let Some(next) = self.peek() {
			if f(next) {
				self.idx += 1;
			} else {
				break;
			}
		}
	}

	/// Advance the iter while the next character matches a predicate,
	/// collecting the characters into a string
	pub fn advance_collect<F>(&mut self, mut f: F) -> String
		where F: FnMut(char) -> bool
	{
		let mut string = String::new();

		while let Some(next) = self.peek() {
			if f(next) {
				string.push(next);
				self.idx += 1;
			} else {
				break;
			}
		}

		string
	}

	/// Runs a function in this context, and returns
	/// the string that was parsed.
	pub fn slice<F>(&mut self, mut f: F) -> WithSource<String>
		where F: FnMut(&mut Self)
	{
		let start_idx = self.idx;
		f(self);
		let end_idx = self.idx;

		let source = Source {
			file: self.file.clone(),
			char_index: start_idx,
			len: end_idx - start_idx,
		};

		source.slice()
			  .with_source(source)
	}

	/// Runs a function in this context, and returns
	/// the string that was parsed.
	pub fn slice_map<F, T: HasSource>(&mut self, mut f: F) -> WithSource<T>
		where F: FnMut(&mut Self) -> T
	{
		let start_idx = self.idx;
		let lexed = f(self);
		let end_idx = self.idx;

		let source = Source {
			file: self.file.clone(),
			char_index: start_idx,
			len: end_idx - start_idx,
		};

		lexed.with_source(source)
	}

	pub fn since_last(&self, start_idx: usize) -> Source {
		let end_idx = self.idx;

		Source {
			file: self.file.clone(),
			char_index: start_idx,
			len: end_idx - start_idx,
		}
	}
}