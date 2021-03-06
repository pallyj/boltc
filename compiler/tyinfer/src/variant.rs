use blir::typ::{StructRef, Type, EnumRef};
use rusttyc::{Partial, Variant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeVariant {
    Unconstrained,

    SomeDiverges,

    SomeInteger,
    SomeFloat,
    SomeBool,
    SomeString,
    SomeChar,
    SomeFunction,

    LlvmInt {
        bits: u32,
    },
    LlvmFloat {
        bits: u32,
    },
    LlvmBool,
    LlvmString,

    Struct(StructRef),
    Enum(EnumRef),

    Function {
        params:      Vec<Type>,
        labels:      Vec<Option<String>>,
        return_type: Box<Type>,
    },
    Tuple(Vec<Type>, Vec<Option<String>>),
    RawPointer(Type),
    Array(Type, usize),

    GenericParam(String),

    Void,
    Diverges,

    Error,
}

impl Variant for TypeVariant {
    type Err = String;

    fn top() -> Self { Self::Unconstrained }

    fn meet(lhs: rusttyc::Partial<Self>, rhs: rusttyc::Partial<Self>) -> Result<rusttyc::Partial<Self>, Self::Err> {
        let variant = match (lhs.variant, rhs.variant) {
            (Self::Diverges, x) | (x, Self::Diverges) => Ok(x),
            
            (Self::Unconstrained, x) | (x, Self::Unconstrained) => Ok(x),

            (Self::SomeDiverges, x) | (x, Self::SomeDiverges) => Ok(x),

            (Self::SomeInteger, Self::LlvmInt { bits }) | (Self::LlvmInt { bits }, Self::SomeInteger) => Ok(Self::LlvmInt { bits }),

            (Self::SomeFloat, Self::LlvmFloat { bits }) | (Self::LlvmFloat { bits }, Self::SomeFloat) => Ok(Self::LlvmFloat { bits }),

            (Self::SomeBool, Self::LlvmBool) | (Self::LlvmBool, Self::SomeBool) => Ok(Self::LlvmBool),

            (Self::SomeString, Self::LlvmString) | (Self::LlvmString, Self::SomeString) |
            (Self::SomeChar, Self::LlvmString) | (Self::LlvmString, Self::SomeChar) => Ok(Self::LlvmString),

            (Self::SomeFunction, function @ Self::Function { .. }) => Ok(function),

            (Self::SomeInteger, Self::Struct(integer_struct)) | (Self::Struct(integer_struct), Self::SomeInteger) => {
                if integer_struct.integer_repr() {
                    Ok(Self::Struct(integer_struct))
                } else {
                    Err(format!("struct '{}' is not representable by an integer",
                                integer_struct.name()))
                }
            }

            (Self::SomeFloat, Self::Struct(float_struct)) | (Self::Struct(float_struct), Self::SomeFloat) => {
                if float_struct.float_repr() {
                    Ok(Self::Struct(float_struct))
                } else {
                    Err(format!("struct '{}' is not representable by a float",
                                float_struct.name()))
                }
            }

            (Self::SomeBool, Self::Struct(bool_struct)) | (Self::Struct(bool_struct), Self::SomeBool) => {
                if bool_struct.bool_repr() {
                    Ok(Self::Struct(bool_struct))
                } else {
                    Err(format!("struct '{}' is not representable by a boolean",
                                bool_struct.name()))
                }
            }

            (Self::SomeChar, Self::Struct(string_struct)) | (Self::Struct(string_struct), Self::SomeChar) => {
                if string_struct.string_repr() {
                    Ok(Self::Struct(string_struct))
                } else if string_struct.char_repr() {
                    Ok(Self::Struct(string_struct))
                } else {
                    Err(format!("struct '{}' is not representable by a string",
                                string_struct.name()))
                }
            }

            (Self::SomeString, Self::Struct(string_struct)) | (Self::Struct(string_struct), Self::SomeString) => {
                if string_struct.string_repr() {
                    Ok(Self::Struct(string_struct))
                } else {
                    Err(format!("struct '{}' is not representable by a string",
                                string_struct.name()))
                }
            }

            (_, Self::Error) | (Self::Error, _) => Ok(Self::Error),

            (x, y) if x == y => Ok(x),
            _ => Err("types don't match".to_string()),
        }?;

        Ok(Partial { variant, least_arity: 0 })
    }

    fn arity(&self) -> rusttyc::Arity { rusttyc::Arity::Fixed(0) }
}


impl Default for TypeVariant {
    fn default() -> Self {
        Self::Void
    }
}