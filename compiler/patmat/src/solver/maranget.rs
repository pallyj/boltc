// Works cited
// 
// 'Compiling Pattern Matching to Good Decision Trees', Luc Maranget © 2008
//


use std::fmt::Debug;

use blir::{pattern::{Pattern, PatternKind}, value::{ValueKind, Value}};

use crate::{PatternMatrix, matrix::MatchEnd};

use super::Solver;

pub struct Maranget {}

impl Solver for Maranget {
    type Automata = DecisionTree;

    fn solve(matrix: crate::PatternMatrix) -> Self::Automata {
        // Calculate a score for each column
		// The score is calculated by traveling down the column
		// Each row adds 1 to the score
		// However, if a wildcard pattern is encountered
		// The count will stop for that column

		let mut scores = vec![0; matrix.columns()];
		let mut is_at_end = vec![false; matrix.columns()];

		for row in matrix.rows() {
			for (i, pattern) in row.patterns.iter().enumerate() {
				if is_at_end[i]  { continue }

				if pattern.matches_any() {
					is_at_end[i] = true
				} else {
					scores[i] += 1
				}
			}
		}

		// Now, we get the column with the highest score

		// Now, we order the columns by their scores
		let mut orders = scores.iter().enumerate().collect::<Vec<_>>();
		orders.sort_by(|a, b| b.1.cmp(a.1));

		// Instead of reordering the columns, simply
		// address them through the index list
		let index_list = orders.iter()
							   .map(|order| order.0)
							   .collect::<Vec<_>>();

		// Don't sort it in place, grab from the old pattern matrix
		// And create a new one
		let matrix = matrix.sort(index_list);

		// Now generate a decision tree for the matrix
		Self::generate_decision_tree_for_matrix(matrix)
    }
}

impl Maranget {
	fn generate_decision_tree_for_matrix(
		matrix: PatternMatrix) -> DecisionTree
	{
		if matrix.rows().len() == 0 {
			return DecisionTree::Fail;
		}

		// Technically, if the first row is all wildcards, this case applies too
		// However, in practice, this is the only time this case should occur
		let first_row = matrix.rows().next().unwrap();
		if first_row
			.patterns.iter()
			.all(|pat| pat.matches_any())
		{
			let leaf = first_row.leaf();
			let bindings = first_row.bindings().clone();
			
			return DecisionTree::Leaf(leaf, bindings)
		}

		let mut cases = Self::cases(&matrix);

		let mut next_patterns = vec![];

		// Find the unique scrutinees and get a scrutinee matrices for them
		while cases.len() > 0 {
            let current_case = cases.remove(0);

            // Remove any overlapping patterns
            cases
				.retain(|possible_duplicate| {
                    // Drain it if its pattern is equivalent to `switch_scrutinee`
                    !Self::patterns_match(&possible_duplicate, &current_case)
                });

				if current_case.matches_any() {
					continue
				}

			// Specialise the matrix on `switch_scrutinee`
			let specialized_matrix = Self::specialize_matrix(&matrix, &current_case);

			let switch_tree = Self::generate_decision_tree_for_matrix(specialized_matrix);

			next_patterns.push((current_case, switch_tree));
        }

		// Get the default case
		let default_matrix = Self::default_matrix(&matrix);
		let default_tree = Self::generate_decision_tree_for_matrix(default_matrix);
		let default_pattern = if let DecisionTree::Fail = default_tree {
			None
		} else {
			Some(Box::new(default_tree))
		};

		if next_patterns.len() == 0 {
			if default_pattern.is_some() {
				return *default_pattern.unwrap();
			} else {
				return DecisionTree::Fail
			}
		}

		let scrutinee = matrix.scrutinees().next().unwrap();

		DecisionTree::Switch { scrutinee: scrutinee.clone(),
							   patterns: next_patterns,
							   default: default_pattern }
	}

	fn cases(
		matrix: &PatternMatrix) -> Vec<Pattern>
	{
		let column = 0;

		matrix.rows()
			  .map(|row| row.patterns[column].clone())
			  .collect()
	}

	fn default_matrix(matrix: &PatternMatrix) -> PatternMatrix {
		let column = 0;

		// Remove the scrutinee at index `column`
		let mut scrutinees = matrix.scrutinees().cloned().collect::<Vec<_>>();
		scrutinees.remove(column);

		// Now retain the rows where the pattern at index `column`
		// is a wildcard pattern
		let mut rows = matrix.rows()
							 .filter(|row| row.patterns[column].matches_any())
							 .cloned()
							 .collect::<Vec<_>>();

		// Get a list of the columns so we can find the scrutinee matrices
		rows.iter_mut()
			.for_each(|row| {
				row.patterns.remove(column);
			});

		PatternMatrix::new(scrutinees, rows)
	}

	fn specialize_matrix(matrix: &PatternMatrix, pattern: &Pattern) -> PatternMatrix {
		let column = 0;

		// Remove the scrutinee at index `column`
		let mut scrutinees = matrix.scrutinees().cloned().collect::<Vec<_>>();
		scrutinees.remove(column);

		// Now retain the rows where the pattern at index `column`
		// is a wildcard pattern
		let mut rows = matrix.rows()
							 .filter(|row| Self::patterns_match(pattern, &row.patterns[column]) ||
							 			   row.patterns[column].matches_any())
							 .cloned()
							 .collect::<Vec<_>>();

		// Get a list of the columns so we can find the scrutinee matrices
		rows.iter_mut()
			.for_each(|row| {
				row.patterns.remove(column);
			});

		PatternMatrix::new(scrutinees, rows)
	}

	fn patterns_match(pattern1: &Pattern, pattern2: &Pattern) -> bool {
        match (&pattern1.kind, &pattern2.kind) {
            (PatternKind::Bind(_), PatternKind::Bind(_)) |
            (PatternKind::Bind(_), PatternKind::Wildcard) |
            (PatternKind::Wildcard, PatternKind::Bind(_)) => true,

			(PatternKind::Integer { value: v1 }, PatternKind::Integer { value: v2 }) => v1 == v2,

            (PatternKind::Literal { value: value1 }, PatternKind::Literal { value: value2 }) => {
                // Check if the values match
                match (&value1.kind, &value2.kind) {
                    (ValueKind::IntLiteral(n), ValueKind::IntLiteral(n2)) => n == n2,
					(ValueKind::BoolLiteral(b1), ValueKind::BoolLiteral(b2)) => b1 == b2,
                    (ValueKind::StringLiteral(s), ValueKind::StringLiteral(s2)) => s == s2,
                    (ValueKind::EnumVariant { variant: v1, .. }, ValueKind::EnumVariant { variant: v2, .. }) => v1.name() == v2.name(),

                    _ => false,
                }
            }

            _ => false
        }
    }
}

pub enum DecisionTree {
	Leaf(MatchEnd, Vec<(String, Value)>),
	Fail,
	Switch {
		scrutinee: Value,
		patterns: Vec<(Pattern, DecisionTree)>,
		default: Option<Box<DecisionTree>>
	},
}

impl Debug for DecisionTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf(arg0, ..) => write!(f, "leaf {}", arg0.0),
            Self::Fail => write!(f, "unreachable"),
            Self::Switch { scrutinee, patterns, default } => {
				writeln!(f, "switch {scrutinee:?}")?;

				for (pattern, next) in patterns {
					let next_str = format!("{:?}", next);
					writeln!(f, "\t{pattern:?} => {}", next_str.replace("\n", "\n\t"))?;
				}

				if let Some(default) = default {
					let next_str = format!("{:?}", default);
					writeln!(f, "\t_ => {}", next_str.replace("\n", "\n\t"))?;
				}

				Ok(())
			}
        }
    }
}