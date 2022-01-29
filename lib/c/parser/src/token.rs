use std::fmt::Display;

use prelude::GenericToken;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Ident(String),
	Keyword(String),

	Directive(String),

	NumberLit(u64),
	FloatLit(f64),
	StringLit(String),

	Symbol(String),
	Operator(String),

	MacroReplace(usize),

    EOF,
}

impl GenericToken for Token {
    fn eof( ) -> Self {
        Self::EOF
    }
 }

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(i) => write!(f, "ident `{}`", i),
            Token::Keyword(k) => write!(f, "keyword `{}`", k),
            Token::Directive(dir) => write!(f, "#{dir}"),
            Token::NumberLit(n) => write!(f, "`{}`", n),
            Token::FloatLit(n) => write!(f, "`{}`", n),
            Token::StringLit(s) => write!(f, "string \"{s}\""),
            Token::Symbol(s) => write!(f, "symbol `{s}`"),
            Token::Operator(op) => write!(f, "operator `{op}`"),
            Token::MacroReplace(n) => write!(f, "macro_arg `{n}`"),
            Token::EOF => write!(f, "eof")
        }
    }
}