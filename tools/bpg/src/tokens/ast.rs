use prelude::WithSource;

#[derive(Debug)]
pub enum TokenParam {
	String,
	UInt,
	Int,
}

#[derive(Debug)]
pub struct TokenDef {
	name: WithSource<String>,
	params: Vec<WithSource<TokenParam>>
}

#[derive(Debug)]
pub struct Ast {
	tokens: Vec<WithSource<TokenDef>>
}

impl TokenDef {
	pub fn new(name: WithSource<String>, params: Vec<WithSource<TokenParam>>) -> Self {
		TokenDef {
			name,
			params
		}
	}
}

impl Ast {
	pub fn new(tokens: Vec<WithSource<TokenDef>>) -> Self {
		Self {
			tokens
		}
	}
}