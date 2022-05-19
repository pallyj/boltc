mod extern_func;
mod func;
mod method;
mod smt;

use std::fmt::Debug;

use errors::Span;
pub use extern_func::*;
pub use func::*;
pub use method::*;
pub use smt::*;

use crate::typ::{Type, TypeKind};

#[derive(Clone)]
pub struct CodeBlock {
    statements: Vec<Statement>,
    span:       Option<Span>,
}

impl CodeBlock {
    pub fn new(statements: Vec<Statement>, span: Span) -> CodeBlock {
        CodeBlock { statements,
                    span: Some(span) }
    }

    pub fn anon(statements: Vec<Statement>) -> CodeBlock { CodeBlock { statements, span: None } }

    pub fn span(&self) -> Option<&Span> { self.span.as_ref() }

    pub fn statements(&self) -> &Vec<Statement> { &self.statements }

    pub fn statements_mut(&mut self) -> &mut Vec<Statement> { &mut self.statements }

    pub fn typ(&self) -> Type {
        for smt in self.statements.iter() {
            if smt.diverges() {
                return TypeKind::Divergent.anon();
            }
        }

        if let Some(smt) = self.statements().last() {
            return smt.typ();
        }

        TypeKind::Void.anon()
    }

    pub fn escapes(&self) -> bool {
        for smt in self.statements.iter() {
            if smt.diverges() {
                return true
            }
        }

        return false
    }
}

impl Debug for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.statements()
                       .iter()
                       .map(|smt| format!("{smt:?}").replace("\n", "\n\t"))
                       .collect::<Vec<_>>()
                       .join("\n\t");

        write!(f, "{{\n\t{code}\n}}")
    }
}
