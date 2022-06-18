use rowan::TextRange;

use crate::{fileinterner::FileInterner, diag::{DiagnosticReporter, CodeLocation}, Span, DiagnosticLevel, IntoDiagnostic, Diagnostic};

#[test]
fn print_loc_error() {
	let test = r#"
	fn main() {
		print("Hello, World")
	}"#;


	let mut interner = FileInterner::new();
	interner.test_code(test);

	let reporter = DiagnosticReporter::new(&interner);

	let range = TextRange::new(17.into(), 22.into());
	reporter.print_span(&CodeLocation {
		suggestion: Some(String::from("Add a semicolon")),
		span: Span::new(range, 0),
	}, DiagnosticLevel::Error, false);
}

#[test]
fn print_loc_warning() {
	let test = r#"
	fn main() {
		print("Hello, World")
	}"#;


	let mut interner = FileInterner::new();
	interner.test_code(test);

	let reporter = DiagnosticReporter::new(&interner);

	let range = TextRange::new(17.into(), 22.into());
	reporter.print_span(&CodeLocation {
		suggestion: Some(String::from("Add a semicolon")),
		span: Span::new(range, 0),
	}, DiagnosticLevel::Warning, false);
}

#[test]
fn print_loc_info() {
	let test = r#"
	fn main() {
		print("Hello, World")
	}"#;


	let mut interner = FileInterner::new();
	interner.test_code(test);

	let reporter = DiagnosticReporter::new(&interner);

	let range = TextRange::new(17.into(), 22.into());
	reporter.print_span(&CodeLocation {
		suggestion: Some(String::from("Add a semicolon")),
		span: Span::new(range, 0),
	}, DiagnosticLevel::Info, false);
}

#[test]
fn print_error() {
	let test = r#"
	fn main() {
		print("Hello, World")
	}"#;


	let mut interner = FileInterner::new();
	interner.test_code(test);

	let reporter = DiagnosticReporter::new(&interner);

	let range = TextRange::new(17.into(), 22.into());
	reporter.throw_diagnostic(TestError::FunctionNotFound("print", range));
}

#[test]
fn print_warnings() {
	let test = r#"
	fn main() {
		return

		print("Hello, World")
	}"#;


	let mut interner = FileInterner::new();
	interner.test_code(test);

	let reporter = DiagnosticReporter::new(&interner);

	let range = TextRange::new(17.into(), 23.into());
	let range2 = TextRange::new(27.into(), 48.into());
	reporter.throw_diagnostic(TestError::DeadCode(range, vec![range2]));
}

enum TestError {
	FunctionNotFound(&'static str, TextRange),
	DeadCode(TextRange, Vec<TextRange>)
}

impl IntoDiagnostic for TestError {
    fn into_diagnostic(self) -> crate::Diagnostic {
        match self {
			Self::FunctionNotFound(name, range) => {
				Diagnostic::new(DiagnosticLevel::Error, "func_not_found", format!("function `{name}` not found in scope: label not found"), vec![ CodeLocation::new(Span { range, file: 0 }, Some(String::from("maybe this function can be found in another library. try importing it"))) ])
			}
			Self::DeadCode(terminator, rest) => {
				Diagnostic::new(DiagnosticLevel::Warning,
								"unreachable_statement",
								format!("unreachable statement"),
								std::iter::once(CodeLocation::new(Span { range: terminator, file: 0 }, Some(String::from("any code after this expression is unreachable"))))
								.chain(rest.iter().map(|loc| CodeLocation::new( Span { range: loc.clone(), file: 0 }, Some(String::from("unreachable statement")) )))
								.collect()
								)
			}
		}
    }
}