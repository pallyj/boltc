use std::fmt::Debug;

use crate::lexer::SyntaxKind;

use super::{expr::{Expr}, find_token};

ast!(struct WildcardPattern(WildcardPattern));
ast!(struct LiteralPattern(LiteralPattern));
ast!(struct VariantPattern(VariantPattern));
ast!(struct TuplePattern(TuplePattern));

ast!(enum Pattern {
	WildcardPattern,
	LiteralPattern,
    VariantPattern,
    TuplePattern,
});

impl LiteralPattern {
    pub fn value(&self) -> Expr {
        self.0.first_child()
            .map(Expr::cast)
            .unwrap()
    }
}

impl Debug for LiteralPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value())
    }
}

impl Debug for WildcardPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "_")
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WildcardPattern(arg0) => write!(f, "{arg0:?}"),
            Self::LiteralPattern(arg0) => write!(f, "{arg0:?}"),
            Self::VariantPattern(arg0) => write!(f, "{arg0:?}"),
            Self::TuplePattern(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl VariantPattern {
    pub fn variant_name(&self) -> String {
        find_token(&self.0, SyntaxKind::Ident).unwrap().text().to_string()
    }
}

impl Debug for VariantPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".{}", self.variant_name())
    }
}

impl TuplePattern {
    pub fn tuple_items(&self) -> impl Iterator<Item = Pattern> {
        self.0.first_child()
            .unwrap()
            .children()
            .map(Pattern::cast)
    }
}

impl Debug for TuplePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items =
            self.tuple_items()
                .map(|item| format!("{item:?}"))
                .collect::<Vec<_>>()
                .join(", ");
        
        write!(f, "({items})")
    }
}