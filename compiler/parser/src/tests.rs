use errors::{debugger::Debugger, fileinterner::FileInterner};

use crate::parser::parse;

#[test]
fn test_code() {
    let mut interner = FileInterner::new();

	interner.open_file("../../test/closure/test.bolt");

    let mut debugger = Debugger::new(&interner);

	for file in interner.iter() {
		let block = parse(file.1.text(), &mut debugger, file.0);

		println!("{:?}", block);

		//let block = Root::cast(block.root).unwrap();
	}
}

// Static
// Imports
