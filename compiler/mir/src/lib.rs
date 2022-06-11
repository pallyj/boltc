#![feature(let_else)]

pub mod ty;
pub mod val;
pub mod instr;
pub mod code;
mod builder;
mod project;
pub mod exc;

pub use builder::Builder;
pub use project::Project;

/*

todo list:

- finish execution engine
- add enums
- add dropping
- add tests
- add documentation
- reach consensus 1
- stress test it

*/


#[cfg(test)]
mod tests;