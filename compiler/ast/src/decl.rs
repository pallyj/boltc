use std::fmt::Display;

use crate::{Func, Import, Let, Struct, AstNode, Enum, Var};

#[derive(Debug, Clone)]
pub enum Decl {
	Func(Func),
	Import(Import),
	Let(Let),
	Var(Var),
	Struct(Struct),
	Enum(Enum),
}

impl Decl {
	pub fn node(&self) -> AstNode {
		match self {
			Decl::Func(f) => f.node(),
			Decl::Struct(s) => s.node(),
			Decl::Let(l) => l.node(),
			Decl::Import(i) => i.node(),
			Decl::Enum(e) => e.node(),
			Decl::Var(v) => v.node(),
		}
	}
}