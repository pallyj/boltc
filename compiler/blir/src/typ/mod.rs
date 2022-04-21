mod struct_;

use std::{fmt::Debug,
          hash::Hash,
          ops::{Deref, DerefMut},
          sync::atomic::{AtomicU64, Ordering}};

use errors::Span;
use mangle::MangledType;
pub use struct_::*;

use crate::{scope::ScopeRef, Symbol};

static NEXT_INFER_KEY: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeKind {
    // Virtual types
    /// A named type. This type is created by the parser.
    Named(String),

    /// A member type. This type is created by the parser
    Member {
        parent: Box<Type>,
        member: String,
    },

    /// Signifies the type must be inferred. This type is created by the parser
    Infer {
        key: u64,
    },

    // First-class types
    Void,
    Function {
        return_type: Box<Type>,
        params:      Vec<Type>,
        labels:      Vec<Option<String>>,
    },

    SomeInteger,
    SomeFloat,
    SomeBool,
    SomeFunction,

    Method {
        reciever:    Box<Type>,
        return_type: Box<Type>,
        params:      Vec<Type>,
    },
    Struct(StructRef),

    // Intrinsic types
    Integer {
        bits: u64,
    },
    Float {
        bits: u64,
    },

    // Second-class types
    Divergent,
    Metatype(Box<Type>),

    Error,
}

impl TypeKind {
    pub fn anon(self) -> Type { Type { kind: self, span: None } }

    pub fn spanned(self, span: Span) -> Type {
        Type { kind: self,
               span: Some(span), }
    }
}

#[derive(Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Option<Span>,
}

impl Type {
    pub fn set_kind(&mut self, kind: TypeKind) { self.kind = kind; }

    pub fn kind(&self) -> &TypeKind { &self.kind }

    pub fn kind_mut(&mut self) -> &mut TypeKind { &mut self.kind }

    pub fn span(&self) -> Option<Span> { self.span }

    pub fn infer_specific(span: Span) -> Type {
        let key = NEXT_INFER_KEY.fetch_add(1, Ordering::AcqRel);

        Type { kind: TypeKind::Infer { key },
               span: Some(span), }
    }

    pub fn infer() -> Type {
        let key = NEXT_INFER_KEY.fetch_add(1, Ordering::AcqRel);

        Type { kind: TypeKind::Infer { key },
               span: None, }
    }

    pub fn lookup_static_item(&self, named: &str) -> Option<Symbol> {
        match &self.kind {
            TypeKind::Struct(r#struct) => r#struct.lookup_static_item(named),
            _ => None,
        }
    }

    pub fn lookup_instance_item(&self, named: &str, scope: &ScopeRef) -> Option<Symbol> {
        match &self.kind {
            TypeKind::Metatype(ty) => ty.clone().lookup_static_item(named),
            TypeKind::Struct(r#struct) => r#struct.lookup_instance_item(named, scope),
            _ => None,
        }
    }

    pub fn init_type(&self) -> TypeKind {
        match &self.kind {
            TypeKind::Struct(r#struct) => {
                let params = r#struct.params();

                TypeKind::Function { return_type: Box::new(self.clone()),
                                     params,
                                     labels: vec![] }
            }

            _ => TypeKind::Function { return_type: Box::new(self.clone()),
                                      params:      vec![self.clone()],
                                      labels:      vec![], },
        }
    }

    pub fn mangle(&self) -> MangledType {
        match self.kind() {
            TypeKind::Named(_) => MangledType::Void,
            TypeKind::Member { .. } => panic!(),
            TypeKind::Infer { .. } => panic!(),

            TypeKind::Integer { bits: 1 } => MangledType::Integer1,
            TypeKind::Integer { bits: 8 } => MangledType::Integer8,
            TypeKind::Integer { bits: 16 } => MangledType::Integer16,
            TypeKind::Integer { bits: 32 } => MangledType::Integer32,
            TypeKind::Integer { bits: 64 } => MangledType::Integer64,

            TypeKind::Float { bits: 16 } => MangledType::Float16,
            TypeKind::Float { bits: 32 } => MangledType::Float32,
            TypeKind::Float { bits: 64 } => MangledType::Float64,

            TypeKind::Function { return_type,
                                 params,
                                 labels: _, } => {
                let params = params.iter().map(|param| param.mangle()).collect();

                MangledType::Function(params, Box::new(return_type.mangle()))
            }
            TypeKind::Struct(r#struct) => MangledType::Struct(r#struct.borrow().path().clone()),

            TypeKind::Void => MangledType::Void,
            TypeKind::Divergent => MangledType::Diverges,

            _ => panic!(),
        }
    }
}

impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.kind.hash(state); }
}

impl Deref for Type {
    type Target = TypeKind;

    fn deref(&self) -> &Self::Target { &self.kind }
}

impl DerefMut for Type {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.kind }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool { self.kind == other.kind }
}

impl Eq for Type {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.deref() {
            TypeKind::Named(name) => write!(f, "#{name}"),
            TypeKind::Member { parent, member } => write!(f, "{parent:?}.{member}"),
            TypeKind::Infer { key } => write!(f, "_{key}"),
            TypeKind::Void => write!(f, "()"),
            TypeKind::Function { return_type,
                                 params,
                                 labels: _, } => {
                let params = params.iter()
                                   .map(|par| format!("{par:?}"))
                                   .collect::<Vec<_>>()
                                   .join(", ");

                write!(f, "func ({params}): {return_type:?}")
            }
            TypeKind::Method { reciever,
                               return_type,
                               params, } => {
                let params = params.iter()
                                   .map(|par| format!("{par:?}"))
                                   .collect::<Vec<_>>()
                                   .join(", ");

                write!(f, "func (self: {reciever:?}, {params}): {return_type:?}")
            }
            TypeKind::Struct(struct_ref) => write!(f, "struct {}", struct_ref.name()),
            TypeKind::Integer { bits } => write!(f, "i{bits}"),
            TypeKind::Float { bits } => write!(f, "f{bits}"),
            TypeKind::Divergent => write!(f, "!"),
            TypeKind::Metatype(t) => write!(f, "<{:?}>", t),
            TypeKind::Error => write!(f, "error"),
            TypeKind::SomeInteger => write!(f, "some Integer"),
            TypeKind::SomeFloat => write!(f, "some Float"),
            TypeKind::SomeBool => write!(f, "some Bool"),
            TypeKind::SomeFunction => write!(f, "some func"),
        }
    }
}
