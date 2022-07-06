use blir::{attributes::{Attribute, Attributes, AttributeArg, AttributeArgs}};
use errors::{IntoDiagnostic, Span, Diagnostic, DiagnosticLevel, CodeLocation};

use crate::AstLowerer;

impl<'a, 'b> AstLowerer<'a, 'b> {
    pub fn lower_attribute(&mut self, attribute: &parser::ast::attribute::Attribute) -> Attribute {
        let span = self.span(attribute.range());

        let args = attribute.args()
            .map(|args|
            args.filter_map(|arg| {
                let label = arg.label();
                let value = self.lower_expr(arg.value(), None);
                let arg_val = match &value.kind {
                    blir::value::ValueKind::Named(name) => AttributeArg::Named(name.clone()),
                    blir::value::ValueKind::IntLiteral(n) => AttributeArg::Integer(*n),
                    blir::value::ValueKind::FloatLiteral(n) => AttributeArg::Float(*n),
                    blir::value::ValueKind::BoolLiteral(b) => AttributeArg::Bool(*b),
                    blir::value::ValueKind::StringLiteral(s) => AttributeArg::String(s.clone()),
                    blir::value::ValueKind::VariantLiteral(v) => AttributeArg::Variant(v.clone()),

                    _ => {
                        self.reporter.throw_diagnostic(NonAttribute(value.span.unwrap()));
                        return None
                    }
                };

                Some((label, arg_val))
            })
            .collect()).unwrap_or_else(|| Vec::new());

        Attribute::new(attribute.attribute_name(), AttributeArgs::new(args), span)
    }

    pub fn lower_attributes(&mut self, attributes: parser::ast::attribute::Attributes) -> Attributes {
        let attributes = attributes.list()
                                   .map(|attribute| self.lower_attribute(&attribute));

        Attributes::new(attributes)
    }
}

struct NonAttribute(Span);

impl IntoDiagnostic for NonAttribute {
    fn into_diagnostic(self) -> errors::Diagnostic {
        Diagnostic::new(DiagnosticLevel::Error,
                        "not_a_literal",
                        String::from("only literals can be used as an attribute args"),
                        vec![ CodeLocation::new(self.0, Some(String::from("not a literal"))) ])
    }
}