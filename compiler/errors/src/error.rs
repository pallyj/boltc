use crate::Span;

#[derive(error_derive::Error)]
pub enum ErrorCode {
	TypeNotFound { name: String },
	SymNotAType { name: String },
	SymbolNotFound { name: String },
	SymNotAValue { name: String },
	MemberNotFound { name: String },
	MemberNotATy { name: String },
	MemberNotAVal { name: String },

	TypeIsNotAnInteger,
	TypeIsNotAFloat,
	TypeIsNotABool,

	MismatchedType,

	AmbiguousTy,
}

impl ErrorCode {
	pub fn description(&self) -> String {
		match self {
			ErrorCode::TypeNotFound { name } => format!("Type {} not found", name),
			ErrorCode::SymNotAType { name } => format!("Symbol {} is not a type", name),
			ErrorCode::SymbolNotFound { name } => format!("Symbol {} not found", name),
			ErrorCode::SymNotAValue { name } => format!("Symbol {} is not a value", name),
			ErrorCode::MemberNotFound { name } => format!("Member {} not found", name),
			ErrorCode::MemberNotATy { name } => format!("Member {} is not a type", name),
			ErrorCode::MemberNotAVal { name } => format!("Member {} is not a value", name),
			ErrorCode::AmbiguousTy => format!("Couldn't infer a type for"),
			ErrorCode::MismatchedType => format!("Mismatched type, expected '' found this"),
			ErrorCode::TypeIsNotAnInteger => format!("Mismatched type, expected '' found this"),
			ErrorCode::TypeIsNotAFloat => format!("Mismatched type, expected '' found this"),
			ErrorCode::TypeIsNotABool => format!("Mismatched type, expected '' found this"),
		}
	}
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