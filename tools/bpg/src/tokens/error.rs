use prelude::BoltMessage;

use super::token::Token;

pub enum LexError {

}

impl BoltMessage for LexError {
    fn code(&self) -> String {
        todo!()
    }

    fn suggestion(&self) -> Option<String> {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }

    fn level(&self) -> prelude::MessageLevel {
        todo!()
    }
}

#[derive(Debug)]
pub enum ParseError {
	ExpectedOpenParen(Token),
	ExpectedCloseParen(Token),
	ExpectedComma(Token),
	ExpectedIdent(Token),
	ExpectedType(Token),
	
}

impl BoltMessage for ParseError {
    fn code(&self) -> String {
        todo!()
    }

    fn suggestion(&self) -> Option<String> {
        todo!()
    }

    fn description(&self) -> String {
        todo!()
    }

    fn level(&self) -> prelude::MessageLevel {
        todo!()
    }
}