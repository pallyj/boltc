use std::fmt::Debug;

use cstree::{interning::Resolver};

use crate::lexer::SyntaxKind;

pub mod file;
pub mod typ;
pub mod expr;
pub mod smt;
pub mod var;
pub mod func;
pub mod containers;

pub type SyntaxNode = cstree::SyntaxNode<BoltLanguage>;
pub type SyntaxToken = cstree::SyntaxToken<BoltLanguage>;
pub type SyntaxElement = cstree::SyntaxElement<BoltLanguage>;

pub struct Parse<I: Resolver> {
	pub (crate) root: SyntaxNode,
	pub (crate) resolver: I,
}

impl<I: Resolver> Debug for Parse<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.root.write_debug(&self.resolver, f, true)
    }
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum BoltLanguage {}

impl cstree::Language for BoltLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: cstree::SyntaxKind) -> Self::Kind {
		assert!(raw.0 < (SyntaxKind::_Invalid as u16));

		unsafe {
			std::mem::transmute(raw.0)
		}
    }

    fn kind_to_raw(kind: Self::Kind) -> cstree::SyntaxKind {
        cstree::SyntaxKind(kind as u16)
    }
}