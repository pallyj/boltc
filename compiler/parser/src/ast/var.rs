/*

Variable 0.3

(*visibility*)? (`let` | `var`) *ident* (`:` **type**)? (`=` **expr**)?

*/

use std::fmt::Debug;

use crate::lexer::SyntaxKind;

use super::{typ::Type, expr::Expr};

ast!(struct VariableDef(VarDef));
ast!(struct LetDef(LetDef));

impl LetDef {
	pub fn visibility(&self) -> Option<SyntaxKind> {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::Visibility)
			.and_then(|visibility| visibility.first_token())
			.map(|tok| tok.kind())
	}

	pub fn label(&self) -> String {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::FuncName)
			.and_then(|func_name| func_name.first_token())
			.map(|name| name.text().to_string())
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

impl Debug for LetDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let visibility = if let Some(visibility) = self.visibility() { format!("{visibility:?} ") } else { "".to_string() };
		let label = self.label();
        let typ = self.typ().map(|typ| format!(": {typ:?}")).unwrap_or("".to_string());
		let value = self.value().map(|value| format!(" = {value:?}")).unwrap_or("".to_string());

		write!(f, "{visibility}let {label}{typ}{value}")
    }
}


impl VariableDef {
	pub fn visibility(&self) -> Option<SyntaxKind> {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::Visibility)
			.and_then(|visibility| visibility.first_token())
			.map(|tok| tok.kind())
	}

	pub fn label(&self) -> String {
		self.0
			.children()
			.find(|child| child.kind() == SyntaxKind::FuncName)
			.and_then(|func_name| func_name.first_token())
			.map(|name| name.text().to_string())
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

impl Debug for VariableDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let visibility = if let Some(visibility) = self.visibility() { format!("{visibility:?} ") } else { "".to_string() };
		let label = self.label();
        let typ = self.typ().map(|typ| format!(": {typ:?}")).unwrap_or("".to_string());
		let value = self.value().map(|value| format!(" = {value:?}")).unwrap_or("".to_string());

		write!(f, "{visibility}let {label}{typ}{value}")
    }
}