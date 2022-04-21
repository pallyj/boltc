use std::fmt::Display;

use crate::Path;

pub enum MangledType {
    Integer8,
    Integer1,
    Char,
    Float64,
    Float32,
    Float16,
    Integer64,
    Integer32,
    Integer16,
    Generic,
    StringSlice,
    Pointer,
    Void,
    Varargs,
    Diverges,

    Tuple(Vec<MangledType>),
    Function(Vec<MangledType>, Box<MangledType>),
    Struct(Path),
}

impl Display for MangledType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MangledType::Integer8 => write!(f, "a"),
            MangledType::Integer1 => write!(f, "b"),
            MangledType::Char => write!(f, "c"),
            MangledType::Float64 => write!(f, "d"),
            MangledType::Float32 => write!(f, "f"),
            MangledType::Float16 => write!(f, "h"),
            MangledType::Integer64 => write!(f, "i"),
            MangledType::Integer32 => write!(f, "j"),
            MangledType::Integer16 => write!(f, "l"),
            MangledType::Generic => write!(f, "p"),
            MangledType::StringSlice => write!(f, "r"),
            MangledType::Pointer => write!(f, "t"),
            MangledType::Void => write!(f, "u"),
            MangledType::Varargs => write!(f, "v"),
            MangledType::Diverges => write!(f, "z"),

            MangledType::Tuple(tuple_items) => {
                let tuple_items = tuple_items.iter()
                                             .map(ToString::to_string)
                                             .collect::<String>();

                write!(f, "T{tuple_items}E")
            }

            MangledType::Function(params, return_type) => {
                let params = params.iter().map(ToString::to_string).collect::<String>();

                write!(f, "F{params}E{return_type}")
            }

            MangledType::Struct(path) => {
                write!(f, "{path}S")
            }
        }
    }
}
