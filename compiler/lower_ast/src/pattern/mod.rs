use blir::{pattern::{Pattern, PatternKind}, value::ValueKind, typ::TypeKind};
use parser::ast::pattern::Pattern as AstPattern;

use crate::AstLowerer;

impl<'a, 'b> AstLowerer<'a, 'b> {

	pub(crate) fn lower_pattern(&mut self, expr: AstPattern) -> Pattern {
		self.lower_pattern_inner(expr, false)
	}
    fn lower_pattern_inner(&mut self, expr: AstPattern, varying: bool) -> Pattern {
		let span = self.span(expr.range());

		match expr {
			AstPattern::WildcardPattern(_) => PatternKind::Wildcard,
			AstPattern::LiteralPattern(literal) => {
				let lowered = self.lower_expr(literal.value(), None);

				match &lowered.kind {
					ValueKind::StringLiteral(_) => {
						PatternKind::Literal { value: lowered }
					}

					ValueKind::BoolLiteral(_) => {
						PatternKind::Literal { value: lowered }
					}

					_ => {
						let value = Self::change_to_u64(self.lower_integer(&lowered));

						PatternKind::Integer { value }
					}
				}
				
			}
			AstPattern::VariantPattern(variant) => {
				let value = ValueKind::VariantLiteral(variant.variant_name())
					.spanned_infer(span);

				if let Some(assoc) = variant.associated_patterns() {
					let (items, labels) = assoc.map(|pat| (self.lower_pattern_inner(pat.pattern(), varying), pat.label()))
											   .unzip();
					
					PatternKind::Variant { variant: value, items, labels }
				} else {
					PatternKind::Literal { value }
				}
			}
			AstPattern::TuplePattern(tuple_pat) => {
				let (sub_patterns, labels): (Vec<_>, Vec<_>) = tuple_pat
					.tuple_items()
					.map(|tuple_item| {
						let pattern = self.lower_pattern_inner(tuple_item.pattern(), varying);
						(pattern, tuple_item.label())
					})
					.unzip();

				let types = sub_patterns.iter()
									    .map(|pat| pat.match_type().clone())
										.collect::<Vec<_>>();

				let tuple_type = TypeKind::Tuple(types, labels.clone()).spanned(span);

				return PatternKind::Tuple { items: sub_patterns, labels }.with(span, tuple_type);
			}
			AstPattern::BindPattern(bind_pattern) => {
				PatternKind::Bind(bind_pattern.bind_name(), varying)
			}
			AstPattern::VaryingPattern(varying_pattern) => {
				return self.lower_pattern_inner(varying_pattern.subpattern(), true)
			}

			AstPattern::Error => panic!(),
		}.with_span(span)
	}
}