use std::{sync::Arc};

use bolt_ast::{Decl, AstNode};
use prelude::{SourceFile, WithSource, Try};

use crate::{Parser, Parse, Context};

pub struct AstTree {
	file: Arc<SourceFile>,
	decls: Vec<WithSource<Decl>>
}

impl std::fmt::Debug for AstTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.decls)
    }
}

impl AstTree {
	pub fn parse(parser: &mut Parser, ctx: &Context) -> AstTree {
		let mut decls = vec![];

		while !parser.is_at_eof() {
			match Decl::parse(parser, ctx) {
				Try::Some(decl) => decls.push(decl),
				Try::Err(err) | Try::None(err) => {
					parser.emit_error(err);
					break;
				}
			}
		}

		let file = parser.file();

		AstTree {
			file,
			decls
		}
	}

	pub fn into_declarations(self) -> Vec<WithSource<Decl>> {
		self.decls
	}

	pub fn node(&self) -> AstNode {
		let mut node = AstNode::new("file");

		node.set("path", &self.file.file_name().to_string());

		for child in self.decls.iter() {
			node.add_child(child.value().node());
		}

		node
	}
}