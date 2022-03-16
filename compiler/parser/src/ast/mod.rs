use std::fmt::Debug;

use crate::{lexer::SyntaxKind, ast::{func::FuncDef}};

pub type SyntaxNode = rowan::SyntaxNode<BoltLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<BoltLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<BoltLanguage>;

pub struct Parse {
	pub (crate) root: SyntaxNode,
}

impl Debug for Parse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:#?}", self.root)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum BoltLanguage {}

impl rowan::Language for BoltLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
		assert!(raw.0 < (SyntaxKind::_Invalid as u16));

		unsafe {
			std::mem::transmute(raw.0)
		}
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

macro_rules! ast {
	(struct $name:ident($kind:ident)) => {
		pub struct $name ($crate::ast::SyntaxNode);
		impl $name {
			pub const KIND: $crate::lexer::SyntaxKind = $crate::lexer::SyntaxKind::$kind;

			pub fn cast(node: $crate::ast::SyntaxNode) -> Option<Self> {
				if node.kind() == $crate::lexer::SyntaxKind::$kind {
					Some(Self(node))
				} else {
					None
				}
			}

			pub unsafe fn unsafe_cast(node: $crate::ast::SyntaxNode) -> Self {
				Self(node)
			}
		}
	};
	(enum $enum_name:ident { $($name:ident),* }) => {
		pub enum $enum_name {
			$(
				$name($name),
			)*
			Error
		}

		impl $enum_name {
			pub fn cast(node: $crate::ast::SyntaxNode) -> Self {
				match node.kind() {
					$(
						$name::KIND => unsafe { <$enum_name>::$name($name::unsafe_cast(node)) },
					)*
					_ => $enum_name::Error,
				}
			}
		}
	};
}

ast!(struct Root(Root));

impl Debug for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", FuncDef::cast(self.0
			.first_child().unwrap().clone()))
    }
}


pub mod file;
pub mod typ;
pub mod expr;
pub mod smt;
pub mod var;
pub mod func;
pub mod containers;