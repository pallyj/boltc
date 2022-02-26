use std::{sync::{Arc}, fmt::Display};

use prelude::Source;

use crate::{typ::Type, func::FuncDef, CodeBlock};

#[derive(Clone)]
pub enum ExprKind {
	Unit,
	Tuple(Vec<Expr>),

	IntLiteral(u64),
	FloatLiteral(f64),
	StringLiteral(String),

	Named(String),

	Member(Box<Expr>, String),
	OptionalMember(Box<Expr>, String),

	/// Calls a function
	FuncCall {
		func: Box<Expr>,
		args: Vec<FuncArg>,
	},

	/// A global function
	Function(Arc<FuncDef>),
	/// An initializer for a type
	Init(Type),
	/// A method
	Method {
		of: Type,
		reciever: Box<Expr>,
	},
	/// A static method
	StaticMethod {
		of: Type
	},

	/// Index into a type
	Index(Type),

	/// An infix operator
	InfixOperator(Type, Type, String),
	/// Either a postfix or a prefix operator
	FixOperator(Type, String),

	/// An intrinsic function
	IntrinsicFunc(String),

	/// Selects a value
	Select {
		branches: Vec<SelectBranch>,
		finally: Option<Box<CodeBlock>>
	},

	FunctionParameter(usize),
	LocalVariable(String),

	None,
}

#[derive(Clone)]
pub struct SelectBranch {
	condition: Expr,
	code: CodeBlock,
}

impl SelectBranch {
	pub fn new(condition: Expr, code: CodeBlock) -> Self {
		Self {
			condition,
			code
		}
	}

	pub fn condition_mut(&mut self) -> &mut Expr {
		&mut self.condition
	}

	pub fn code_mut(&mut self) -> &mut CodeBlock {
		&mut self.code
	}
}

impl Display for SelectBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} => {{\n    {}\n}}", self.condition, self.code.to_string().replace("\n", "\n    "))
    }
}

#[derive(Clone)]
pub struct Expr {
	kind: ExprKind,
	typ: Type,
	source: Option<Source>
}

#[derive(Clone)]
pub struct FuncArg {
	label: Option<String>,
	value: Expr,
}

impl Expr {
	pub fn new(kind: ExprKind, typ: Type, source: Source) -> Self {
		Self {
			kind,
			typ,
			source: Some(source)
		}
	}

	pub fn new_anon(kind: ExprKind, typ: Type) -> Self {
		Self {
			kind,
			typ,
			source: None,
		}
	}

	pub fn typ(&self) -> Type {
		self.typ.clone()
	}

	pub fn typ_ref(&self) -> &Type {
		&self.typ
	}

	pub fn typ_mut(&mut self) -> &mut Type {
		&mut self.typ
	}

	pub fn kind(&self) -> &ExprKind {
		&self.kind
	}

	pub fn kind_mut(&mut self) -> &mut ExprKind {
		&mut self.kind
	}

	pub fn source(&self) -> Option<Source> {
		self.source.clone()
	}

	pub fn set_source(&mut self, s: Option<Source>) {
		self.source = s;
	}
}

impl FuncArg {
	pub fn new(value: Expr, label: Option<String>) -> Self {
		FuncArg {
			label,
			value,
		}
	}

	pub fn into_value(self) -> Expr {
		self.value
	}

	pub fn into_label(self) -> Option<String> {
		self.label
	}

	pub fn value(&self) -> &Expr {
		&self.value
	}

	pub fn value_mut(&mut self) -> &mut Expr {
		&mut self.value
	}
}

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::None => write!(f, "!!!error!!!"),
			Self::Unit => write!(f, "()"),
			Self::Tuple(items) => {
				let items = items
					.iter()
					.map(|item| item.to_string())
					.collect::<Vec<_>>()
					.join(", ");
				
					write!(f, "({items})")
			}

			Self::IntLiteral(i) => write!(f, "{i}"),
			Self::FloatLiteral(fl) => write!(f, "{fl}"),
			Self::StringLiteral(s) => write!(f, "\"{s}\""),

			Self::Named(name) => write!(f, "{name}"),

			Self::FixOperator(_, op) => write!(f, "{op}"),
			Self::InfixOperator(.., op) => write!(f, "{op}"),

			Self::FuncCall { func, args } => {
				let args = args
					.iter()
					.map(|arg| arg.to_string())
					.collect::<Vec<_>>()
					.join(", ");
				
				write!(f, "{}({})", func, args)
			}

			Self::IntrinsicFunc(name) => {
				write!(f, "{name}")
			}

			Self::Function(func) => {
				write!(f, "{}", func.name())
			}

			Self::Select { branches, finally } => {
				writeln!(f, "select {{")?;
				for branch in branches {
					writeln!(f, "    {}", branch.to_string().replace("\n", "\n    "))?;
				}
				if let Some(finally) = finally {
					writeln!(f, "    _ => {{\n        {}\n    }}", finally.to_string().replace("\n", "\n    "))?;
				}
				write!(f, "}}")
			}

			Self::FunctionParameter(idx) => write!(f, "%par{idx}"),
			Self::LocalVariable(var) => write!(f, "%{var}"),

			_ => write!(f, "unknown")
		}
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} as {})", self.kind, self.typ)
    }
}

impl Display for FuncArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.label {
			write!(f, "{}: {}", label, self.value)
		} else {
			write!(f, "{}", self.value)
		}
    }
}