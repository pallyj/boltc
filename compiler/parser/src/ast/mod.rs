use std::fmt::Debug;

use self::file::FileItem;
use crate::lexer::SyntaxKind;

pub type SyntaxNode = rowan::SyntaxNode<BoltLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<BoltLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<BoltLanguage>;

pub struct Parse {
    pub file: usize,
    pub root: SyntaxNode,
}

impl Debug for Parse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:#?}", self.root) }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum BoltLanguage {}

impl rowan::Language for BoltLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 < (SyntaxKind::_Invalid as u16));

        unsafe { std::mem::transmute(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind { rowan::SyntaxKind(kind as u16) }
}

macro_rules! ast {
	(struct $name:ident($kind:ident)) => {
		pub struct $name ($crate::ast::SyntaxNode);
		impl $name {
			pub const KIND: $crate::lexer::SyntaxKind = $crate::lexer::SyntaxKind::$kind;

			pub fn range(&self) -> rowan::TextRange {
				self.0.text_range()
			}

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
	(enum $enum_name:ident { $($name:ident, )* }) => {
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
					_ => {
						println!("error {:?}", node);
						$enum_name::Error
					},
				}
			}

			pub fn range(&self) -> rowan::TextRange {
				match self {
					$(
						Self::$name(node) => node.range(),
					)*
					Self::Error => rowan::TextRange::new(0.into(), 1.into())
				}
			}
		}
	};
}

ast!(struct Root(Root));

impl Root {
    pub fn items(&self) -> Vec<FileItem> {
        self.0
            .children()
            .map(|child| FileItem::cast(child))
            .collect()
    }
}

impl Debug for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.items()
                       .iter()
                       .map(|item| format!("{item:?}"))
                       .collect::<Vec<_>>()
                       .join("\n");

        write!(f, "{code}")
    }
}

pub mod attribute;
pub mod containers;
pub mod expr;
pub mod file;
pub mod func;
pub mod smt;
pub mod typ;
pub mod var;
pub mod pattern;
pub mod alias;

fn find_token(node: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    node.children_with_tokens()
        .find(|child| child.kind() == kind)
        .and_then(|matching_child| matching_child.into_token())
}
