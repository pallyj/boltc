use serde::{Serialize, Deserialize};
use super::{typ::Type};

#[derive(Serialize, Deserialize)]
pub struct TypeAlias {
	/// The generic parameters in the type alias
	//generic_params: Vec<GenericParameter>,

	/// The type this type alias is set to
	typ: Type,
}