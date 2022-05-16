mod maranget;

pub use maranget::Maranget;
pub use maranget::DecisionTree;

use crate::PatternMatrix;

pub trait Solver {
	type Automata;

	fn solve(matrix: PatternMatrix) -> Self::Automata;
}