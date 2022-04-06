#[test]
fn test_code() {
    let interner = FileInterner::new();
    let mut debugger = Debugger::new(&interner);

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
	if a.lt(b) {
		return gcd(b, a)
	}

	if a.mod(b).eq(0) {
		return b
	}

	return gcd(b, a.mod(b))
}
	"#;

    let mut lib = Library::new("");

    AstLowerer::new(parse(&code, &mut debugger, 0)).lower_file(&mut lib);

    println!("{lib:?}");
}

// Static
// Imports

use blir::Library;
use errors::{debugger::Debugger, fileinterner::FileInterner};
use parser::parser::parse;

use crate::AstLowerer;
