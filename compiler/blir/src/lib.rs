#![feature(let_else)]

pub mod attributes;
pub mod code;
pub mod intrinsics;
pub mod scope;
pub mod typ;
pub mod value;

mod library;
mod sym;
mod visibility;

pub use library::*;
pub use sym::*;
use typ::StructRef;
pub use visibility::*;

pub struct BlirContext {
    pub default_integer_repr: Option<StructRef>,
    pub default_float_repr:   Option<StructRef>,
    pub default_bool_repr:    Option<StructRef>,
    pub default_string_repr:  Option<StructRef>,

    pub entry_point: Option<String>,
}

impl BlirContext {
    pub fn new() -> Self {
        Self { default_integer_repr: None,
               default_float_repr:   None,
               default_bool_repr:    None,
               default_string_repr:  None,

               entry_point: None, }
    }
}
