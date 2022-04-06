use super::StructRef;

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Void,

    Integer {
        bits: u32,
    },
    Float {
        bits: u32,
    },

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
}

impl Type {
    pub fn func_type(self, pars: Vec<Type>) -> Type {
        Type::Function { pars,
                         return_type: Box::new(self) }
    }

    pub fn pointer(self) -> Type { Type::Pointer { pointee: Box::new(self) } }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "void"),

            Type::Integer { bits } => write!(f, "i{bits}"),
            Type::Float { bits } => write!(f, "f{bits}"),

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
        }
    }
}
