use crate::{ty::Type, Project};

///
/// Refers to a function in a project
/// 
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternFunctionId {
	id: usize,
}

///
/// Represents a function, with a name, parameters, a return_type and a collection of `BasicBlocks`
/// 
pub struct ExternFunction {
	id: ExternFunctionId,

	name: String,

	params: Vec<Type>,
	return_type: Type,
}

impl ExternFunction {
	///
	/// Creates a new function
	/// 
	pub (crate) fn new(id: ExternFunctionId, name: &str, params: Vec<Type>, return_type: Type) -> ExternFunction {
		ExternFunction {
			id,
			name: name.to_string(),
			params,
			return_type }
	}

	///
	/// The unique identifier of a function
	/// 
	pub fn id(&self) -> ExternFunctionId {
		self.id
	}

	///
	/// Gets the unique, mangled name of a function
	/// 
	pub fn name(&self) -> &str {
		&self.name
	}

	///
	/// The parameters a function takes
	/// 
	pub fn params(&self) -> &[Type] {
		&self.params
	}

	///
	/// The type a function returns
	/// 
	pub fn return_type(&self) -> &Type {
		&self.return_type
	}

	///
	/// 
	/// 
	pub fn func_type(&self) -> Type {
		self.return_type.clone().func(self.params.clone())
	}

	///
	/// Writes a function to a Formatter
	/// 
	/// Takes a project parameter for access to the basic blocks in the function, necessitating
	/// the use of a custom function instead of Display
	/// 
	#[allow(unstable_name_collisions)]
	pub (crate) fn write(&self, f: &mut std::fmt::Formatter, project: &Project) -> std::fmt::Result {
		write!(f, "func {} (", self.name())?;

		self.params().iter()
					 .enumerate()
					 .map(|(i, item)| { write!(f, "_{i}: ")?; item.write(f, project)?; write!(f, ", ")})	
					 .collect::<std::fmt::Result>()?;

		write!(f, ") -> ")?; 

		self.return_type.write(f, project)?;

		writeln!(f)
	}
}

impl ExternFunctionId {
	///
	/// Creates a new `FunctionId`
	/// 
	pub (crate) fn new(id: usize) -> Self {
		ExternFunctionId { id }
	}

	///
	/// The unique identifier of the `FunctionId`
	/// 
	pub (crate) fn unique_idx(&self) -> usize {
		self.id
	}
}