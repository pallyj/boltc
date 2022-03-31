use crate::lexer::{SyntaxKind, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
	Expected (SyntaxKind)
}

impl ParseError {
	pub fn to_string(&self, next: Option<Token>) -> String {
		match self {
			ParseError::Expected(expected) => format!("expected {}, found {}", token_group(*expected), token_specific(next)),
		}
	}
}

fn token_group(kind: SyntaxKind) -> &'static str {
	match kind {
		SyntaxKind::Ident => "identifier",

		_ => "other"
	}
}

fn token_specific(token: Option<Token>) -> String {
	let Some(token) = token else {
		return "<eof>".to_string();
	};

	match &token.kind {
		SyntaxKind::StructKw | SyntaxKind::ImportKw |
		SyntaxKind::FuncKw | SyntaxKind::InitKw |
		SyntaxKind::LetKw | SyntaxKind::VarKw |
		SyntaxKind::IfKw | SyntaxKind::ElseKw |
		SyntaxKind::ReturnKw |
		SyntaxKind::StaticKw |
		SyntaxKind::PublicKw | SyntaxKind::InternalKw |
		SyntaxKind::FilePrivateKw | SyntaxKind::PrivateKw |
		SyntaxKind::UnderscoreKw =>  format!("keyword `{}`", token.source),

		SyntaxKind::Comment =>  format!("comment"),
		SyntaxKind::Whitespace =>  format!("whitespace"),
		SyntaxKind::Error =>  format!("error"),

		_ =>  format!("`{}`", token.source),
	}
}