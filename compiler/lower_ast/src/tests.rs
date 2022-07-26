use std::collections::HashMap;

use blir::Library;
use errors::{fileinterner::FileInterner, DiagnosticReporter};
use parser::{operators::OperatorFactory, parser::parse};

use crate::AstLowerer;


#[test]
fn test_code() {
    let interner = FileInterner::new();
    let mut debugger = DiagnosticReporter::new(&interner);
    let mut operator_factory = OperatorFactory::new();
    operator_factory.register_intrinsics();

    let code = r#"
import intrinsics

struct Int64 {
	var repr: i64

	func add(b: Int64): Int64 {

	}

	func sub(b: Int64): Int64 {

	}
}

func gcd(a: Int64, b: Int64): Int64 {
	if a < b { gcd(b, a) }
	else if a % b == 0 { b }
	else { gcd(b, a % b) }
}
	"#;

    let mut lib = Library::new("");
	let imports = HashMap::new();

    AstLowerer::new(parse(&code, &mut debugger, 0, &operator_factory), &mut debugger, &operator_factory, &imports).lower_file(&mut lib);

    eprintln!("{lib:?}");
}

// Static
// Imports