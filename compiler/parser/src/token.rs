use prelude::GenericToken;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Keyword(String),
	Ident(String),

	IntLit(u64),
	FloatLit(f64),
	StringLit(String),

	Punctuation(String),

	Comment(String),

	EOF
}

impl GenericToken for Token {
    fn eof() -> Self {
        Self::EOF
    }
}