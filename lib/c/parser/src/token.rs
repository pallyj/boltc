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

	MacroReplace(usize)
}

impl GenericToken for Token { }