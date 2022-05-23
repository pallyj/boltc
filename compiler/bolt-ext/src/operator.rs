/// Represents the fix of an operator
/// 
/// Prefix operators are placed before their operand,
/// postfix operators are after, and infix operators are
/// between their two operands.
pub enum Fix {
	Prefix,
	Infix,
	Postfix
}

/// Represents the precedence group of an operator
/// 
/// Precedence groups are ordered by how tightly they bind values
/// The highest precedence is ::Intrinsic and the lowest is ::Assignment
#[repr(u32)]
pub enum Precedence {
	Intrinsic      = 1100,
    Postfix        = 1000,
    Prefix         = 900,
    Exponential    = 800,
    Multiplicative = 700,
    Additive       = 600,
    Secondary      = 500,
    Comparison     = 400,
    And            = 300,
    Or             = 200,
    Assignment     = 100,
}

/// A declaration of a new operator
/// 
/// The operator has a name for overloading, a symbol,
/// a fix, and a precedence
/// 
/// The name must be a valid identifier and the symbol must be a valid operator
/// 
pub struct Operator {
	pub name: &'static str,
	pub symbol: &'static str,
	pub fix: Fix,
	pub precedence: Precedence
}