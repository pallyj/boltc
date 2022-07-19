use blir::typ::Type;
use errors::{Span, IntoDiagnostic, Diagnostic, CodeLocation};

pub enum LoweringErrorKind
{
	CantSwitchOn(Type),
	ImmutableReciever(Span),
	ImmutablePlace,
	ImmutableParam,
	InvalidIndex(Type),

	NoPatternInLet,
	SplitPatternInLet,
	RefutablePatternInLet,


	LoopDoesNotExist,
}

pub struct LoweringError
{
	kind: LoweringErrorKind,
	span: Span
}

impl LoweringErrorKind
{
	pub fn with_span(self, span: Span) -> LoweringError
	{
		LoweringError { kind: self, span }
	}
}

impl IntoDiagnostic for LoweringError
{
    fn into_diagnostic(self) -> errors::Diagnostic {
        match self.kind
		{
			LoweringErrorKind::CantSwitchOn(ty) => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"cant_switch_on",
								format!("can't switch on value of type `{ty}`"),
								vec![ CodeLocation::new(self.span, Some("try adding an implementation of `==` for this type".into())) ])
			}
			LoweringErrorKind::ImmutableReciever(method_span) => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"immut_reciever",
								format!("can't call a mutating method on an immutable value"),
								vec![ CodeLocation::new(self.span, None),
									  CodeLocation::new(method_span, Some("did you mean to make this method mutating?".into())) ])
			}
			LoweringErrorKind::ImmutablePlace => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"immut_place",
								format!("can't assign to an immutable variable"),
								vec![ CodeLocation::new(self.span, None) ])
			}
			LoweringErrorKind::ImmutableParam => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"immut_param",
								format!("only mutable variables can be shared"),
								vec![ CodeLocation::new(self.span, None) ])
			}
			LoweringErrorKind::InvalidIndex(ty) => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"invalid_index",
								format!("value of type `{ty}` can't be used as an array index"),
								vec![ CodeLocation::new(self.span, None) ])
			},
			LoweringErrorKind::NoPatternInLet => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"no_let_pat",
								format!("bind statement doesn't have a pattern"),
								vec![ CodeLocation::new(self.span, None) ])
			}
			LoweringErrorKind::SplitPatternInLet => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"split_let_pat",
								format!("bind statement's pattern splits"),
								vec![ CodeLocation::new(self.span, None) ])
			}
			LoweringErrorKind::RefutablePatternInLet => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"refutable_pat",
								format!("can't bind to a refutable pattern"),
								vec![ CodeLocation::new(self.span, Some("use a guard let ... else {} clause".into())) ])
			}
			LoweringErrorKind::LoopDoesNotExist => {
				Diagnostic::new(errors::DiagnosticLevel::Error,
								"loop_dne",
								format!("loop not found"),
								vec![ CodeLocation::new(self.span, None) ])
			}
		}
    }
}