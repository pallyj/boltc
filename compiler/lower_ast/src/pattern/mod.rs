use blir::{pattern::{Pattern, PatternKind}, value::ValueKind};
use parser::ast::pattern::Pattern as AstPattern;

use crate::AstLowerer;

impl AstLowerer {
    pub(crate) fn lower_pattern(&self, expr: AstPattern) -> Pattern {
		let span = self.span(expr.range());

		match expr {
			AstPattern::WildcardPattern(_) => PatternKind::Wildcard,
			AstPattern::LiteralPattern(literal) => {
				let value = self.lower_expr(literal.value());

				PatternKind::Literal { value }
			}
			AstPattern::VariantPattern(variant) => {
				let value = ValueKind::VariantLiteral(variant.variant_name())
					.spanned_infer(span);

				PatternKind::Literal { value }
			}

			AstPattern::Error => panic!(),
		}.with_span(span)
	}
}