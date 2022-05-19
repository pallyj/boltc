use std::fmt::Debug;

use crate::lexer::SyntaxKind;

use super::{expr::{Expr}, find_token};

ast!(struct WildcardPattern(WildcardPattern));
ast!(struct LiteralPattern(LiteralPattern));
ast!(struct VariantPattern(VariantPattern));
ast!(struct TuplePattern(TuplePattern));
ast!(struct TupleMember(FuncArg));
ast!(struct BindPattern(BindPattern));

ast!(enum Pattern {
	WildcardPattern,
	LiteralPattern,
    VariantPattern,
    TuplePattern,
    BindPattern,
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
            Self::BindPattern(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl VariantPattern {
    pub fn variant_name(&self) -> String {
        find_token(&self.0, SyntaxKind::Ident).unwrap().text().to_string()
    }

    pub fn associated_patterns(&self) -> Option<impl Iterator<Item = TupleMember>> {
        Some(self.0.last_child()?
            .children()
            .filter_map(TupleMember::cast))
    }
}

impl Debug for VariantPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".{}", self.variant_name())
    }
}

impl TuplePattern {
    pub fn tuple_items(&self) -> impl Iterator<Item = TupleMember> {
        self.0.first_child()
            .unwrap()
            .children()
            .filter_map(TupleMember::cast)
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

impl TupleMember {
    pub fn label(&self) -> Option<String> {
        find_token(&self.0, SyntaxKind::Ident)
            .map(|token| token.text().to_string())
    }

    pub fn pattern(&self) -> Pattern {
        self.0.first_child()
            .map(Pattern::cast)
            .unwrap()
    }
}

impl Debug for TupleMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = self.label() {
            write!(f, "{label}: {:?}", self.pattern())
        } else {
            write!(f, "{:?}", self.pattern())
        }
    }
}

impl BindPattern {
    pub fn bind_name(&self) -> String {
        find_token(&self.0, SyntaxKind::Ident)
            .unwrap()
            .text()
            .to_string()
    }
}

impl Debug for BindPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.bind_name())
    }
}