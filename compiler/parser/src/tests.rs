use errors::{DiagnosticReporter, fileinterner::FileInterner};

use crate::{operators::OperatorFactory, parser::{test}};

#[test]
fn test_code() {
    let mut interner = FileInterner::new();
    let mut operator_factory = OperatorFactory::new();
    operator_factory.register_intrinsics();

    interner.open_file("../../test/patterns/match.bolt", "test");

    let mut debugger = DiagnosticReporter::new(&interner);
}

// Static
// Imports
