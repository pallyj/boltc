use crate::AstLowerer;
use parser::ast::expr::{Expr as AstExpr, LiteralKind, IfExpr, IfExprNegative};
use blir::value::{Value, ValueKind, FunctionArgs, IfValue, IfBranch};

impl AstLowerer {
	pub (crate) fn lower_expr(&self, expr: AstExpr) -> Value {
		let range = expr.range();
		let span = self.span(range);

		match expr {
			AstExpr::NamedExpr(named) => ValueKind::Named(named.name()).spanned_infer(span),

			AstExpr::MemberExpr(member_expr) => {
				ValueKind::Member {
					parent: Box::new(self.lower_expr(member_expr.parent())),
					member: member_expr.child().unwrap()
				}.spanned_infer(span)
			}

			AstExpr::LiteralExpr(literal) => {
				let text = literal.text().replace("_", "");

				match literal.literal_kind() {
					LiteralKind::True => ValueKind::BoolLiteral(true).spanned_infer(span),
					LiteralKind::False => ValueKind::BoolLiteral(false).spanned_infer(span),
					// TODO: Add parsing
					LiteralKind::DecInteger => ValueKind::IntLiteral(u64::from_str_radix(&text, 10).unwrap()).spanned_infer(span),
					LiteralKind::HexInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 16).unwrap()).spanned_infer(span),
					LiteralKind::OctInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 8).unwrap()).spanned_infer(span),
					LiteralKind::BinInteger => ValueKind::IntLiteral(u64::from_str_radix(&text[2..], 2).unwrap()).spanned_infer(span),

					LiteralKind::DecFloat => ValueKind::FloatLiteral(fast_float::parse(&text).unwrap()).spanned_infer(span),

					_ => ValueKind::BoolLiteral(true).spanned_infer(span),
				}
			}

			AstExpr::ParenthesizedExpr(paren) => {
				// TODO: Add old span

				self.lower_expr(paren.expr())
			}

			AstExpr::FuncCallExpr(call) => {
				let func = self.lower_expr(call.function());

				let args = call.args().into_iter()
					.map(|arg| self.lower_expr(arg))
					.collect();

				ValueKind::FuncCall {
					function: Box::new(func),
					args: FunctionArgs { args }
				}.spanned_infer(span)
			}

			AstExpr::IfExpr(expr) => {
				let if_value = self.lower_if_expr(expr);

				ValueKind::If(if_value).spanned_infer(span)
			}

			AstExpr::Error => panic!()
		}
	}

	pub (crate) fn lower_if_expr(&self, expr: IfExpr) -> IfValue {
		let condition = Box::new(self.lower_expr(expr.condition()));
		let positive = self.lower_code_block(expr.positive());
		let negative = match expr.negative() {
			Some(IfExprNegative::CodeBlock(cb)) => Some(IfBranch::CodeBlock(self.lower_code_block(cb))),
			Some(IfExprNegative::IfExpr(else_if)) => Some(IfBranch::Else(Box::new(self.lower_if_expr(else_if)))),
			_ => None
		};

		IfValue {
			condition,
			positive,
			negative,
		}
	}
}