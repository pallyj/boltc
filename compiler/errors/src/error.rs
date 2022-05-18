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

    FunctionSigNotFound,
    AmbiguousFunc,

    AttributeDoesNotExist(String),
    IsNotAFunc,

    OperatorDNE(String),

    OperatorNotDefined(String, String),

    OperatorExpectedParams(String, usize),
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
            ErrorCode::MismatchedType { expected } => format!("Mismatched type: expected '{expected}', found this"),
            ErrorCode::TypeIsNotAnInteger => format!("Mismatched type: expected '', found integer"),
            ErrorCode::TypeIsNotAFloat => format!("Mismatched type: expected '', found float"),
            ErrorCode::TypeIsNotABool => format!("Mismatched type: expected '', found bool"),
            ErrorCode::ExpectedFound(expected, found) => format!("Mismatched type: Expected {expected}, found {found}"),
            ErrorCode::ExtraParams => format!("Found extra parameters"),
            ErrorCode::MissingParams => format!("Missing argument for parameter"),
            ErrorCode::MismatchedIfBranchTypes => format!("If branches have mismatched types"),
            ErrorCode::IsNotAFunc => format!("Value is not a function"),
            ErrorCode::FunctionSigNotFound => format!("No function matching this signature was found"),
            ErrorCode::AttributeDoesNotExist(name) => format!("cannot find attribute `{name}`"),
            ErrorCode::AmbiguousFunc => format!("ambiguous function signatures"),
            ErrorCode::OperatorDNE(name) => format!("operator `{name}` is not defined"),
            ErrorCode::OperatorNotDefined(operator, typ) => format!("operator `{operator}` is not defined on {typ}",),
            ErrorCode::OperatorExpectedParams(name, n_params) => format!("operator {name} takes {n_params} parameter"),
        }
    }
}

#[allow(dead_code)]
pub struct Error {
    code:  ErrorCode,
    spans: Vec<Span>,
}

impl Error {
    pub fn new(code: ErrorCode, spans: Vec<Span>) -> Error { Error { code, spans } }
}

// Error

// Kind
// Description
// Spans
