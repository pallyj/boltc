pub enum Expression {
	Named(String),

	IntLit(u64),
	FloatLit(f64),
	StringLit(String),

	InfixOp(Expression, String, Expression),
	PostfixOp(Expression, String),
	PrefixOp(Expression, String),

	Convert(Expression, Type)
}