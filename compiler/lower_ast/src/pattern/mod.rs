use blir::{pattern::{Pattern, PatternKind}, value::ValueKind};
use parser::ast::pattern::Pattern as AstPattern;

use crate::AstLowerer;

impl<'a, 'b> AstLowerer<'a, 'b> {
    pub(crate) fn lower_pattern(&mut self, expr: AstPattern) -> Pattern {
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

				if let Some(assoc) = variant.associated_patterns() {
					PatternKind::Variant { variant: value, items: assoc.map(|pat| self.lower_pattern(pat)).collect() }
				} else {
					PatternKind::Literal { value }
				}
			}
			AstPattern::TuplePattern(tuple_pat) => {
				let sub_patterns = tuple_pat
					.tuple_items()
					.map(|tuple_item| self.lower_pattern(tuple_item))
					.collect();

				PatternKind::Tuple { items: sub_patterns }
			}
			AstPattern::BindPattern(bind_pattern) => {
				PatternKind::Bind(bind_pattern.bind_name())
			}

			AstPattern::Error => panic!(),
		}.with_span(span)
	}
}