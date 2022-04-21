use std::fmt::Debug;

use super::{attribute::Attributes,
            file::NoOp,
            func::FuncDef,
            var::{LetDef, VariableDef}};
use crate::lexer::SyntaxKind;

// Struct 0.3
//
// (*visibility*)? `struct` (*ident*)? `{` **struct_item** `}`
//
// struct_item:
// constant
// variable
// struct
// function
//

ast!(struct StructBody(StructBody));
ast!(struct StructDef(StructDef));

ast!(
    enum StructItem {
        StructDef,
        FuncDef,
        VariableDef,
        LetDef,
        NoOp,
    }
);

impl Debug for StructItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StructDef(arg0) => write!(f, "{arg0:?}"),
            Self::FuncDef(arg0) => write!(f, "{arg0:?}"),
            Self::VariableDef(arg0) => write!(f, "{arg0:?}"),
            Self::LetDef(arg0) => write!(f, "{arg0:?}"),
            Self::NoOp(_) => write!(f, ";"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl StructDef {
    pub fn attributes(&self) -> Attributes { self.0.children().find_map(Attributes::cast).unwrap() }

    pub fn visibility(&self) -> Option<SyntaxKind> {
        self.0
            .children()
            .find(|child| child.kind() == SyntaxKind::Visibility)
            .and_then(|visibility| visibility.first_token())
            .map(|tok| tok.kind())
    }

    pub fn name(&self) -> String {
        self.0
            .children()
            .find(|child| child.kind() == SyntaxKind::FuncName)
            .and_then(|func_name| func_name.first_token())
            .map(|name| name.text().to_string())
            .unwrap()
    }

    pub fn body(&self) -> StructBody {
        self.0
            .children()
            .find(|child| child.kind() == SyntaxKind::StructBody)
            .and_then(StructBody::cast)
            .unwrap()
    }
}

impl Debug for StructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let visibility = if let Some(visibility) = self.visibility() {
            format!("{visibility:?} ")
        } else {
            "".to_string()
        };
        let name = self.name();
        let body = self.body();

        write!(f, "{visibility}struct {name} {body:?}")
    }
}

impl StructBody {
    pub fn items(&self) -> Vec<StructItem> { self.0.children().map(StructItem::cast).collect() }
}

impl Debug for StructBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let struct_items = self.items()
                               .iter()
                               .map(|struct_item| format!("\t{struct_item:?}").replace("\n", "\n\t"))
                               .collect::<Vec<_>>()
                               .join("\n");

        write!(f, "{{\n{struct_items}\n}}")
    }
}
