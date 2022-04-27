// File 0.3
//
// (**file_item**)*
//
// file_item:
// import |
// constant |
// func |
// struct
//
// import:
// `import` **ident**
//

use core::fmt::Debug;

use super::{containers::{StructDef, EnumDef}, find_token, func::FuncDef, var::LetDef};
use crate::lexer::SyntaxKind;

ast!(struct ImportDef(Import));
ast!(struct NoOp(NoOp));

impl ImportDef {
    pub fn import_library(&self) -> String {
        find_token(&self.0, SyntaxKind::Ident).map(|token| token.text().to_string())
                                              .unwrap_or_else(|| "".to_string())
    }
}

impl Debug for ImportDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "import {}", self.import_library()) }
}

ast!(
    enum FileItem {
        StructDef,
        FuncDef,
        ImportDef,
        LetDef,
        EnumDef,
        NoOp,
    }
);

impl Debug for FileItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StructDef(arg0) => write!(f, "{arg0:?}"),
            Self::FuncDef(arg0) => write!(f, "{arg0:?}"),
            Self::ImportDef(arg0) => write!(f, "{arg0:?}"),
            Self::LetDef(arg0) => write!(f, "{arg0:?}"),
            Self::EnumDef(arg0) => write!(f, "{arg0:?}"),
            Self::NoOp(_) => write!(f, ";"),
            Self::Error => write!(f, "Error"),
        }
    }
}
