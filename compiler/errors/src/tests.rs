use rowan::TextRange;

use crate::{fileinterner::FileInterner, diag::{DiagnosticReporter, CodeLocation}, Span};

#[test]
fn print_loc() {
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
	});
}