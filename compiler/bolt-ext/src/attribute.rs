use crate::Debugger;

/// Provides behavior for applying an attribute to a function or a type definition
pub trait Attribute {
	// Gets the label the attribute can be identified by
	fn label(&self) -> &'static str;

	/// Applys an attribute to a function signature, with optional error handling
	fn apply_to_func(&mut self,
					 func: FunctionSignature,
					 debugger: &mut dyn Debugger);
}

/// Represents the signature of a function or method
/// 
/// This provides information about a function and can be changed
pub struct FunctionSignature<'a> {
	inline: &'a mut bool,
	name: &'a String,
	link_name: &'a mut String,
	kind: FunctionKind,
	n_par: usize,
}

/// Describes what kind of function this is
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FunctionKind {
	Function,
	ExternFunction,
	StaticMethod,
	Method,
	Operator,
	Initializer
}

impl<'a> FunctionSignature<'a> {
	/// Marks the function as inlining
	pub fn set_inline(&mut self) {
		*self.inline = true;
	}

	/// Gets the name of the function
	pub fn name<'b>(&'b self) -> &'b String {
		self.name
	}

	/// Gets the link name of the function
	pub fn link_name<'b>(&'b self) -> &'b String {
		self.link_name
	}

	/// Sets the link name of the function
	/// 
	/// let function_name = String::from(function.name);
	/// function.set_link_name(function_name);
	/// 
	pub fn set_link_name(&mut self, name: String) {
		*self.link_name = name;
	}

	/// Gets what kind of function this is
	pub fn kind(&self) -> FunctionKind {
		self.kind
	}

	/// The number of parameters the function has
	pub fn params_num(&self) -> usize {
		self.n_par
	}

	pub fn new(
		inline: &'a mut bool,
		name: &'a String,
		link_name: &'a mut String,
		kind: FunctionKind,
		params_num: usize) -> Self
	{
		Self { inline,
			   name,
			   link_name,
			   kind,
			   n_par: params_num }
	}
}