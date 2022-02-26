// TODO: Move this into the prelude

// Errors v2:
// MessageList
// CorrectnessCtx
//
//

use std::sync::Mutex;

use crate::{BoltMessage, Source, WithSource};


pub struct ErrorCtx {
	inner: Mutex<ErrorCtxInner>,
}

impl ErrorCtx {
	/// Creates a new error context
	pub fn new() -> ErrorCtx {
		ErrorCtx {
			inner: Mutex::new(ErrorCtxInner::new()),
		}
	}

	/// Raises an error with a source
	pub fn raise_error<T: BoltMessage + 'static>(&self, error: WithSource<T>) {
		let (error, source) = error.unwrap();

		let boxed_error = Box::new(error);

		self.inner
			.lock()
			.unwrap()
			.raise(boxed_error, Some(source), true);
	}

	/// Raises an error anonymously
	pub fn raise_error_anon<T: BoltMessage + 'static>(&self, error: T) {
		let boxed_error = Box::new(error);

		self.inner
			.lock()
			.unwrap()
			.raise(boxed_error, None, true);
	} 

	/// Raises a warning at a source
	pub fn raise_warning<T: BoltMessage + 'static>(&self, warning: WithSource<T>) {
		let (warning, source) = warning.unwrap();

		let boxed_warning = Box::new(warning);

		self.inner
			.lock()
			.unwrap()
			.raise(boxed_warning, Some(source), false);
	}

	/// Raises a warning anonymously
	pub fn raise_warning_anon<T: BoltMessage + 'static>(&self, warning: T) {
		let boxed_warning = Box::new(warning);

		self.inner
			.lock()
			.unwrap()
			.raise(boxed_warning, None, false);
	}

	/// TODO: Change to print errors
	pub fn messages(&self) -> &Vec<(Box<dyn BoltMessage>, Option<Source>)> {
		todo!()
	}

	pub fn has_errors(&self) -> bool {
		self.inner
			.lock()
			.unwrap().error_count > 0
	}
}



struct ErrorCtxInner {
	messages: Vec<(Box<dyn BoltMessage>, Option<Source>)>,
	error_count: u64,
	warning_count: u64,
}

impl ErrorCtxInner {
	pub fn new() -> Self {
		Self {
			messages: vec![],
			error_count: 0,
			warning_count: 0,
		}
	}

	pub (self) fn raise(&mut self, err: Box<dyn BoltMessage>, source: Option<Source>, is_error: bool) {
		self.messages.push((err, source));

		if is_error {
			self.error_count += 1;
		} else {
			self.warning_count += 1;
		}
	}

	fn errors(&self) -> &Vec<(Box<dyn BoltMessage>, Option<Source>)> {
		&self.messages
	}
}