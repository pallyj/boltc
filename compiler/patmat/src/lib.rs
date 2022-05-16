#![feature(type_alias_impl_trait)]
#![feature(drain_filter)]

mod solver;
mod matrix;

#[cfg(test)]
mod tests;

pub use matrix::PatternMatrix;
pub use solver::Maranget;
pub use solver::DecisionTree;