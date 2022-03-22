#[test]
fn test_code() {
	let mut parser = Parser::new(r#"
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
	"#);

	parser.operator_factory().register_intrinsics();

	let mut lib = Library::new("");

	AstLowerer::new(parser.parse_file())
		.lower_file(&mut lib);

	println!("{lib:?}");
}

// Static
// Imports

use blir::Library;
use parser::{parser::Parser};

use crate::AstLowerer;