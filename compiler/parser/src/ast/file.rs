/*

File 0.3

(**file_item**)*

file_item:
	import |
	constant |
	func |
	struct

import:
	`import` **ident**

*/

use crate::lexer::SyntaxKind;

use super::{containers::StructDef, func::FuncDef, find_token};
use core::fmt::Debug;

ast!(struct ImportDef(Import));

impl ImportDef {
	pub fn import_library(&self) -> String {
		find_token(&self.0, SyntaxKind::Ident)
			.map(|token| token.text().to_string())
			.unwrap_or("".to_string())
	}
}

impl Debug for ImportDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "import {}", self.import_library())
    }
}

ast!(enum FileItem {
	StructDef,
	FuncDef,
	ImportDef
});

impl Debug for FileItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StructDef(arg0) => write!(f, "{arg0:?}"),
            Self::FuncDef(arg0) => write!(f, "{arg0:?}"),
			Self::ImportDef(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "Error"),
        }
    }
}