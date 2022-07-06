// Statements 0.3
//
// Eval: **expr** (`;`)?
// Return: `return` (**expr**)?
// Bind: `let` *ident* (`:` **type**) `=` **expr**
// NoOp: `;`
//

use std::fmt::Debug;

use super::{expr::Expr, typ::Type, pattern::Pattern, find_token};
use crate::lexer::SyntaxKind;

ast!(struct EvalSmt(EvalSmt));
ast!(struct ReturnSmt(ReturnSmt));
ast!(struct LetSmt(LetSmt));
ast!(struct NoOp(NoOp));
ast!(struct BreakSmt(BreakSmt));
ast!(struct ContinueSmt(ContinueSmt));
ast!(struct GuardSmt(Guard));
ast!(struct GuardLetSmt(GuardLet));

ast!(
    enum Smt {
        EvalSmt,
        ReturnSmt,
        LetSmt,
        NoOp,
        BreakSmt,
        ContinueSmt,
        GuardSmt,
        GuardLetSmt,
    }
);

impl Debug for Smt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EvalSmt(arg0) => write!(f, "{arg0:?}"),
            Self::ReturnSmt(arg0) => write!(f, "{arg0:?}"),
            Self::LetSmt(arg0) => write!(f, "{arg0:?}"),
            Self::NoOp(_) => write!(f, ";"),
            Self::BreakSmt(arg0) => write!(f, "{arg0:?}"),
            Self::ContinueSmt(arg0) => write!(f, "{arg0:?}"),
            Self::GuardSmt(arg0) => write!(f, "{arg0:?}"),
            Self::GuardLetSmt(arg0) => write!(f, "{arg0:?}"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl EvalSmt {
    pub fn value(&self) -> Expr { self.0.first_child().map(Expr::cast).unwrap() }

    pub fn is_escaped(&self) -> bool {
        self.0
            .children_with_tokens()
            .any(|tok| tok.kind() == SyntaxKind::Semicolon)
    }
}

impl Debug for EvalSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_escaped() {
            write!(f, "{:?};", self.value())
        } else {
            write!(f, "{:?}", self.value())
        }
    }
}

impl ReturnSmt {
    pub fn return_value(&self) -> Option<Expr> { self.0.first_child().map(Expr::cast) }
}

impl Debug for ReturnSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(return_value) = self.return_value() {
            write!(f, "return {return_value:?}")
        } else {
            write!(f, "return")
        }
    }
}

impl LetSmt {
    pub fn pattern(&self) -> Pattern {
        self.0
            .first_child()
            .map(Pattern::cast)
            .unwrap()
    }

    pub fn typ(&self) -> Option<Type> {
        self.0
            .children()
            .find(|element| element.kind() == SyntaxKind::BindType)
            .and_then(|element| element.first_child())
            .map(Type::cast)
    }

    pub fn value(&self) -> Option<Expr> {
        self.0
            .children()
            .find(|element| element.kind() == SyntaxKind::AssignValue)
            .and_then(|element| element.first_child())
            .map(Expr::cast)
    }
}


impl GuardSmt {
    pub fn condition(&self) -> Expr {
        self.0
            .children()
            .find(|syn| syn.kind() == SyntaxKind::Condition)
            .and_then(|condition| condition.first_child())
            .map(Expr::cast)
            .unwrap()
    }

    pub fn else_block(&self) -> CodeBlock {
        self.0
            .last_child()
            .and_then(CodeBlock::cast)
            .unwrap()
    }
}

impl GuardLetSmt {
    pub fn pattern(&self) -> Pattern {
        self.0
            .first_child()
            .map(Pattern::cast)
            .unwrap()
    }
    pub fn value(&self) -> Expr {
        self.0
            .children()
            .find(|syn| syn.kind() == SyntaxKind::Condition)
            .and_then(|condition| condition.first_child())
            .map(Expr::cast)
            .unwrap()
    }

    pub fn else_block(&self) -> CodeBlock {
        self.0
            .last_child()
            .and_then(CodeBlock::cast)
            .unwrap()
    }
}

impl Debug for LetSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pattern = self.pattern();
        let typ = self.typ()
                      .map(|typ| format!(": {typ:?}"))
                      .unwrap_or_else(|| "".to_string());
        let value = self.value()
                        .map(|value| format!(" = {value:?}"))
                        .unwrap_or_else(|| "".to_string());

        write!(f, "let {pattern:?}{typ}{value}")
    }
}

ast!(struct CodeBlock(CodeBlock));

impl CodeBlock {
    pub fn statements(&self) -> Vec<Smt> { self.0.children().map(Smt::cast).collect() }
}

impl Debug for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let statements = self.statements()
                             .iter()
                             .map(|smt| format!("\t{smt:?}").replace("\n", "\n\t"))
                             .collect::<Vec<_>>()
                             .join("\n");

        write!(f, "{{\n{statements}\n}}")
    }
}

impl BreakSmt {
    pub fn value(&self) -> Option<Expr> {
        self.0
            .first_child()
            .map(Expr::cast)
    }
    pub fn scope(&self) -> Option<String> {
        find_token(&self.0, SyntaxKind::Scope).map(|t| t.text().to_string())
    }
}

impl Debug for BreakSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(scope_label) = self.scope() {
            write!(f, "break {scope_label}")
        } else {
            write!(f, "break")
        }
    }
}

impl ContinueSmt {
    pub fn scope(&self) -> Option<String> {
        find_token(&self.0, SyntaxKind::Scope).map(|t| t.text().to_string())
    }
}

impl Debug for ContinueSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(scope_label) = self.scope() {
            write!(f, "continue {scope_label}")
        } else {
            write!(f, "continue")
        }
    }
}

impl Debug for GuardSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "guard {:?} else {:?}", self.condition(), self.else_block())
    }
}

impl Debug for GuardLetSmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "guard let {:?} = {:?} else {:?}", self.pattern(), self.value(), self.else_block())
    }
}