use errors::{IntoDiagnostic, Diagnostic, DiagnosticLevel, CodeLocation, Span};
use rowan::TextRange;

pub struct ParseError {
	error: String,
	span: (usize, (usize, usize))
}

impl ParseError {
	pub fn new(message: String, span: (usize, (usize, usize))) -> Self {
		Self { error: message, span }
	}
}

impl IntoDiagnostic for ParseError {
    fn into_diagnostic(self) -> Diagnostic {
		let file = self.span.0 as u32;
		let (range_lo, range_hi) = self.span.1;
		let text_range = TextRange::new((range_lo as u32).into(), (range_hi as u32).into());

		let span = Span::new(text_range, file);
		let loc = CodeLocation::new(span, None);

        Diagnostic::new(DiagnosticLevel::Error, "parser", self.error, vec![loc] )
    }
}