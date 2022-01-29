use serde::{Serialize, Deserialize};
use super::{Visibility, typ::Type};

#[derive(Serialize, Deserialize)]
pub struct Function {
	/// Any documentation attached to the function
	description: String,

	/// Attributes of the function
	// attributes: Vec<Attribute>,
	
	/// The visibility of the symbol
	/// 
	/// `Public` - Visible outside the library
	/// `Internal` - Visibile inside the file
	/// `Private` - Not visible outside the file
	/// 
	visibility: Visibility,

	/// The name of the function
	name: String,

	/// The symbol corresponding with the function
	/// 
	/// This field enables ffi between languages
	symbol_name: String,

	// generics: Vec<GenericParameter>

	/// The parameters of the function
	parameters: Vec<FunctionParameter>,

	/// The return type of the function
	return_type: Type,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionParameter {
	// attributes: Vec<Attribute>

	/// The label for the parameter
	label_name: String,

	/// The variable this parameter is bound to
	bind_name: String,

	/// The type of the parameter
	typ: Type,
}