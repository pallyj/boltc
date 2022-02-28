use std::fmt::Display;

use prelude::Source;

use crate::{Expr, Type};

#[derive(Clone)]
pub enum StatementKind {
	Eval(Expr),

	Bind {
		name: String,
		typ: Type,
		value: Option<Expr>,
	},

	Repeat {
		code: CodeBlock,
	},

	While {
		condition: Expr,
		code: CodeBlock,
	},

	Break {
		label: Option<String>,
	},

	Continue {
		label: Option<String>,
	},

	Return {
		value: Option<Expr>,
	},

	Throw {
		value: Expr,
	}
}

impl StatementKind {
	pub fn anon(self) -> Statement {
		Statement::new_anon(self)
	}

	pub fn sourced(self, source: Source) -> Statement {
		Statement::new(self, source)
	}
}

#[derive(Clone)]
pub struct Statement {
	kind: StatementKind,
	source: Option<Source>,
}

impl Statement {
	pub fn new(kind: StatementKind, source: Source) -> Statement {
		Statement {
			kind,
			source: Some(source),
		}
	}

	pub fn new_anon(kind: StatementKind) -> Statement {
		Statement {
			kind,
			source: None
		}
	}

	pub fn kind(&self) -> &StatementKind {
		&self.kind
	}

	pub fn kind_mut(&mut self) -> &mut StatementKind {
		&mut self.kind
	}

	pub fn source(&self) -> &Option<Source> {
		&self.source
	}

	pub fn source_mut(&mut self) -> &mut Option<Source> {
		&mut self.source
	}

	pub fn diverges(&self) -> bool {
		match self.kind() {
			StatementKind::Return { .. } => true,
			StatementKind::Throw { .. } => true,
			StatementKind::Break { .. } => true,
			StatementKind::Continue { .. } => true,
			_ => false
		}
	}
}

#[derive(Clone)]
pub struct CodeBlock {
	statements: Vec<(Statement, bool)>,
}

impl CodeBlock {
	pub fn new(statements: Vec<(Statement, bool)>) -> CodeBlock {
		CodeBlock {
			statements
		}
	}

	pub fn statements_mut(&mut self) -> &mut Vec<(Statement, bool)> {
		&mut self.statements
	}

	pub fn statements(&self) -> &Vec<(Statement, bool)> {
		&self.statements
	}
}

impl Display for StatementKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			StatementKind::Eval(expr) => write!(f, "{expr}"),

			StatementKind::Bind { name, typ, value } => {
				write!(f, "let {name}")?;

				write!(f, ": {typ}")?;

				if let Some(val) = value {
					write!(f, " = {val}")?;
				}

				Ok(())
			}

			StatementKind::Break { label } => if let Some(label) = label {
				write!(f, "break {label}")
			} else {
				write!(f, "break")
			}

			StatementKind::Continue { label } => if let Some(label) = label {
				write!(f, "continue {label}")
			} else {
				write!(f, "continue")
			}

			StatementKind::Return { value } => if let Some(value) = value {
				write!(f, "return {value}")
			} else {
				write!(f, "return")
			}

			StatementKind::Throw { value } => write!(f, "throw {value}"),

			StatementKind::Repeat { code } => write!(f, "repeat {{\n{code}\n}}"),
			StatementKind::While { condition, code } => write!(f, "while {condition} {{\n{code}\n}}")
		}
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let smts = self.statements
			.iter()
			.map(|smt| {
				if smt.1 {
					format!("{}", smt.0)
				} else {
					format!("{};", smt.0)
				}
			})
			.collect::<Vec<_>>()
			.join("\n");

        write!(f, "{smts}")
    }
}