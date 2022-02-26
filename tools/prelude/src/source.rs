use std::{sync::Arc, fmt::{Debug, Display}};
use crate::Try;

use super::SourceFile;

#[derive(Clone)]

/// Where something came from
pub struct Source {
	/// The file this source comes from
	pub (crate) file: Arc<SourceFile>,

	/// The character index this source starts at
	/// 
	/// Source files contain tables of the lines and columns,
	/// so these don't have to be stored separately
	pub (crate) char_index: usize,

	/// How many characters the source extends for
	pub (crate) len: usize,
}

impl Source {

	/// Creates a new source in between two others
	pub fn until(&self, other_source: Source) -> Source {
		assert!(Arc::ptr_eq(&self.file, &other_source.file), "Fatal error: Tried to combine spans in different files");

		let len = if self.char_index > other_source.char_index + other_source.len {
			0
		} else {
			(other_source.char_index + other_source.len) - self.char_index 
		};

		Source {
			file: self.file.clone(),
			char_index: self.char_index,
			len
		}
	}

	/// The name of the file the error's in
	pub fn file_name(&self) -> &str {
		&self.file.file_name
	}

	/// The line the error starts on
	pub fn line(&self) -> usize {
		self.file.line(self.char_index)
	}

	/// The column where the error starts
	pub fn col(&self) -> usize {
		self.file.col(self.char_index)
	}

	/// The code causing the error
	pub fn slice(&self) -> String {
		// No source can be made outside of the file
		self.file.slice(self.char_index, self.len)
			.unwrap()
	}

	/// The entire line causing the error
	pub fn line_slice(&self) -> String {
		// No source can be made outside of the file
		self.file.line_slice(self.char_index, self.len)
			.unwrap_or("".to_string())
	}

	pub fn index_of_line(&self) -> usize {
		self.file.index_of_line(self.char_index, self.len)
			.unwrap_or(0)
	}

	pub fn len(&self) -> usize {
		self.len
	}
}

pub struct WithSource<T> {
	value: T,
	source: Source
}

impl<T: Sized> WithSource<T> {
	/// Unwraps a WithSource, returning its value and source
	pub fn unwrap(self) -> (T, Source) {
		(self.value, self.source)
	}

	/// Allows for getting fields of the value without unwrapping
	pub fn value(&self) -> &T {
		&self.value
	}

	/// Allows for editing the value without unwrapping
	pub fn value_mut(&mut self) -> &mut T {
		&mut self.value
	}

	/// Retrieves the source of the span
	pub fn source(&self) -> &Source {
		&self.source
	}

	/// Creates a WithSource of a ref to the current WithSource
	pub fn as_ref<'a>(&'a self) -> WithSource<&'a T> {
		let source = self.source.clone();

		WithSource {
			value: &self.value,
			source,
		}
	}

	/// Maps a WithSource from one type to another
	pub fn map<F, U>(self, f: F) -> WithSource<U>
		where F: FnOnce(T) -> U
	{
		let source = self.source.clone();

		WithSource {
			value: f(self.value),
			source,
		}
	}
}

impl <T: core::fmt::Debug> Debug for WithSource<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)
    }
}

impl<T> Display for WithSource<T> where T: core::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)
    }
}

impl<T: HasSource> WithSource<Option<T>> {
	/// Moves the WithSource inside the Option
	pub fn take(self) -> Option<WithSource<T>> {
		let (value, source) = self.unwrap();

		value.map(|value| {
			value.with_source(source)
		})
	}
}

impl<S: HasSource, E: HasSource> WithSource<Result<S, E>> {
	/// Moves the WithSource inside the Result
	pub fn take(self) -> Result<WithSource<S>, WithSource<E>> {
		let (value, source) = self.unwrap();

		match value {
			Ok(success) => Ok(success.with_source(source)),
			Err(err) => Err(err.with_source(source))
		}
	}
}

impl Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file_name(), self.line(), self.col())
    }
}

impl<S: HasSource, E: HasSource> WithSource<Try<S, E>> {
	/// Moves the WithSource inside the Result
	pub fn take(self) -> Try<WithSource<S>, WithSource<E>> {
		let (value, source) = self.unwrap();

		match value {
			Try::Some(success) => Try::Some(success.with_source(source)),
			Try::None(err) => Try::None(err.with_source(source)),
			Try::Err(err) => Try::None(err.with_source(source)),
		}
	}
}

impl<S: Clone> Clone for WithSource<S> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), source: self.source.clone() }
    }
}

pub trait HasSource: Sized {
	fn with_source(self, source: Source) -> WithSource<Self> {
		WithSource {
			value: self,
			source
		}
	}
}

impl<T> HasSource for T {}