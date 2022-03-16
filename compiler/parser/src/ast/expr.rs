/*

Expr syntax 0.3

Integer literal: *int_lit*
Float literal: *float_lit*
Named: *ident*

Member: **expr** `.` *ident*
Function Call: **expr** `(` (**function arg**),* `)` 

Paren: `(` **expr** `)`

PrefixOp: *op* **expr**
PostfixOp: **expr** *op*
InfixOp: **expr** *op* **expr**

If: `if` **expr** **codeblock**
	(`else` **codeblock** | **if_smt**)?

*/

use std::fmt::Debug;

use crate::lexer::SyntaxKind;

use super::{smt::CodeBlock};

ast!(struct NamedExpr(NamedExpr));
ast!(struct LiteralExpr(Literal));
ast!(struct ParenthesizedExpr(ParenthesizedExpr));
ast!(struct IfExpr(IfExpr));
ast!(struct MemberExpr(MemberExpr));
ast!(struct FuncCallExpr(FuncCallExpr));

ast!(enum Expr {
	NamedExpr,
	LiteralExpr,
	ParenthesizedExpr,
	IfExpr,
	MemberExpr,
	FuncCallExpr
});

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NamedExpr(arg0) => write!(f, "{arg0:?}"),
            Self::LiteralExpr(arg0) => write!(f, "{arg0:?}"),
            Self::ParenthesizedExpr(arg0) => write!(f, "{arg0:?}"),
            Self::IfExpr(arg0) => write!(f, "{arg0:?}"),
            Self::MemberExpr(arg0) => write!(f, "{arg0:?}"),
            Self::FuncCallExpr(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "Error"),
        }
    }
}

ast!(enum IfExprNegative {
	IfExpr,
	CodeBlock
});

impl Debug for IfExprNegative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IfExpr(arg0) => write!(f, "{arg0:?}"),
            Self::CodeBlock(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl NamedExpr {
	pub fn name(&self) -> String {
		self.0
			.first_token()
			.map(|token| token.text().to_string())
			.unwrap()
	}
}

impl Debug for NamedExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub enum LiteralKind {
	True,
	False,
	DecInteger,
	HexInteger,
	OctInteger,
	BinInteger,
	DecFloat,
	Error,
}

impl LiteralExpr {
	pub fn text(&self) -> String {
		self.0
			.first_token()
			.map(|token| token.text().to_string())
			.unwrap()
	}

	pub fn literal_kind(&self) -> LiteralKind {
		self.0
			.first_token()
			.map(|tok| match tok.kind() {
				SyntaxKind::LiteralBinInt => LiteralKind::BinInteger,
				SyntaxKind::LiteralOctInt => LiteralKind::OctInteger,
				SyntaxKind::LiteralHexInt => LiteralKind::HexInteger,
				SyntaxKind::LiteralDecInt => LiteralKind::DecInteger,
				SyntaxKind::LiteralDecFloat => LiteralKind::DecFloat,
				SyntaxKind::LiteralFalse => LiteralKind::False,
				SyntaxKind::LiteralTrue => LiteralKind::True,
				_ => LiteralKind::Error
			})
			.unwrap_or(LiteralKind::Error)
	}
}

impl Debug for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl ParenthesizedExpr {
	pub fn expr(&self) -> Expr {
		self.0
			.first_child()
			.map(|expr| Expr::cast(expr.clone()))
			.unwrap_or(Expr::Error)
	}
}

impl Debug for ParenthesizedExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?})", self.expr())
    }
}

impl MemberExpr {
	pub fn parent(&self) -> Expr {
		Expr::cast(
		self.0
				.first_child()
				.unwrap()
				.clone()
		)
	}

	pub fn child(&self) -> Option<String> {
		self.0.last_token()
			.map(|token| token.text().to_string())
	}
}

impl Debug for MemberExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}.{}", self.parent(), self.child().unwrap_or("".to_string()))
    }
}

impl IfExpr {
	pub fn condition(&self) -> Expr {
		self.0
			.children()
			.find(|syn| syn.kind() == SyntaxKind::Condition)
			.and_then(|condition| condition.first_child())
			.map(|condition| Expr::cast(condition.clone()))
			.unwrap()
	}

	pub fn positive(&self) -> CodeBlock {
		self.0
			.children()
			.find(|syn| syn.kind() == SyntaxKind::Positive)
			.and_then(|positive| positive.first_child())
			.and_then(|condition| { CodeBlock::cast(condition.clone()) })
			.unwrap()
	}

	pub fn negative(&self) -> Option<IfExprNegative> {
		self.0
			.children()
			.find(|syn| syn.kind() == SyntaxKind::Negative)
			.and_then(|positive| positive.first_child())
			.map(|condition| IfExprNegative::cast(condition.clone()))
	}
}

impl FuncCallExpr {
	pub fn function(&self) -> Expr {
		self.0
			.first_child()
			.map(|function| Expr::cast(function))
			.unwrap()
	}

	pub fn args(&self) -> Vec<Expr> {
		self.0
			.children()
			.find(|node| node.kind() == SyntaxKind::CommaSeparatedList)
			.unwrap()
			.children()
			.map(|child| Expr::cast(child.clone()))
			.collect()
	}
}

impl Debug for FuncCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self.args()
			.iter()
			.map(|arg| format!("{arg:?}"))
			.collect::<Vec<_>>()
			.join(", ");

		write!(f, "{:?}({})", self.function(), args)
    }
}

impl Debug for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(negative) = self.negative() {
			write!(f, "if {:?} {:?} {:?}",
				self.condition(),
				self.positive(),
				negative)
		} else {
			write!(f, "if {:?} {:?}",
				self.condition(),
				self.positive())
		}
    }
}