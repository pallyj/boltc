//!
//! The Bolt Mangler takes a list of symbols for each declaration, and outputs a string combining all of them. The string starts off with a list of the symbol,
//! identifiers, followed by a list of the identifier text. Any time two numbers are next to each other, an underscore is placed between them. The list of symbols can be found below -
//!
//! | Declaration  | Identifying letter | Associated Data           |
//! |--------------|--------------------|---------------------------|
//! | Library      | L                  | None                      |
//! | Protocol     | P                  | Generic Types             | 
//! | Class        | C                  | Generic Types             |
//! | Struct       | S                  | Generic Types             |
//! | Enum         | E                  | Generic Types             |
//! | Function     | F                  | Generic Types, Parameters |
//! | Initializer  | I                  | Generic Types, Parameters |
//! | Operator     | O                  | Generic Types, Parameters |
//! | Variable     | V                  | Generic Types             |
//! | Intrinsic    | i                  | None                      |
//! | Generic      | g                  | None                      |
//! 
//! # Associated Data
//! 
//! Some types of declaration can have data associated with them. This data is included in the identifier list at the end, and its symbol and the amount of items
//! are postpended onto the symbol declaration. The associated data types are as follows -
//! 
//! # Examples
//! 
//! func lang.Int.add => L4S3F3p2_4lang3Int3add_1

mod mangled;
pub (crate) mod parser;

pub use mangled::*;

#[cfg(test)]
mod tests;