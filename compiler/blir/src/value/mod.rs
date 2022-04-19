mod constant;
mod var;
mod closure;

use std::{fmt::Debug,
          ops::{Deref, DerefMut}};

pub use constant::*;
use errors::Span;
pub use var::*;
pub use closure::*;

use crate::{code::{CodeBlock, ExternFunctionRef, FunctionRef, MethodRef},
            intrinsics::{BinaryIntrinsicFn, UnaryIntrinsicFn},
            typ::{Type, TypeKind}, Monomorphizer};

#[derive(Clone)]
pub enum ValueKind {
    // Virtual Values
    Named(String),
    Member {
        parent: Box<Value>,
        member: String,
    },
    FuncCall {
        function: Box<Value>,
        args:     FunctionArgs,
    },
    SelfVal,
    Polymorphic(Monomorphizer),
    PolymorphicMethod {
        reciever:    Box<Value>,
        polymorphic: Monomorphizer
    },
    Operator(String),

    // Literal Values
    IntLiteral(u64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    Closure(Closure),
    Uninit,
    //Deref(Box<Value>),

    // Variable Values
    Metatype(TypeKind),
    LocalVariable(String),
    FunctionParam(String),
    Assign(Box<Value>, Box<Value>),

    // Function Values
    UnaryIntrinsicFn(UnaryIntrinsicFn),
    BinaryIntrinsicFn(BinaryIntrinsicFn),
    StaticFunc(FunctionRef),
    StaticMethod(MethodRef),
    ExternFunc(ExternFunctionRef),
    InstanceMethod {
        reciever: Box<Value>,
        method:   MethodRef,
    },
    Init(Type),

    // Variable Values
    InstanceVariable {
        reciever: Box<Value>,
        var:      VarRef,
    },

    // Logic
    If(IfValue),

    // Second-class Values
    Unit,

    Error,
}

impl ValueKind {
    pub fn anon(self, typ: Type) -> Value {
        Value { kind: self,
                span: None,
                typ }
    }

    pub fn infer(self) -> Value {
        Value { kind: self,
                span: None,
                typ:  Type::infer(), }
    }

    pub fn spanned(self, typ: Type, span: Span) -> Value {
        Value { kind: self,
                span: Some(span),
                typ }
    }

    pub fn spanned_infer(self, span: Span) -> Value {
        Value { kind: self,
                span: Some(span),
                typ:  Type::infer(), }
    }
}

#[derive(Clone)]
pub struct Value {
    pub kind: ValueKind,
    pub span: Option<Span>,
    pub typ:  Type,
}

impl Deref for Value {
    type Target = ValueKind;

    fn deref(&self) -> &Self::Target { &self.kind }
}

impl DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.kind }
}

impl Value {
    pub fn set_kind(&mut self, kind: ValueKind) { self.kind = kind; }

    pub fn set_type(&mut self, typ: Type) { self.typ = typ; }
}

#[derive(Clone)]
pub struct FunctionArgs {
    pub args: Vec<Value>,
    pub labels: Vec<Option<String>>,
}

impl Debug for FunctionArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self.args
                       .iter()
                       .map(|arg| format!("{arg:?}"))
                       .collect::<Vec<_>>()
                       .join(", ");

        write!(f, "{args}")
    }
}

#[derive(Clone)]
pub struct IfValue {
    pub condition: Box<Value>,
    pub positive:  CodeBlock,
    pub negative:  Option<IfBranch>,
}

#[derive(Clone)]
pub enum IfBranch {
    CodeBlock(CodeBlock),
    Else(Box<IfValue>),
}

impl Debug for IfBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CodeBlock(arg0) => write!(f, "{arg0:?}"),
            Self::Else(arg0) => {
                if let Some(neg) = &arg0.negative {
                    write!(f,
                           "if {:?} {:?} else {:?}",
                           arg0.condition, arg0.positive, neg)
                } else {
                    write!(f, "if {:?} {:?}", arg0.condition, arg0.positive)
                }
            }
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        match self.deref() {
            ValueKind::Named(name) => write!(f, "%{name}"),
            ValueKind::Member { parent, member } => write!(f, "{parent:?}.{member}"),
            ValueKind::FuncCall { function, args } => write!(f, "{function:?}({args:?})"),
            ValueKind::SelfVal => write!(f, "self"),
            ValueKind::Polymorphic(mono) => write!(f, "function ({} degrees)", mono.degrees()),
            ValueKind::PolymorphicMethod { reciever, polymorphic } => write!(f, "{:?}.method ({})", reciever, polymorphic.degrees()),
            ValueKind::Operator(operator) => write!(f, "{operator}"),
            ValueKind::IntLiteral(i) => write!(f, "{i}"),
            ValueKind::FloatLiteral(fl) => write!(f, "{}", fl),
            ValueKind::BoolLiteral(b) => write!(f, "{b}"),
            ValueKind::Uninit => write!(f, "uninit<{:?}>", self.typ),
            ValueKind::Assign(ptr, value) => write!(f, "{ptr:?} = {value:?}"),
            //ValueKind::Deref(value) => write!(f, "*{value:?}"),
            ValueKind::Closure(c) => write!(f, "{{ {:?} }}", c.code),
            ValueKind::Metatype(t) => write!(f, "<{:?}>", t.clone().anon()),
            ValueKind::LocalVariable(name) => write!(f, "{name}"),
            ValueKind::FunctionParam(name) => write!(f, "{name}"),
            ValueKind::UnaryIntrinsicFn(intrinsic) => write!(f, "{intrinsic:?}"),
            ValueKind::BinaryIntrinsicFn(intrinsic) => write!(f, "{intrinsic:?}"),
            ValueKind::StaticFunc(func) => write!(f, "{}", func.take_name()),
            ValueKind::StaticMethod(func) => write!(f, "{}", func.borrow().info.name()),
            ValueKind::InstanceMethod { reciever, method } => write!(f, "{reciever:?}.{}", method.borrow().info.name()),
            ValueKind::ExternFunc(extern_func) => write!(f, "{}", extern_func.borrow().info.name()),
            ValueKind::Init(t) => write!(f, "{t:?}"),
            ValueKind::InstanceVariable { reciever, var } => write!(f, "{reciever:?}.{}", var.borrow().name),
            ValueKind::If(if_value) => {
                if let Some(neg) = &if_value.negative {
                    write!(f,
                           "if {:?} {:?} else {:?}",
                           if_value.condition, if_value.positive, neg)
                } else {
                    write!(f, "if {:?} {:?}", if_value.condition, if_value.positive)
                }
            }
            ValueKind::Unit => write!(f, "()"),
            ValueKind::Error => write!(f, "Error"),
        }?;

        write!(f, " : {:?})", self.typ)
    }
}


impl Default for Value {
    fn default() -> Self {
        ValueKind::Unit.infer()
    }
}