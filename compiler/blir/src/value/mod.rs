mod closure;
mod constant;
mod var;
pub mod match_;

use std::{fmt::Debug,
          ops::{Deref, DerefMut}, collections::HashMap};

pub use closure::*;
pub use constant::*;
use errors::Span;
use itertools::Itertools;
pub use var::*;
pub use match_::*;

use crate::{code::{CodeBlock, ExternFunctionRef, FunctionRef, MethodRef},
            intrinsics::{BinaryIntrinsicFn, UnaryIntrinsicFn},
            typ::{Type, TypeKind, CaseRef, EnumRef},
            Monomorphizer, attributes::AttributeArgs};

use self::match_::MatchValue;

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
    SelfVal(bool),
    Polymorphic(Monomorphizer),
    PolymorphicMethod {
        reciever:    Box<Value>,
        polymorphic: Monomorphizer,
    },
    Operator(String),

    // Literal Values
    IntLiteral(u64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),
    VariantLiteral(String),
    Closure(Closure),
    Uninit,
    Tuple(Vec<Value>),
    // Deref(Box<Value>),

    EnumVariant {
        of_enum: EnumRef,
        variant: CaseRef,
    },
    CastEnumToVariant {
        enum_value: Box<Value>,
        variant: CaseRef,
    },
    SequenceLiteral(Vec<Value>),
    RepeatingLiteral {
        repeating: Box<Value>,
        count: Option<u64>,
    },


    // Variable Values
    Metatype(TypeKind),
    LocalVariable(String, bool),
    FunctionParam(String, bool),
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
    Initializer(MethodRef, Type),

    // Variable Values
    InstanceVariable {
        reciever: Box<Value>,
        var:      VarRef,
    },
    TupleField(Box<Value>, usize),

    // Logic
    If(IfValue),
    Match(MatchValue),
    Loop {
        label: String,
        code: CodeBlock
    },

    // Second-class Values
    Unit,

    // Generic
    MonomorphizeFn {
        function: Box<Value>,
        generics: HashMap<String, Type>,
    },

    Macro(String, AttributeArgs),

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

    pub fn spanned(self, mut typ: Type, span: Span) -> Value {
        typ.span = Some(span);
        Value { kind: self,
                span: Some(span),
                typ }
    }

    pub fn spanned_infer(self, span: Span) -> Value {
        let infer = Type::infer().kind;
        Value { kind: self,
                span: Some(span),
                typ:  infer.spanned(span), }
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

    pub fn set_type(&mut self, mut typ: Type) { typ.span = self.span; self.typ = typ; }

    pub fn monomorph_infer(self, args: Vec<String>) -> Value {
        let mut monomorph_params = HashMap::new();

        for arg in args {
            monomorph_params.insert(arg, Type::infer());
        }

        let passthrough_type = self.typ.clone();
        let passthrough_span = self.span.clone();


        let mut val = ValueKind::MonomorphizeFn { function: Box::new(self), generics: monomorph_params }.anon(passthrough_type);
        val.span = passthrough_span;
        val
    }

    pub fn is_mutable(&self) -> bool {
        match &self.kind {
            ValueKind::FuncCall { function, args } => {
                match &function.kind {
                    ValueKind::BinaryIntrinsicFn(BinaryIntrinsicFn::ArrayItem) => {
                        args.args[0].is_mutable()
                    }
                    _ => false
                }
            }, // todo: maybe
            ValueKind::SelfVal(mutating) => *mutating, // I Think?

            ValueKind::CastEnumToVariant { enum_value, .. } => enum_value.is_mutable(),
            ValueKind::InstanceVariable { reciever, var } => reciever.is_mutable() && !var.borrow().is_constant,
            ValueKind::TupleField(tuple, _) => tuple.is_mutable(),

            ValueKind::LocalVariable(_, mutating) => *mutating,
            ValueKind::FunctionParam(_, mutating) => *mutating,

            ValueKind::Named(_) => false,
            ValueKind::Member { .. } => false,
            
            ValueKind::Polymorphic(_) => false,
            ValueKind::PolymorphicMethod { .. } => false,
            ValueKind::Operator(_) => false,
            ValueKind::IntLiteral(_) => false,
            ValueKind::FloatLiteral(_) => false,
            ValueKind::BoolLiteral(_) => false,
            ValueKind::StringLiteral(_) => false,
            ValueKind::VariantLiteral(_) => false,
            ValueKind::Closure(_) => false,
            ValueKind::Uninit => false,
            ValueKind::Tuple(_) => false,
            ValueKind::EnumVariant { .. } => false,
            
            ValueKind::Metatype(_) => false,
            
            ValueKind::Assign(_, _) => false,
            ValueKind::UnaryIntrinsicFn(_) => false,
            ValueKind::BinaryIntrinsicFn(_) => false,
            ValueKind::StaticFunc(_) => false,
            ValueKind::StaticMethod(_) => false,
            ValueKind::ExternFunc(_) => false,
            ValueKind::InstanceMethod { .. } => false,
            ValueKind::Init(_) => false,
            ValueKind::Initializer(_, _) => false,
            ValueKind::If(_) => false,
            ValueKind::Match(_) => false,
            ValueKind::Loop { .. } => false,
            ValueKind::Unit => false,
            ValueKind::MonomorphizeFn { .. } => false,

            ValueKind::SequenceLiteral(_) => false,
            ValueKind::RepeatingLiteral { .. } => false,

            ValueKind::Macro(_, _) => false,

            ValueKind::Error => false,
        }
    }
}

#[derive(Clone)]
pub struct FunctionArgs {
    pub args:   Vec<Value>,
    pub labels: Vec<Option<String>>,
    pub is_shared: Vec<bool>
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
    ElseLet(Box<MatchValue>),
}

impl Debug for IfBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CodeBlock(arg0) => write!(f, "{arg0:?}"),
            Self::ElseLet(arg0) => write!(f, "{arg0:?}"),
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
        match self.deref() {
            ValueKind::MonomorphizeFn { function, generics } => {
                write!(f, "{function:?}<{}>", generics.iter().map(|(key, ty)| format!("{key} = {ty:?}")).collect::<Vec<_>>().join(","))
            }
            ValueKind::Named(name) => write!(f, "%{name}"),
            ValueKind::Member { parent, member } => write!(f, "{parent:?}.{member}"),
            ValueKind::FuncCall { function, args } => write!(f, "{function:?}({args:?})"),
            ValueKind::SelfVal(_) => write!(f, "self"),
            ValueKind::Polymorphic(mono) => write!(f, "function ({} degrees)", mono.degrees()),
            ValueKind::PolymorphicMethod { reciever, polymorphic } => write!(f, "{:?}.method ({})", reciever, polymorphic.degrees()),
            ValueKind::Operator(operator) => write!(f, "{operator}"),
            ValueKind::IntLiteral(i) => write!(f, "{i}"),
            ValueKind::FloatLiteral(fl) => write!(f, "{}", fl),
            ValueKind::StringLiteral(string) => write!(f, r#""{string}""#),
            ValueKind::BoolLiteral(b) => write!(f, "{b}"),
            ValueKind::VariantLiteral(name) => write!(f, ".{name}"),
            ValueKind::EnumVariant { of_enum, variant } => write!(f, "{}.{}", of_enum.name(), variant.name()),
            ValueKind::CastEnumToVariant { enum_value, variant } => write!(f, "{enum_value:?} as {}", variant.name()),
            ValueKind::Uninit => write!(f, "uninit"),
            ValueKind::Assign(ptr, value) => write!(f, "{ptr:?} = {value:?}"),
            // ValueKind::Deref(value) => write!(f, "*{value:?}"),
            ValueKind::Closure(c) => write!(f, "{{ {:?} }}", c.code),
            ValueKind::Metatype(t) => write!(f, "<{:?}>", t.clone().anon()),
            ValueKind::LocalVariable(name, varying) => if *varying {
                write!(f, "var {name}")
            } else {
                write!(f, "{name}")
            },
            ValueKind::FunctionParam(name, _) => write!(f, "{name}"),
            ValueKind::UnaryIntrinsicFn(intrinsic) => write!(f, "{intrinsic:?}"),
            ValueKind::BinaryIntrinsicFn(intrinsic) => write!(f, "{intrinsic:?}"),
            ValueKind::StaticFunc(func) => write!(f, "{}", func.take_name()),
            ValueKind::StaticMethod(func) => write!(f, "{}", func.borrow().info.name()),
            ValueKind::InstanceMethod { reciever, method } => write!(f, "{reciever:?}.{}", method.borrow().info.name()),
            ValueKind::ExternFunc(extern_func) => write!(f, "{}", extern_func.borrow().info.name()),
            ValueKind::Init(t) => write!(f, "{t:?}"),
            ValueKind::Initializer(method, _) => write!(f, "{}", method.name()),
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
            ValueKind::Match(match_value) => {
                write!(f, "{match_value:?}")
            }
            ValueKind::Tuple(items) => {
                let tuple_items = items.iter().map(|item| format!("{item:?}")).collect::<Vec<_>>().join(", ");

                write!(f, "({tuple_items})")
            }
            ValueKind::TupleField(value, n) => write!(f, "{value:?}.item{n}"),
            ValueKind::SequenceLiteral(sequence) => write!(f, "[{:?}]", sequence.iter().format(", ")),
            ValueKind::RepeatingLiteral { repeating, count } => if let Some(count) = count {
                write!(f, "[repeating: {repeating:?}, count: {count}]")
            } else {
                write!(f, "[repeating: {repeating:?}]")
            },
            ValueKind::Unit => write!(f, "()"),
            ValueKind::Error => write!(f, "Error"),
            ValueKind::Loop { code: code_block, label } => write!(f, "loop {code_block:?} `{label}"),

            ValueKind::Macro(name, args) => write!(f, "@{name}()"),
        }?;

        write!(f, " <{:?}>", self.typ)
    }
}

impl Default for Value {
    fn default() -> Self { ValueKind::Unit.infer() }
}
