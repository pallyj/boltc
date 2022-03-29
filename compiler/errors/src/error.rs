use crate::Span;

#[derive(error_derive::Error)]
pub enum ErrorCode {
	ExpectedIdent,
}

pub struct Error {
	code: ErrorCode,
	spans: Vec<Span>,
}

impl Error {
	pub fn new(code: ErrorCode, spans: Vec<Span>) -> Error {
		Error { code, spans }
	}
}

// Error

// Kind
// Description
// Spans