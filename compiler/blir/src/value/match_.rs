use std::fmt::Debug;

use crate::{code::CodeBlock, pattern::Pattern};

use super::Value;

#[derive(Clone)]
pub struct MatchValue {
	pub discriminant: Box<Value>,
	pub branches: Vec<MatchBranch>,
}

impl Debug for MatchValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "match {:?} {{", self.discriminant)?;

		for branch in &self.branches {
			let branch = format!("\t{branch:?}");
			writeln!(f, "{}", branch.replace("\n", "\n\t"))?;
		}

		write!(f, "}}")
    }
}

#[derive(Clone)]
pub struct MatchBranch {
	pub pattern: Pattern,
	pub code: CodeBlock,
}

impl Debug for MatchBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{pattern:?} => {code:?}", pattern=self.pattern, code=self.code)
    }
}
