use prelude::WithSource;

use crate::FuncArg;

/// @ `attribute_name` ( func_args )
#[derive(Debug, Clone)]
pub struct Attribute {
	name: String,

	args: Vec<WithSource<FuncArg>>
}

impl Attribute {
	/// Creates a new named attribute without arguments
	pub fn new(name: String, args: Vec<WithSource<FuncArg>>) -> Self {
		Self {
			name: name,
			args,
		}
	}
}

// Attribute::new("link")
//     .pass_arg(FuncArg::no_label(StringLiteral::new("")))