use errors::{Span, IntoDiagnostic, Diagnostic, DiagnosticLevel, CodeLocation};
use rowan::TextRange;

pub (crate) enum Error {
	FeatureNotEnabled(&'static str),
	Unicode(String),
	NotAnInteger,
	BreakOutsideLoop,
	ContinueOutsideLoop,

	NegativeArrayLength(u64),
	LibraryDoesNotExist(String),

	NoTrailingClosure,
}

pub (crate) struct SpannedError {
	error: Error,
	span: Span,
}

impl Error {
	pub fn at(self, span: Span) -> SpannedError {
		SpannedError { error: self, span }
	}

	pub fn at_raw(self, span: (usize, (usize, usize))) -> SpannedError {
		let file = span.0 as u32;
		let range_lo = span.1.0 as u32;
		let range_hi = span.1.1 as u32;

		SpannedError { error: self, span: Span::new(TextRange::new(range_lo.into(), range_hi.into()), file) }
	}
}

impl IntoDiagnostic for SpannedError {
    fn into_diagnostic(self) -> Diagnostic {
        match self.error {
			Error::FeatureNotEnabled(feature) => {
				Diagnostic::new(DiagnosticLevel::Error,
							 	"feature_not_enabled",
								format!("feature `{feature}` is not enabled"),
								vec![ CodeLocation::new(self.span, Some(format!("try passing the flag `--feature {feature}`"))) ])
			}

			Error::Unicode(error) => {
				Diagnostic::new(DiagnosticLevel::Error,
								"unicode",
								error,
								vec![ CodeLocation::new(self.span, None) ])
			}

			Error::NotAnInteger => {
				Diagnostic::new(DiagnosticLevel::Error,
								"not_an_integer",
							 	String::from("value cannot be coerced to an integer"),
								vec![ CodeLocation::new(self.span, None) ])
			}

			Error::BreakOutsideLoop => {
				Diagnostic::new(DiagnosticLevel::Error,
								"flow_outside_loop",
								String::from("break can't be used outside a loop"),
								vec![ CodeLocation::new(self.span, None) ])
			}

			Error::ContinueOutsideLoop => {
				Diagnostic::new(DiagnosticLevel::Error,
					"flow_outside_loop",
					String::from("continue can't be used outside a loop"),
					vec![ CodeLocation::new(self.span, None) ])
			}

			Error::NoTrailingClosure => {
				Diagnostic::new(DiagnosticLevel::Error,
								"trailing_non_func",
								String::from("cannot used trailing closure with non-function"),
								vec![ CodeLocation::new(self.span, None) ])
			}

			Error::NegativeArrayLength(len) => {
				Diagnostic::new(DiagnosticLevel::Error,
								"negative_arr_len",
								format!("found array with negative length -{len}"),
								vec![ CodeLocation::new(self.span, None) ])
			}

			Error::LibraryDoesNotExist(library) => {
				Diagnostic::new(DiagnosticLevel::Error,
								"library_dne",
								format!("couldn't find library {library}"),
								vec![ CodeLocation::new(self.span, Some("did you mean to import this library?".into())) ])
			}
		}
    }
}