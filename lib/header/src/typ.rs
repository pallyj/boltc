use serde::{Serialize, Deserialize};
use super::{function::FunctionParameter};

#[derive(Serialize, Deserialize)]
pub enum Type {
	/// A type built in to the language
	Intrinsic(String),
	
	/// A reference to a struct in a library
	StructRef { library: String, name: String },

	/// A reference to a class in a library
	ClassRef { library: String, name: String },

	/// A reference to an enum in a library
	EnumRef { library: String, name: String },

	/// A reference to a union in a library
	UnionRef { library: String, name: String },

	/// Adds generic parameters to a type
	/// 
	/// Base<T, U>
	/// 
	Generic { base: Box<Type>, generic_args: Vec<GenericArg> },

	/// An empty type
	/// 
	/// () in bolt
	/// void in C
	/// 
	Unit,

	/// A raw pointer
	/// 
	/// RawPointer<T> in bolt
	/// const T * in bolt
	/// 
	ConstPtr(Box<Type>),

	/// A mutable raw pointer
	/// 
	/// MutRawPointer<T> in bolt
	/// T * in bolt
	/// 
	MutPtr(Box<Type>),

	/// A fixed size array of a type
	Array { unit: Box<Type>, count: usize },

	/// A function
	Function(Box<FunctionPrototype>),
}

#[derive(Serialize, Deserialize)]
pub struct FunctionPrototype {
	// generics: Vec<GenericParameter>

	/// The parameters of the function
	parameters: Vec<FunctionParameter>,

	/// The return type of the function
	return_type: Type,
}

#[derive(Serialize, Deserialize)]
pub enum GenericArg {
	/// A normal generic parameter
	///   i.e. Vec<T>
	Ordered(Type),

	/// A named generic parameter
	///   i.e. Iterator<Item=T>
	Named { name: String, typ: Type }
}