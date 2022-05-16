use std::{fmt::Debug};

use blir::{value::{Value, ValueKind}, pattern::{Pattern, PatternKind}, typ::{TypeKind, Type}};

use crate::solver::Solver;

#[derive(Copy, Clone)]
pub struct MatchEnd(pub (crate) u64);

impl MatchEnd {
	pub fn index(&self) -> u64 {
		self.0
	}
}

pub struct PatternMatrix {
	compare_values: Vec<Value>,
	rows: Vec<PatternRow>
}

#[derive(Clone)]
pub struct PatternRow {
	pub (crate) patterns: Vec<Pattern>,
	binds: Vec<(String, Value)>,
	end: MatchEnd,
}

impl PatternMatrix {
	pub (crate) fn new(
		compare_values: Vec<Value>,
		rows: Vec<PatternRow>) -> Self 
	{
		Self { compare_values, rows }
	}

	/// `compare_value` should be a variable, to prevent
	/// code repetition
	pub fn construct(
		compare_value: Value,
		branches: Vec<Pattern>) -> Self
	{
		let rows = branches.into_iter()
						   .enumerate()
						   .map(|(i, pat)| PatternRow { patterns: vec![ pat ], binds: vec![], end: MatchEnd(i as u64) })
						   .collect();

		Self {
			compare_values: vec![compare_value],
			rows,
		}
	}

	fn expand_or_branches(&mut self) {
		// We don't support OR branches
	}

	pub fn expand(mut self) -> Self {
		use std::mem;

		// First, expand any OR patterns
		self.expand_or_branches();

		let mut current_matrix = self;

		loop {
			let mut compare_values = vec![];

			let mut rows = current_matrix.rows.iter()
											  .map(|row| PatternRow::proceed(row))
											  .collect::<Vec<_>>();

			let old_len = current_matrix.compare_values.len();

			for (i, compare_value) in current_matrix.compare_values.into_iter().enumerate() {
				let should_split = current_matrix.rows
					.iter()
					.any(|row| row.patterns[i].has_children());

				if should_split {
					// Split the scrutinee into categories
					let split_scrutinee = split_scrutinee(&compare_value);
					let split_types = split_scrutinee.iter()
													 .map(|scrut| scrut.typ.clone())
													 .collect::<Vec<_>>();

					compare_values.extend_from_slice(&split_scrutinee);

					// Add the pattern to each row
					for (old_row, new_row) in current_matrix.rows.iter_mut().zip(&mut rows) {
						let taken_pat = mem::take(&mut old_row.patterns[i]); // mem::replace(&mut old_row.patterns[i], mem::zeroed())

						// If the pattern we took is a bind, then add a bind to compare_value
						if let PatternKind::Bind(bind_name) = &taken_pat.kind {
							new_row.binds.push((bind_name.clone(), compare_value.clone()));
						}

						let sub_patterns = split_pattern(taken_pat, &split_types);

						new_row.patterns.extend_from_slice(&sub_patterns)
					}
				} else {
					// Add the old scrutinee
					compare_values.push(compare_value.clone());

					// Add the pattern to each row
					for (old_row, new_row) in current_matrix.rows.iter_mut().zip(&mut rows) {
						let mut taken_pat = mem::take(&mut old_row.patterns[i]); // mem::replace(&mut old_row.patterns[i], mem::zeroed())

						if let PatternKind::Bind(bind_name) = &taken_pat.kind {
							new_row.binds.push((bind_name.clone(), compare_value.clone()));
							taken_pat.kind = PatternKind::Wildcard;
						}

						new_row.patterns.push(taken_pat);
					}
				}
			}

			current_matrix = Self { compare_values, rows };

			if current_matrix.compare_values.len() == old_len {
				break
			}
		}

		current_matrix	
	}

	pub fn solve<S: Solver>(self) -> S::Automata {
		S::solve(self)
	}

	pub fn rows(&self) -> std::slice::Iter<PatternRow> {
		self.rows.iter()
	}

	pub fn columns(&self) -> usize {
		self.compare_values.len()
	}

	pub fn scrutinees(&self) -> std::slice::Iter<Value> {
		self.compare_values.iter()
	}

	pub fn sort(
		mut self,
		index_list: Vec<usize>) -> PatternMatrix
	{
		use std::mem;

		let mut scrutinees = vec![];
		let mut rows = self.rows()
						   .map(|row| PatternRow::proceed(row))
						   .collect::<Vec<_>>();

		for index in index_list {
			scrutinees.push(mem::take(&mut self.compare_values[index]));

			for (old_row, new_row) in self.rows.iter_mut().zip(&mut rows) {
				new_row.patterns.push(mem::take(&mut old_row.patterns[index]));
			}
		}

		PatternMatrix::new(scrutinees, rows)
	}
}

fn split_scrutinee(scrutinee: &Value) -> Vec<Value> {
	match scrutinee.typ.kind() {
		TypeKind::Tuple(tuple_types) => {
			tuple_types.iter()
					   .enumerate()
					   .map(|(i, ty)| ValueKind::TupleField(Box::new(scrutinee.clone()), i).anon(ty.clone()))
					   .collect()
		}
		_ => vec![ scrutinee.clone() ]
	}
}

fn split_pattern(pattern: Pattern, tuple_types: &[Type]) -> Vec<Pattern> {
	if pattern.matches_any() {
		return tuple_types.iter()
				   		  .map(|ty| PatternKind::Wildcard.with_type(ty.clone()))
						  .collect()
	}

	match pattern.kind {
		PatternKind::Tuple { items } => items,
		_ => unreachable!()
	}
}

impl PatternRow {
	pub (crate) fn proceed(old_row: &PatternRow) -> Self {
		PatternRow { patterns: vec![],
					 binds: old_row.binds.clone(),
					 end: old_row.end }
	}

	pub fn leaf(&self) -> MatchEnd {
		self.end
	}
}

impl Debug for PatternRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let seq = self.patterns.iter()
					  		   .map(|pat| format!("{pat:?}"))
					  		   .collect::<Vec<_>>()
					  		   .join(", ");

		write!(f, "{seq} => {n}", n = self.end.0)?;

		for bind in &self.binds {
			writeln!(f)?;
			write!(f, "  let {} = {:?}", bind.0, &bind.1)?;
		}

		Ok(())
    }
}

impl Debug for PatternMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scrutinee = self.compare_values.iter()
										   .map(|pat| format!("{pat:?}"))
										   .collect::<Vec<_>>()
										   .join(", ");

		writeln!(f, "{scrutinee}")?;
		writeln!(f)?;

		for row in &self.rows {
			writeln!(f, "{row:?}")?;
		}

		Ok(())
    }
}



// construct
// expand
// 


// [a, b, c]
//
// [1, 2, 3]
// [1, _, 1]
// [_, 2, _]
// _


// a     b      c
// 1     2      3
// 1     _      1
// _     2      _
// _     _      _