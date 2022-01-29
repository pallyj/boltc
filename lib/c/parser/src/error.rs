use std::ptr::NonNull;

use prelude::{WithSource, BoltMessage};

use crate::Token;

#[derive(Debug, Clone)]
pub enum ParseError {
    /// Expected identifier in struct definition, found 
    /// ^^^^
    /// Add label
	ExpectedIdentInStruct(Token),
	ExpectedIdentInEnum(Token),
	ExpectedIdentInUnion(Token),
	ExpectedType(Token),

    UnexpectedToken(Token),

    ExpectedFunctionPointerStar(Token),

    ExpectedClosingParen(Token),
    ExpectedOpeningParen(Token),

    ExpectedClosingBrace(Token),
    ExpectedOpeningBrace(Token),

    MissingSemicolon(Token),

    ExpectedArrayLen(Token),

    ExpectedIdent(Token),

	EOF
}

pub enum LexError {

}

impl BoltMessage for ParseError {
    fn code(&self) -> String {
        "C001".to_string()
    }

    fn suggestion(&self) -> Option<String> {
        match self {
            Self::ExpectedIdentInStruct(tok) => {
                None
            }
            Self::MissingSemicolon(tok) => {
                Some("Add semicolon".to_string())
            }
            Self::ExpectedArrayLen(tok) => {
                None
            }
            _ => None
        }
    }

    fn description(&self) -> String {
        match self {
            Self::ExpectedIdentInStruct(tok) => {
                format!("Expected identifier in struct definition, found {}", tok)
            }
            Self::MissingSemicolon(tok) => {
                format!("Expected semicolon, found {}", tok)
            }
            Self::ExpectedArrayLen(tok) => {
                format!("Expected array length, found {}", tok)
            }
            _ => "".to_string()
        }
    }

    fn level(&self) -> prelude::MessageLevel {
        prelude::MessageLevel::Error
    }
}

impl BoltMessage for LexError {
    fn code(&self) -> String {
        todo!()
    }

    fn suggestion(&self) -> Option<String> {
        None
    }

    fn description(&self) -> String {
        todo!()
    }

    fn level(&self) -> prelude::MessageLevel {
        todo!()
    }
}