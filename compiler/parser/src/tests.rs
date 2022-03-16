use crate::{parser::Parser, ast::{Root}};


#[test]
fn test_code() {
	let mut parser = Parser::new(r#"
	func gcd(a: Int, b: Int): Int {
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

	let block = parser.parse_file();

	println!("{:?}", block);

	let block = Root::cast(block.root).unwrap();
	println!("{block:?}");
}

// Static
// Imports