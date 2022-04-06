pub mod typ;
pub mod value;
pub mod code;
pub mod intrinsics;
pub mod scope;
pub mod attributes;

mod sym;
mod library;
mod visibility;

pub use sym::*;
pub use library::*;
use typ::StructRef;
pub use visibility::*;

pub struct BlirContext {
	pub default_integer_repr: Option<StructRef>,
	pub default_float_repr: Option<StructRef>,
	pub default_bool_repr: Option<StructRef>,

	pub entry_point: Option<String>
}

impl BlirContext {
	pub fn new() -> Self {
		Self {
			default_integer_repr: None,
			default_float_repr: None,
			default_bool_repr: None,

			entry_point: None
		}
	}
}