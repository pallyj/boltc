use crate::parser::Parser;

#[test]
fn test_empty_file() {
	let parser = Parser::new("String.Int");

	println!("{:#?}", parser.parse_file());
}