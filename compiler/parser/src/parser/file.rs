use cstree::interning::{IntoResolver, Resolver};

use crate::{lexer::SyntaxKind, ast::{SyntaxNode, Parse}};

use super::Parser;

impl<'a> Parser<'a> {
	pub fn parse_file(mut self) -> Parse<impl Resolver> {
		self.start_node(SyntaxKind::Root);

		self.parse_ty();

		self.finish_node();

		let (green, cache) = self.builder.finish();

		Parse {
			root: SyntaxNode::new_root(green),
			resolver: cache.unwrap().into_interner().unwrap().into_resolver()
		}
	}
}