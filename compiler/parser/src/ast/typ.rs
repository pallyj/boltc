// Type syntax 0.3
//
// Named: *ident*
// Member: **type** `.` *ident*
// Unit: `(` `)`
// Function: `func` `(` (**type**),* `)` `:` **type**
// Infer: `_`
//

use std::fmt::Debug;

use crate::lexer::SyntaxKind;

ast!(struct NamedType(NamedType));
ast!(struct MemberType(MemberType));
ast!(struct UnitType(UnitType));
ast!(struct FuncType(FuncType));
ast!(struct InferType(InferType));
ast!(struct ParenthesizedType(ParenthesizedType));
ast!(struct TupleType(TupleType));

ast!(
    enum Type {
        NamedType,
        MemberType,
        UnitType,
        FuncType,
        ParenthesizedType,
        TupleType,
        InferType,
    }
);

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NamedType(arg0) => write!(f, "{arg0:?}"),
            Self::MemberType(arg0) => write!(f, "{arg0:?}"),
            Self::UnitType(_) => write!(f, "()"),
            Self::FuncType(arg0) => write!(f, "{arg0:?}"),
            Self::ParenthesizedType(arg0) => write!(f, "{arg0:?}"),
            Self::TupleType(arg0) => write!(f, "{arg0:?}"),
            Self::InferType(_) => write!(f, "_"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl NamedType {
    pub fn name(&self) -> String {
        self.0
            .first_token()
            .map(|token| token.text().to_string())
            .unwrap()
    }
}

impl Debug for NamedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.name()) }
}

impl MemberType {
    pub fn parent(&self) -> Type { Type::cast(self.0.first_child().unwrap()) }

    pub fn child(&self) -> Option<String> { self.0.last_token().map(|token| token.text().to_string()) }
}

impl Debug for MemberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "{:?}.{}",
               self.parent(),
               self.child().unwrap_or_else(|| "".to_string()))
    }
}

impl FuncType {
    pub fn return_type(&self) -> Option<Type> {
        self.0
            .children()
            .find(|syn| syn.kind() == SyntaxKind::FuncReturn)
            .and_then(|syn| syn.children().last())
            .map(Type::cast)
    }

    pub fn params(&self) -> Vec<Type> {
        self.0
            .children()
            .find(|syn| syn.kind() == SyntaxKind::CommaSeparatedList)
            .map(|syn| syn.children().map(Type::cast).collect())
            .unwrap_or_else(Vec::new)
    }
}

impl Debug for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self.params()
                         .iter()
                         .map(|param| format!("{param:?}"))
                         .collect::<Vec<_>>()
                         .join(", ");

        match self.return_type() {
            Some(ty) => write!(f, "func ({params}): {ty:?}"),
            None => write!(f, "func ({params})"),
        }
    }
}

impl ParenthesizedType {
    pub fn typ(&self) -> Type {
        self.0
            .first_child()
            .unwrap()
            .first_child()
            .map(Type::cast)
            .unwrap()
    }
}

impl Debug for ParenthesizedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?})", self.typ())
    }
}

impl TupleType {
    pub fn types(&self) -> impl Iterator<Item = Type> {
        self.0
            .first_child()
            .unwrap()
            .children()
            .map(Type::cast)
    }
}

impl Debug for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuple_types = self
            .types()
            .map(|typ| format!("{typ:?}"))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "({tuple_types})")
    }
}