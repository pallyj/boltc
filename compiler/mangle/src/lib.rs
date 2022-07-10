//! The Bolt mangler produces a unique name for a language item, allowing polymorphism and reusing the same name in a different scope
//! The Bolt mangler mangles names based on a set of rules, which can be composed of other rules
//!
//! ## Identifier
//!
//! An identifier is mangled by prefixing the text of the identifier with its length
//!
//! ## Path
//!
//! A path is made up of a list of members. The last identifier in the list is the item, and the others are its parents in reverse order
//!
//! ## Struct definition
//!
//! A struct definition consists of a path, and is marked as a struct by a 'S' postfix
//!
//! ## Types
//!
//! ### Basic types
//!
//! Basic, or intrinsic types, are marked by a letter. The table below provides a list of intrinsic types and their mangle symbols:
//!
//! | Type      | Symbol    |
//! |-----------|-----------|
//! | i8        | a         |
//! | i1        | b         |          
//! | char      | c         |
//! | f64       | d         |
//! | f32       | f         |
//! | f16       | h         |
//! | i64       | i         |
//! | i32       | j         |
//! | i16       | l         |
//! | generic   | p         |
//! | strslice  | r         |
//! | pointer   | t         |
//! | void      | u         |
//! | varargs   | v         |
//! | diverges  | z         |
//!
//! ### Collection types
//!
//! Intrinsic types which are based on other types can also be represented
//!
//! | Type      | Prefix    | List              | End | After       |
//! |-----------|-----------|-------------------|-----|-------------|
//! | Tuple     | T         | Tuple Types       | E   |             |
//! | Function  | F         | Parameter Types   | E   | Return type |
//!
//! ### User defined types
//!
//! A user defined type is written as its path, followed by a letter identifying what kind of type it is
//!
//! | Type   | Letter   |
//! |--------|----------|
//! | Struct | S        |
//!
//! ## Function definition
//!
//! A function definition is mangled as a path, followed by a specifier with its labels and parameters
//!
//! The specifier starts with the letter F, followed by a list of the type parameters. This list ends with an E,
//! after which a list of the labels are provided. A missing label will be implemented as a 0 length string.

mod defs;
mod path;
#[cfg(test)]
mod tests;
mod typ;

pub use defs::{MangledFunction, MangledStruct, MangledEnum, MangledGlobal};
pub use path::Path;
pub use typ::MangledType;
