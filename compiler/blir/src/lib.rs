#![feature(associated_type_defaults)]
#![feature(arc_new_cyclic)]

use std::fmt::Display;

mod sym;
mod typ;
mod var;
mod func;
mod expr;
mod class;
mod scope;
mod method;
mod library;
mod enumdef;
mod metadata;
mod protocol;
mod structdef;
mod walker;
mod smt;
mod scopes;
mod intrinsics;

pub use sym::*;
pub use typ::*;
pub use var::*;
pub use func::*;
pub use expr::*;
pub use class::*;
pub use scope::*;
pub use method::*;
pub use library::*;
pub use enumdef::*;
pub use metadata::*;
pub use protocol::*;
pub use structdef::*;
pub use walker::*;
pub use smt::*;
pub use scopes::*;
pub use intrinsics::*;

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Internal,
    Fileprivate,
    Private
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "public"),
            Visibility::Internal => write!(f, "internal"),
            Visibility::Fileprivate => write!(f, "fileprivate"),
            Visibility::Private => write!(f, "private"),
        }
    }
}