/*

Function 0.3

*visibility* (*static*)? `func` (*ident*)? `(` (**func_arg**),* `)` (`:` **type**)? (`{` **code_block** `}`)?

func_arg: (*ident*)? *ident* `:` **type**

Code Block

(**smt**)*

*/

use std::fmt::Debug;

use crate::lexer::SyntaxKind;

use super::{typ::Type, smt::CodeBlock};

ast!(struct FuncPar(FuncPar));
ast!(struct FuncDef(FuncDef));

impl FuncDef {
	pub fn visibility(&self) -> Option<SyntaxKind> {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::Visibility)
			.and_then(|visibility| visibility.first_token())
			.map(|tok| tok.kind())
	}

	pub fn name(&self) -> String {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::FuncName)
			.and_then(|func_name| func_name.first_token())
			.map(|name| name.text().to_string())
			.unwrap()
	}

	pub fn parameters(&self) -> Vec<FuncPar> {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::CommaSeparatedList)
			.unwrap()
			.children()
			.map(|par| FuncPar::cast(par).unwrap())
			.collect()
	}

	pub fn return_type(&self) -> Option<Type> {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::FuncReturn)
			.and_then(|return_node| return_node.first_child())
			.map(|return_ty| Type::cast(return_ty))
	}

	pub fn code(&self) -> CodeBlock {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::CodeBlock)
			.and_then(|block_node| CodeBlock::cast(block_node))
			.unwrap()
	}
}

impl Debug for FuncDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vis = if let Some(vis) = self.visibility() { format!("{vis:?} ") } else { "".to_string() };
		let name = self.name();
		let params = self.parameters()
			.iter()
			.map(|p| format!("{p:?}"))
			.collect::<Vec<_>>()
			.join(", ");
		let return_type = if let Some(rt) = self.return_type() { format!(": {rt:?}") } else { "".to_string() };
		let code = format!("{:?}", self.code());

		write!(f, "{vis}func {name}({params}){return_type} {code}")
    }
}

impl FuncPar {
	pub fn label(&self) -> String {
		self.0
			.first_token()
			.map(|t| t.text().to_string())
			.unwrap_or_else(|| "".to_string())
	}

	pub fn typ(&self) -> Type {
		self.0
			.last_child()
			.map(|last| Type::cast(last))
			.unwrap_or(Type::Error)
	}
}

impl Debug for FuncPar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.label(), self.typ())
    }
}