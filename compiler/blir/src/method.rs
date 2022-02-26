use std::{sync::Weak, fmt::Debug};

use prelude::Source;

use crate::{Visibility, scope::Scope, func::FuncParam, typ::Type};

#[derive(Clone)]
pub struct MethodDef {
	/// The type the method takes as a reciever
	reciever: Weak<dyn Scope>,

	// TODO: Attributes

	/// The visibility of the method in its reciever
	visibility: Visibility,

	/// Whether the method is static
	is_static: bool,

	/// Whether the method takes a mutable reciever
	is_mutating: bool,

	/// The name of the method
	name: String,

	// TODO: Generic Parameters

	/// Parameters to the method
	parameters: Vec<FuncParam>,

	/// The return type of the method
	return_type: Type,

	// TODO: Code

	/// The source code defining the function
	source: Option<Source>,
}

#[derive(Clone)]
pub struct FunctionPrototypeDef {
	is_static: bool,
	is_mutating: bool,

	name: String,

	// TODO: Generic Parameters

	/// Parameters to the method
	parameters: Vec<FuncParam>,

	/// The return type of the method
	return_type: Type,

	/// The source code defining the function
	source: Option<Source>,
}