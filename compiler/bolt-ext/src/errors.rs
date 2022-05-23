/// Provides error handling for plugins
pub trait Debugger {
	/// Throws a warning for this structs.
	/// The warning will be placed at the name of the struct
	/// 
	/// debugger.warn("struct name is not UpperCamelCase")
	/// 
	fn warn(
		&mut self,
		warning: &str);

	/// Throws an error for the struct
	/// The error will be placed at the name of the struct
	/// 
	/// debugger.throw("struct is not complete")
	/// 
	fn throw(
		&mut self, 
		error: &str);
}