use std::fmt::Display;

use blir::Type;

#[derive(Clone)]
pub enum Constraint {
	Absolute(Type),
	Suggestion(Type),

	Equality(u64),

	SomeInteger,
	SomeFloat,
	SomeString,
	SomeCollection,
	SomeRecord,
	SomeVariant
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u32)]
pub enum ConstraintPriority {
	Absolute = 3,
	NeedsSuggestion = 2,
	Suggestion = 1,
}

impl Constraint {
	pub fn priority(&self) -> ConstraintPriority {
		match self {
			Self::Absolute(_) => ConstraintPriority::Absolute,
			Self::Suggestion(_) => ConstraintPriority::Suggestion,

			_ => ConstraintPriority::NeedsSuggestion,
		}
	}
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::SomeInteger => write!(f, "integer"),
			Self::SomeFloat => write!(f, "float"),
			Self::SomeString => write!(f, "string"),
			Self::SomeRecord => write!(f, "record"),
			Self::SomeCollection => write!(f, "collection"),
			Self::SomeVariant => write!(f, "variant"),
			Self::Suggestion(ty) => write!(f, "suggest {}", ty),
			Self::Equality(ctx) => write!(f, "equals {ctx}"),
			Self::Absolute(ty) => write!(f, "equals {ty}"),
		}
    }
}