use std::{fmt::Debug,
          ops::{Deref, DerefMut}};

use errors::Span;

use crate::{typ::{Type, TypeKind},
            value::Value, pattern::Pattern};

use super::CodeBlock;

#[derive(Clone)]
pub enum StatementKind {
    Eval {
        value:   Value,
        escaped: bool,
    },

    Bind {
        pattern: Pattern,
        typ:     Type,
        value:   Option<Value>,
    },

    Return {
        value: Option<Value>,
    },

    Break(Option<Value>, String),
    Continue(String),

    Guard {
        condition: Box<Value>,
        otherwise: CodeBlock
    },

    GuardLet {
        pattern: Pattern,
        value: Value,
        otherwise: CodeBlock,
    },

    Panic
}

impl StatementKind {
    pub fn anon(self) -> Statement { Statement { kind: self, span: None } }

    pub fn spanned(self, span: Span) -> Statement {
        Statement { kind: self,
                    span: Some(span), }
    }
}

#[derive(Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Option<Span>,
}

impl Statement {
    pub fn set_kind(&mut self, kind: StatementKind) { self.kind = kind; }

    pub fn span(&self) -> Option<&Span> { self.span.as_ref() }

    pub fn typ(&self) -> Type {
        match self.deref() {
            StatementKind::Bind { .. } => TypeKind::Void.anon(),
            StatementKind::Return { .. } => TypeKind::Divergent.anon(),
            StatementKind::Eval { value, escaped } => {
                let ty = value.typ.clone();

                if let TypeKind::Divergent = ty.deref() {
                    return ty;
                }

                if *escaped {
                    TypeKind::Void.anon()
                } else {
                    ty
                }
            }
            StatementKind::Break(_, _) => TypeKind::Divergent.anon(),
            StatementKind::Continue(_) => TypeKind::Divergent.anon(),

            StatementKind::Guard { .. } => {
                TypeKind::Void.anon()
            }

            StatementKind::GuardLet { .. } => {
                TypeKind::Void.anon()
            },

            StatementKind::Panic => TypeKind::Divergent.anon(),
        }
    }

    pub fn diverges(&self) -> bool {
        match self.deref() {
            StatementKind::Return { .. } => true,
            StatementKind::Eval { value, .. } => matches!(value.typ.deref(), TypeKind::Divergent),
            StatementKind::Break(_, _) => true,
            StatementKind::Continue(_) => true,
            StatementKind::Panic => true,
            _ => false,
        }
    }
}

impl Deref for Statement {
    type Target = StatementKind;

    fn deref(&self) -> &Self::Target { &self.kind }
}

impl DerefMut for Statement {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.kind }
}

impl Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.deref() {
            StatementKind::Eval { value, escaped } => {
                if *escaped {
                    write!(f, "{value:?};")
                } else {
                    write!(f, "{value:?}")
                }
            }
            StatementKind::Bind { pattern, typ, value } => {
                if let Some(value) = value {
                    write!(f, "let {pattern:?}: {typ:?} = {value:?}")
                } else {
                    write!(f, "let {pattern:?}: {typ:?}")
                }
            }
            StatementKind::Return { value } => {
                if let Some(value) = value {
                    write!(f, "return {value:?}")
                } else {
                    write!(f, "return")
                }
            }
            StatementKind::Break(value, label) => if let Some(value) = value {
                write!(f, "break {value:?} `{label}")
            } else {
                write!(f, "break `{label}")
            },
            StatementKind::Continue(label) => write!(f, "continue `{label}"),

            StatementKind::Guard { condition, otherwise } => {
                write!(f, "guard {condition:?} else {otherwise:?}")
            }

            StatementKind::GuardLet { pattern, value, otherwise } => {
                write!(f, "guard let {pattern:?} = {value:?} else {otherwise:?}")
            }

            StatementKind::Panic => write!(f, "panic"),
        }
    }
}
