use prelude::BoltMessage;

use crate::Token;

#[derive(Debug, Clone)]
pub enum ParseError {
	ExpectedIdent(Token),

	ExpectedOpenParen(Token),
	ExpectedCloseParen(Token),

	ExpectedOpenBrace(Token),
	ExpectedCloseBrace(Token),

	ExpectedOpenBracket(Token),
	ExpectedCloseBracket(Token),

    ExpectedExpr(Token),
    ExpectedType(Token),
    ExpectedStatement(Token),
    ExpectedDecl(Token),
    ExpectedVisibility(Token),
    ExpectedAttribute(Token),

    ExpectedFunc(Token),
    ExpectedImport(Token),
    ExpectedLet(Token),
    ExpectedStruct(Token),
    ExpectedStructItem(Token),
    ExpectedEnum(Token),
    ExpectedEnumVariant(Token),
    ExpectedEnumItem(Token),

    ExpectedColon(Token),

    // TODO: This is a unit
    EmptyTuple(Token),

    ExpectedMemberName(Token),
    ExpectedEnumVariantLiteral(Token),


    UnexpectedToken(Token),
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

pub enum LexError {}

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