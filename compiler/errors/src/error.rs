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

	MismatchedType { expected: String },

	AmbiguousTy,

	ExpectedFound(String, String),
	MismatchedIfBranchTypes,

	ExtraParams,
	MissingParams,

	IsNotAFunc
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
			ErrorCode::MismatchedType { expected } => format!("Mismatched type, expected '{expected}' found this"),
			ErrorCode::TypeIsNotAnInteger => format!("Mismatched type, expected '' found this"),
			ErrorCode::TypeIsNotAFloat => format!("Mismatched type, expected '' found this"),
			ErrorCode::TypeIsNotABool => format!("Mismatched type, expected '' found this"),
			ErrorCode::ExpectedFound(expected, found) => format!("Expected {expected}, found {found}"),
			ErrorCode::ExtraParams => format!("Found extra parameters"),
			ErrorCode::MissingParams => format!("Missing argument for parameter"),
			ErrorCode::MismatchedIfBranchTypes => format!("If branches have mismatched types"),
			ErrorCode::IsNotAFunc => format!("Value is not a function"),
		}
	}
}

#[allow(dead_code)]
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