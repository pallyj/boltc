mod struct_;
mod enum_;
mod case_;

use std::{fmt::{Debug, Display},
          hash::Hash,
          ops::{Deref, DerefMut},
          sync::atomic::{AtomicU64, Ordering}};

use errors::Span;
use itertools::Itertools;
use mangle::MangledType;
pub use struct_::*;
pub use enum_::*;
pub use case_::*;

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

    ///
    /// Signifies the type must be inferred. This type is created by the parser
    /// 
    Infer {
        key: u64,
    },

    UnknownInfer,

    // First-class types
    Void,
    Function {
        return_type: Box<Type>,
        params:      Vec<Type>,
        labels:      Vec<Option<String>>,
    },
    Method {
        reciever:    Box<Type>,
        return_type: Box<Type>,
        params:      Vec<Type>,
    },
    Struct(StructRef),
    Enum(EnumRef),
    Tuple(Vec<Type>, Vec<Option<String>>),

    SomeInteger,
    SomeFloat,
    SomeBool,
    SomeFunction,

    // Intrinsic types
    Integer {
        bits: u64,
    },
    Float {
        bits: u64,
    },
    StrSlice,

    // Second-class types
    Divergent,
    Metatype(Box<Type>),

    // Generic types
    GenericParam(String),
    GenericOf {
        higher_kind: Box<Type>,
        params: Vec<Type>,
    },

    HKRawPointer,

    ///
    /// A raw pointer to a value
    /// 
    RawPointer { pointer_type: Box<Type> },

    Array { item: Box<Type>, len: usize },

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
            TypeKind::Enum(r#enum) => r#enum.lookup_static_item(named),
            _ => None,
        }
    }

    pub fn lookup_instance_item(&self, named: &str, scope: &ScopeRef) -> Option<Symbol> {
        match &self.kind {
            TypeKind::Metatype(ty) => ty.clone().lookup_static_item(named),
            TypeKind::Struct(r#struct) => r#struct.lookup_instance_item(named, scope),
            TypeKind::Tuple(items, labels) => {
                for (i, label) in labels.iter().enumerate() {
                    if let Some(label) = label {
                        if label == named {
                            return Some(Symbol::TupleField(items[i].clone(), i))
                        }
                    }
                }

                if !named.starts_with("item") {
                    return None
                }

                let Ok(field_number) = usize::from_str_radix(&named[4..], 10) else {
                    return None
                };

                if field_number > items.len() || field_number == 0 {
                    return None
                }

                Some(Symbol::TupleField(items[field_number - 1].clone(), field_number - 1))
            }
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
            TypeKind::Enum(r#enum) => MangledType::Enum(r#enum.path()), // TODO: Fix

            TypeKind::Void => MangledType::Void,
            TypeKind::Divergent => MangledType::Diverges,

            TypeKind::StrSlice => MangledType::StringSlice,

            TypeKind::Tuple(types, _) => MangledType::Tuple(types.iter().map(Self::mangle).collect()),

            TypeKind::RawPointer { pointer_type: _ } => MangledType::Pointer, // todo: add arg

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
            TypeKind::UnknownInfer => write!(f, "_"),
            TypeKind::Void => write!(f, "()"),
            TypeKind::Function { return_type,
                                 params,
                                 labels, } => {
                let params = params.iter().zip(labels)
                                   .map(|(par, label)| if let Some(label) = label {
                                       format!("{label}: {par:?}")
                                   } else {
                                       format!("{par:?}")
                                   })
                                   .collect::<Vec<_>>()
                                   .join(", ");

                write!(f, "func ({params}): {return_type:?}")
            }
            TypeKind::Tuple(tuple_items, labels) => {
                let tuple_items = tuple_items.iter().zip(labels)
                                .map(|(par, label)| if let Some(label) = label { format!("{label}: {par:?}") } else { format!("{par:?}") } )
                                .collect::<Vec<_>>()
                                .join(", ");

                write!(f, "({tuple_items})")
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
            TypeKind::Enum(enum_ref) => write!(f, "enum {}", enum_ref.name()),
            TypeKind::Integer { bits } => write!(f, "i{bits}"),
            TypeKind::Float { bits } => write!(f, "f{bits}"),
            TypeKind::StrSlice => write!(f, "strslice"),
            TypeKind::Divergent => write!(f, "!"),
            TypeKind::Metatype(t) => write!(f, "<{:?}>", t),
            TypeKind::Error => write!(f, "error"),
            TypeKind::SomeInteger => write!(f, "some Integer"),
            TypeKind::SomeFloat => write!(f, "some Float"),
            TypeKind::SomeBool => write!(f, "some Bool"),
            TypeKind::SomeFunction => write!(f, "some func"),

            TypeKind::GenericParam(name) => write!(f, "{name}"),
            TypeKind::GenericOf { higher_kind, params } => write!(f, "{higher_kind:?}<{}>", params.iter().map(|p| format!("{p:?}")).collect::<Vec<_>>().join(", ")),
            TypeKind::HKRawPointer => write!(f, "RawPointer<_>"),
            TypeKind::RawPointer { pointer_type: ptr } => write!(f, "RawPointer<{ptr:?}>"),
            TypeKind::Array { item, len } => write!(f, "{item:?}[{len}]"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TypeKind::*;
        
        match self.kind() {
            Void => write!(f, "()"),
            Function { return_type, params, labels } => write!(f, "func ({}) -> {return_type}", params.iter().zip(labels).map(|(ty, label)| if let Some(label) = label {
                format!("{label}: {}", ty)
            } else {
                format!("{ty}")
            }).join(", ")),
            Method { reciever, return_type, params } => write!(f, "func (self: {reciever}, {}) -> {return_type}", params.iter().join(", ")),
            Struct(struct_repr) => write!(f, "struct `{}`", struct_repr.name()),
            Enum(enum_repr) => write!(f, "enum `{}`", enum_repr.name()),
            Tuple(types, labels) => write!(f, "({})", types.iter().zip(labels).map(|(ty, label)| if let Some(label) = label {
                format!("{label}: {ty}")
            } else {
                format!("{ty}")
            }).join(", ")),
            
            Integer { bits } => write!(f, "i{bits}"),
            Float { bits } => write!(f, "f{bits}"),
            Divergent => write!(f, "!"),
            
            RawPointer { pointer_type } => write!(f, "RawPointer<{pointer_type}>"),
            Array { item, len } => write!(f, "{item}[{len}]"),
    
            Error => todo!(),
    
            Metatype(_) => todo!(),
    
            Named(_) => todo!(),
            Member { parent, member } => todo!(),
            GenericParam(_) => todo!(),
            GenericOf { higher_kind, params } => todo!(),
            HKRawPointer => todo!(),
    
            Infer { key } => todo!(),
            UnknownInfer => todo!(),
    
            SomeInteger => todo!(),
            SomeFloat => todo!(),
            SomeBool => todo!(),
            SomeFunction => todo!(),
    
            StrSlice => todo!(),
        }
    }
}