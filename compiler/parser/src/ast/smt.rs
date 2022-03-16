/*

Statements 0.3

Eval: **expr** (`;`)?
Return: `return` (**expr**)?
Bind: `let` *ident* (`:` **type**) `=` **expr**
NoOp: `;`

*/

use std::fmt::Debug;

use crate::lexer::SyntaxKind;

use super::{expr::Expr, typ::Type};

ast!(struct EvalSmt(EvalSmt));
ast!(struct ReturnSmt(ReturnSmt));
ast!(struct LetSmt(LetSmt));
ast!(struct NoOp(NoOp));

ast!(enum Smt {
	EvalSmt,
	ReturnSmt,
	LetSmt,
	NoOp
});

impl Debug for Smt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EvalSmt(arg0) => write!(f, "{arg0:?}"),
            Self::ReturnSmt(arg0) => write!(f, "{arg0:?}"),
            Self::LetSmt(arg0) => write!(f, "{arg0:?}"),
            Self::NoOp(_) => write!(f, ";"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl EvalSmt {
	pub fn value(&self) -> Expr {
		self.0
			.first_child()
			.map(Expr::cast)
			.unwrap()
	}

	pub fn is_escaped(&self) -> bool {
		self.0
			.last_token()
			.map(|tok| tok.kind() == SyntaxKind::Semicolon)
			.unwrap_or(false)
	}
}

impl Debug for EvalSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_escaped() {
			write!(f, "{:?};", self.value())
		} else {
			write!(f, "{:?}", self.value())
		}
    }
}

impl ReturnSmt {
	pub fn return_value(&self) -> Option<Expr> {
		self.0
			.first_child()
			.map(Expr::cast)
	}
}

impl Debug for ReturnSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(return_value) = self.return_value() {
			write!(f, "return {return_value:?}")
		} else {
			write!(f, "return")
		}
    }
}

impl LetSmt {
	pub fn label(&self) -> String {
		self.0
			.children_with_tokens()
			.find(|element| element.kind() == SyntaxKind::Ident)
			.and_then(|element| element.into_token())
			.map(|token| token.text().to_string())
			.unwrap()
	}

	pub fn typ(&self) -> Option<Type> {
		self.0
			.children()
			.find(|element| element.kind() == SyntaxKind::BindType)
			.and_then(|element| element.first_child())
			.map(|element| Type::cast(element))
	}

	pub fn value(&self) -> Option<Expr> {
		self.0
			.children()
			.find(|element| element.kind() == SyntaxKind::AssignValue)
			.and_then(|element| element.first_child())
			.map(|element| Expr::cast(element))
	}
}

impl Debug for LetSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let label = self.label();
        let typ = self.typ().map(|typ| format!(": {typ:?}")).unwrap_or("".to_string());
		let value = self.value().map(|value| format!(" = {value:?}")).unwrap_or("".to_string());

		write!(f, "let {label}{typ}{value}")
    }
}

ast!(struct CodeBlock(CodeBlock));

impl CodeBlock {
	pub fn statements(&self) -> Vec<Smt> {
		self.0
			.children()
			.map(|smt| Smt::cast(smt.clone()))
			.collect()
	}
}

impl Debug for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = self.0
			.children()
			.map(|smt| Smt::cast(smt.clone()))
			.map(|smt| format!("{smt:?}").replace("\n", "\n\t"))
			.collect::<Vec<_>>()
			.join("\n");

		write!(f, "{{\n{statements}\n}}")
    }
}