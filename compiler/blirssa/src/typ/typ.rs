use super::{StructRef, EnumRef};

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Void,

    Integer {
        bits: u32,
    },
    Float {
        bits: u32,
    },
    StrSlice,

    Function {
        pars:        Vec<Type>,
        return_type: Box<Type>,
    },

    Pointer {
        pointee: Box<Type>,
    },

    // Struct
    Struct {
        container: StructRef,
    },
    Enum (EnumRef),
    Tuple(Vec<Type>),
}

impl Type {
    pub fn func_type(self, pars: Vec<Type>) -> Type {
        Type::Function { pars,
                         return_type: Box::new(self) }
    }

    pub fn pointer(self) -> Type { Type::Pointer { pointee: Box::new(self) } }

    /*pub fn size(&self) -> usize {
        match self {
            Type::Void => 0,
            Type::Integer { bits } => bits / 8,
            Type::Float { bits } => bits / 8,
            Type::StrSlice => 16, // 8 on 32-bit platform
            Type::Function { pars, return_type } => 0, // unsized
            Type::Pointer { pointee } => 8, // 4 on 32-bit platform
            Type::Struct { container } => container.size(),
            Type::Enum(container) => container.size(),
            Type::Tuple(tuple_items) => tuple_items.iter().fold(0, |acc, x| acc + x.size()),
        }
    }*/

    /*pub fn align(&self) -> usize {
        match self {
            Type::Void => todo!(),
            Type::Integer { bits } => todo!(),
            Type::Float { bits } => todo!(),
            Type::StrSlice => todo!(),
            Type::Function { pars, return_type } => todo!(),
            Type::Pointer { pointee } => todo!(),
            Type::Struct { container } => todo!(),
            Type::Enum(_) => todo!(),
            Type::Tuple(_) => todo!(),
        }
    }*/
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),

            Type::Integer { bits } => write!(f, "i{bits}"),
            Type::Float { bits } => write!(f, "f{bits}"),
            Type::StrSlice => write!(f, "strslice"),

            Type::Function { pars, return_type } => {
                let pars = pars.iter()
                               .map(|par| par.to_string())
                               .collect::<Vec<_>>()
                               .join(", ");

                write!(f, "({pars}): {return_type}")
            }

            Type::Pointer { pointee } => {
                write!(f, "inout {pointee}")
            }

            Type::Struct { container } => {
                write!(f, "struct {}", container.name())
            }

            Type::Enum(container) => {
                write!(f, "enum {}", container.name())
            }

            Type::Tuple(tuple_items) => {
                let tuple_items = tuple_items.iter()
                                             .map(|item| item.to_string())
                                             .collect::<Vec<_>>()
                                             .join(", ");

                write!(f, "({tuple_items})")
            }
        }
    }
}
