use prelude::GenericToken;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Ident(String),
	OpenParen,
	CloseParen,
	Comma,
	EOF,
}

impl GenericToken for Token {
    fn eof() -> Self {
        Self::EOF
    }
}