use std::fmt::Debug;

use super::{attribute::Attributes,
            file::NoOp,
            func::FuncDef,
            var::{LetDef, VariableDef}, find_token, typ::Type};
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

ast!(struct CaseDef(CaseDef));
ast!(struct CaseItem(CaseItem));
ast!(struct EnumBody(EnumBody));
ast!(struct EnumDef(EnumDef));

ast!(
    enum EnumItem {
        StructDef,
        FuncDef,
        VariableDef,
        LetDef,
        CaseDef,
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

impl CaseDef {
    pub fn items(&self) -> impl Iterator<Item=CaseItem> {
        self.0.children()
            .filter_map(CaseItem::cast)
    }
}

impl Debug for CaseDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self.items().map(|item| format!("{item:?}")).collect::<Vec<_>>().join(", ");
        write!(f, "case {items}")
    }
}

impl CaseItem {
    pub fn name(&self) -> String {
        find_token(&self.0, SyntaxKind::Ident)
            .unwrap()
            .text()
            .to_string()
    }
}

impl Debug for CaseItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Debug for EnumItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StructDef(arg0) => write!(f, "{arg0:?}"),
            Self::FuncDef(arg0) => write!(f, "{arg0:?}"),
            Self::VariableDef(arg0) => write!(f, "{arg0:?}"),
            Self::LetDef(arg0) => write!(f, "{arg0:?}"),
            Self::CaseDef(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl EnumDef {
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

    pub fn repr_type(&self) -> Option<Type> {
        self.0
            .children()
            .find(|child| child.kind() == SyntaxKind::FuncReturn)
            .and_then(|return_node| return_node.first_child())
            .map(Type::cast)
    }

    pub fn body(&self) -> EnumBody {
        self.0
            .children()
            .find(|child| child.kind() == SyntaxKind::EnumBody)
            .and_then(EnumBody::cast)
            .unwrap()
    }
}

impl Debug for EnumDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let visibility = if let Some(visibility) = self.visibility() {
            format!("{visibility:?} ")
        } else {
            "".to_string()
        };
        let name = self.name();
        let body = self.body();

        write!(f, "{visibility}enum {name} {body:?}")
    }
}


impl EnumBody {
    pub fn items(&self) -> impl Iterator<Item=EnumItem> { self.0.children().map(EnumItem::cast) }
}

impl Debug for EnumBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let struct_items = self.items()
                               .map(|enum_item| format!("\t{enum_item:?}").replace("\n", "\n\t"))
                               .collect::<Vec<_>>()
                               .join("\n");

        write!(f, "{{\n{struct_items}\n}}")
    }
}