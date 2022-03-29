use crate::{error::{Error, ErrorCode}, Span};
use colored::Colorize;

pub struct Debugger {
	errors: Vec<Error>,
	
	
}

impl Debugger {
	pub fn new() -> Debugger {
		Debugger { errors: vec![] }
	}

	pub fn throw(&mut self, code: ErrorCode, spans: Vec<Span>) {
		println!("{}", code.error_code().red());

		self.errors.push(Error::new(code, spans));
	}

	pub fn has_errors(&self) -> bool {
		self.errors.len() > 0
	}
}