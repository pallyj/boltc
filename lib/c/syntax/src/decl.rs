use std::{fmt::Display, env::VarError};

use prelude::WithSource;

use crate::{Type, TypeDecl};

#[derive(Debug)]
pub enum Decl {
	TypeDef(TypeDef),
	Function(Function),
	Variable(Variable),
}

#[derive(Debug)]
pub struct TypeDef {
	pub name: String,
	pub typ: Type,	
}

#[derive(Debug)]
pub struct Variable {
	pub name: Option<String>,
	pub typ: Type,
	//pub default: Option<Expression>,
}

#[derive(Debug)]
pub struct Function {
	pub return_type: Type,
	pub name: String,
	pub params: Vec<WithSource<TypeDecl>>,
	pub code: ()
}

impl Display for TypeDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "typedef {:?} {};", self.typ, self.name)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {};", self.typ, self.name.as_ref().map(|s| s.as_str()).unwrap_or(&""))
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}()", self.return_type, self.name)
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decl::TypeDef(d) => write!(f, "{}", d),
            Decl::Function(d) => write!(f, "{}", d),
            Decl::Variable(d) => write!(f, "{}", d),
        }
    }
}