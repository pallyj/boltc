// Expr syntax 0.5
//
// Integer literal: *int_lit*
// Float literal: *float_lit*
// String literal: "text"
// Boolean literal: true | false
// Variant literal: `.` *ident*
// Named: *ident*
//
// Member: **expr** `.` *ident*
// Function Call: **expr** `(` (**function arg**),* `)`
//
// Tuple: `(` **expr**, + `)`
//
// Closure: `{` (**closure_param_list** `=>`) **closure_body** `}`
//
// Paren: `(` **expr** `)`
//
// PrefixOp: *op* **expr**
// PostfixOp: **expr** *op*
// InfixOp: **expr** *op* **expr**
//
// If: `if` **expr** **codeblock**
// (`else` **codeblock** | **if_smt**)?
//
// Index: **expr** `[` **expr** `]`
//

use std::fmt::Debug;

use super::{find_token, smt::CodeBlock, typ::Type, pattern::Pattern};
use crate::lexer::SyntaxKind;

ast!(struct NamedExpr(NamedExpr));
ast!(struct LiteralExpr(Literal));
ast!(struct ParenthesizedExpr(ParenthesizedExpr));
ast!(struct IfExpr(IfExpr));
ast!(struct MemberExpr(MemberExpr));
ast!(struct FuncCallExpr(FuncCallExpr));
ast!(struct UnitExpr(UnitExpr));
ast!(struct PrefixExpr(PrefixExpr));
ast!(struct PostfixExpr(PostfixExpr));
ast!(struct InfixExpr(InfixExpr));
ast!(struct ClosureExpr(Closure));
ast!(struct TrailingClosureExpr(TrailingClosure));
ast!(struct TupleExpr(Tuple));
ast!(struct TupleMember(FuncArg));
ast!(struct IndexExpr(IndexExpr));
ast!(struct VariantLiteral(VariantLiteral));
ast!(struct MatchExpr(MatchExpr));


ast!(struct MatchBranch(MatchBranch));
ast!(struct FuncArg(FuncArg));
ast!(struct ClosureParam(FuncPar));

ast!(
    enum Expr {
        NamedExpr,
        LiteralExpr,
        ParenthesizedExpr,
        IfExpr,
        MemberExpr,
        FuncCallExpr,
        UnitExpr,
        PrefixExpr,
        PostfixExpr,
        InfixExpr,
        ClosureExpr,
        TrailingClosureExpr,
        TupleExpr,
        IndexExpr,
        VariantLiteral,
        MatchExpr,
    }
);

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NamedExpr(arg0) => write!(f, "{arg0:?}"),
            Self::LiteralExpr(arg0) => write!(f, "{arg0:?}"),
            Self::ParenthesizedExpr(arg0) => write!(f, "{arg0:?}"),
            Self::IfExpr(arg0) => write!(f, "{arg0:?}"),
            Self::MemberExpr(arg0) => write!(f, "{arg0:?}"),
            Self::FuncCallExpr(arg0) => write!(f, "{arg0:?}"),
            Self::UnitExpr(arg0) => write!(f, "{arg0:?}"),
            Self::PrefixExpr(arg0) => write!(f, "{arg0:?}"),
            Self::PostfixExpr(arg0) => write!(f, "{arg0:?}"),
            Self::InfixExpr(arg0) => write!(f, "{arg0:?}"),
            Self::ClosureExpr(arg0) => write!(f, "{arg0:?}"),
            Self::TrailingClosureExpr(arg0) => write!(f, "{arg0:?}"),
            Self::TupleExpr(arg0) => write!(f, "{arg0:?}"),
            Self::IndexExpr(arg0) => write!(f, "{arg0:?}"),
            Self::VariantLiteral(arg0) => write!(f, "{arg0:?}"),
            Self::MatchExpr(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "Error"),
        }
    }
}

ast!(
    enum IfExprNegative {
        IfExpr,
        CodeBlock,
    }
);

impl Debug for IfExprNegative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IfExpr(arg0) => write!(f, "{arg0:?}"),
            Self::CodeBlock(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl NamedExpr {
    pub fn name(&self) -> String {
        self.0
            .first_token()
            .map(|token| token.text().to_string())
            .unwrap()
    }
}

impl Debug for NamedExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.name()) }
}

pub enum LiteralKind {
    True,
    False,
    DecInteger,
    HexInteger,
    OctInteger,
    BinInteger,
    DecFloat,
    String,
    LongString,
    Error,
}

impl LiteralExpr {
    pub fn text(&self) -> String {
        self.0
            .first_token()
            .map(|token| token.text().to_string())
            .unwrap()
    }

    pub fn literal_kind(&self) -> LiteralKind {
        self.0
            .first_token()
            .map(|tok| match tok.kind() {
                SyntaxKind::LiteralBinInt => LiteralKind::BinInteger,
                SyntaxKind::LiteralOctInt => LiteralKind::OctInteger,
                SyntaxKind::LiteralHexInt => LiteralKind::HexInteger,
                SyntaxKind::LiteralDecInt => LiteralKind::DecInteger,
                SyntaxKind::LiteralDecFloat => LiteralKind::DecFloat,
                SyntaxKind::LiteralFalse => LiteralKind::False,
                SyntaxKind::LiteralTrue => LiteralKind::True,
                SyntaxKind::StringLiteral => LiteralKind::String,
                SyntaxKind::LongStringLiteral => LiteralKind::LongString,
                _ => LiteralKind::Error,
            })
            .unwrap_or(LiteralKind::Error)
    }
}

impl Debug for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.text()) }
}

impl ParenthesizedExpr {
    pub fn expr(&self) -> Expr { self.0.first_child().map(Expr::cast).unwrap_or(Expr::Error) }
}

impl Debug for ParenthesizedExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({:?})", self.expr()) }
}

impl MemberExpr {
    pub fn parent(&self) -> Expr { Expr::cast(self.0.first_child().unwrap()) }

    pub fn child(&self) -> Option<String> { find_token(&self.0, SyntaxKind::Ident).map(|token| token.text().to_string()) }
}

impl Debug for MemberExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "{:?}.{}",
               self.parent(),
               self.child().unwrap_or_else(|| "".to_string()))
    }
}

impl IfExpr {
    pub fn condition(&self) -> Expr {
        self.0
            .children()
            .find(|syn| syn.kind() == SyntaxKind::Condition)
            .and_then(|condition| condition.first_child())
            .map(Expr::cast)
            .unwrap()
    }

    pub fn positive(&self) -> CodeBlock {
        self.0
            .children()
            .find(|syn| syn.kind() == SyntaxKind::Positive)
            .and_then(|positive| positive.first_child())
            .and_then(CodeBlock::cast)
            .unwrap()
    }

    pub fn negative(&self) -> Option<IfExprNegative> {
        self.0
            .children()
            .find(|syn| syn.kind() == SyntaxKind::Negative)
            .and_then(|positive| positive.first_child())
            .map(IfExprNegative::cast)
    }
}

impl FuncCallExpr {
    pub fn function(&self) -> Expr { self.0.first_child().map(Expr::cast).unwrap() }

    pub fn args(&self) -> impl Iterator<Item = FuncArg> {
        self.0
            .children()
            .find(|node| node.kind() == SyntaxKind::CommaSeparatedList)
            .unwrap()
            .children()
            .filter_map(FuncArg::cast)
    }
}

impl FuncArg {
    pub fn label(&self) -> Option<String> { find_token(&self.0, SyntaxKind::Ident).map(|arg_label| arg_label.text().to_string()) }

    pub fn value(&self) -> Expr { self.0.last_child().map(Expr::cast).unwrap() }
}

impl PrefixExpr {
    pub fn operator(&self) -> String {
        find_token(&self.0, SyntaxKind::Operator).map(|op| op.text().to_string())
                                                 .unwrap()
    }

    pub fn unit(&self) -> Expr { self.0.first_child().map(Expr::cast).unwrap() }
}

impl PostfixExpr {
    pub fn operator(&self) -> String {
        find_token(&self.0, SyntaxKind::Operator).map(|op| op.text().to_string())
                                                 .unwrap()
    }

    pub fn unit(&self) -> Expr { self.0.first_child().map(Expr::cast).unwrap() }
}

impl InfixExpr {
    pub fn operator(&self) -> String {
        find_token(&self.0, SyntaxKind::Operator).map(|op| op.text().to_string())
                                                 .unwrap()
    }

    pub fn left(&self) -> Expr { self.0.first_child().map(Expr::cast).unwrap() }

    pub fn right(&self) -> Expr { self.0.last_child().map(Expr::cast).unwrap() }
}

impl ClosureParam {
    pub fn bind_name(&self) -> String {
        find_token(&self.0, SyntaxKind::Ident).unwrap()
                                              .text()
                                              .to_string()
    }

    pub fn explicit_type(&self) -> Option<Type> { self.0.last_child().map(Type::cast) }
}

impl Debug for ClosureParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(explicit_type) = self.explicit_type() {
            write!(f, "{}: {:?}", self.bind_name(), explicit_type)
        } else {
            write!(f, "{}", self.bind_name())
        }
    }
}

impl ClosureExpr {
    pub fn parameters(&self) -> Option<impl Iterator<Item = ClosureParam>> {
        self.0
            .children()
            .find(|node| node.kind() == SyntaxKind::CommaSeparatedList)
            .map(|params| params.children().filter_map(ClosureParam::cast))
    }

    pub fn code_block(&self) -> CodeBlock { self.0.children().find_map(CodeBlock::cast).unwrap() }
}

impl Debug for ClosureExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(parameters) = self.parameters() {
            let parameters = parameters.map(|d| format!("{d:?}"))
                                       .collect::<Vec<_>>()
                                       .join(", ");

            write!(f, "{{ {parameters} => {:?} }}", self.code_block())
        } else {
            write!(f, "{:?}", self.code_block())
        }
    }
}

impl TrailingClosureExpr {
    pub fn function(&self) -> Expr { self.0.first_child().map(Expr::cast).unwrap() }

    pub fn closure(&self) -> ClosureExpr { self.0.last_child().and_then(ClosureExpr::cast).unwrap() }
}

impl Debug for TrailingClosureExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?} {:?}", self.function(), self.closure()) }
}

impl Debug for FuncCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self.args()
                       .map(|arg| format!("{arg:?}"))
                       .collect::<Vec<_>>()
                       .join(", ");

        write!(f, "{:?}({})", self.function(), args)
    }
}

impl Debug for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(negative) = self.negative() {
            write!(f,
                   "if {:?} {:?} {:?}",
                   self.condition(),
                   self.positive(),
                   negative)
        } else {
            write!(f, "if {:?} {:?}", self.condition(), self.positive())
        }
    }
}

impl Debug for UnitExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "()") }
}

impl Debug for FuncArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = self.label() {
            write!(f, "{label}: ")?;
        }
        write!(f, "{:?}", self.value())
    }
}

impl Debug for PrefixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({} {:?})", self.operator(), self.unit()) }
}

impl Debug for PostfixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({:?} {})", self.unit(), self.operator()) }
}

impl Debug for InfixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "({:?} {} {:?})",
               self.left(),
               self.operator(),
               self.right())
    }
}


impl TupleExpr {
    pub fn items(&self) -> impl Iterator<Item = TupleMember> {
        self.0
            .first_child()
            .unwrap()
            .children()
            .filter_map(TupleMember::cast)
    }
}

impl Debug for TupleExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuple_items = self
            .items()
            .map(|typ| format!("{typ:?}"))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "({tuple_items})")
    }
}

impl TupleMember {
    pub fn label(&self) -> Option<String> {
        find_token(&self.0, SyntaxKind::Ident)
            .map(|token| token.text().to_string())
    }

    pub fn expr(&self) -> Expr {
        self.0.first_child()
            .map(Expr::cast)
            .unwrap()
    }
}

impl Debug for TupleMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = self.label() {
            write!(f, "{label}: {:?}", self.expr())
        } else {
            write!(f, "{:?}", self.expr())
        }
    }
}

impl IndexExpr {
    pub fn parent(&self) -> Expr { self.0.first_child().map(Expr::cast).unwrap() }

    pub fn index(&self) -> FuncArg { self.0.last_child().and_then(FuncArg::cast).unwrap() }
}

impl Debug for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[{:?}]", self.parent(), self.index())
    }
}

impl VariantLiteral {
    pub fn variant_name(&self) -> String { find_token(&self.0, SyntaxKind::Ident).unwrap().text().to_string() }
}

impl Debug for VariantLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".{}", self.variant_name())
    }
}

impl MatchExpr {
    pub fn discriminant(&self) -> Expr {
        self.0
            .children()
            .find(|syn| syn.kind() == SyntaxKind::Condition)
            .and_then(|condition| condition.first_child())
            .map(Expr::cast)
            .unwrap()
    }

    pub fn branches(&self) -> impl Iterator<Item=MatchBranch> {
        self.0
            .children()
            .filter_map(MatchBranch::cast)
    }
}

impl Debug for MatchExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "match {:?} {{", self.discriminant())?;

        for branch in self.branches() {
            writeln!(f, "{branch:?}")?;
        }

        write!(f, "}}")
    }
}

impl MatchBranch {
    pub fn pattern(&self) -> Pattern {
        self.0
            .first_child()
            .map(Pattern::cast)
            .unwrap()
    }

    pub fn code_block(&self) -> Option<CodeBlock> {
        self.0
            .last_child()
            .and_then(CodeBlock::cast)
    }

    pub fn value(&self) -> Option<Expr> {
        self.0
            .last_child()
            .map(Expr::cast)
    }
}

impl Debug for MatchBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(code_block) = self.code_block() {
            write!(f, "{:?} => {code_block:?}", self.pattern())
        } else {
            write!(f, "{:?} => {:?}", self.pattern(), self.value().unwrap())
        }
    }
}