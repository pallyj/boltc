use crate::{visibility::Visibility, ty::Type, attribute::Attribute};
use blir::code::FuncParam;
use itertools::Itertools;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Function {
	/// Doc comments on the function
	pub meta: String,

	/// Attributes applied to the function
	pub attributes: Vec<Attribute>,

	/// The function's visibility in its scope
	pub visibility: Visibility,

	/// The name of the function
	pub name: String,

	/// Parameters
	pub params: Vec<FunctionParameter>,

	/// Return type
	pub return_type: Type,

	/// The reciever for the function
	/// 
	/// None if it is a static method or a function
	pub reciever: Option<Type>,

	/// Whether the function can mutate its reciever
	pub is_mutating: bool,

	/// Whether the function is an operator
	pub is_operator: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionParameter {
	pub is_shared: bool,
	pub label: String,
	pub bind_name: String,
	pub param_type: Type
}

impl Function {
	///
	/// Composes a document function from a compiler function
	/// 
	pub fn compose(function: &blir::code::FunctionRef) -> Option<Self> {
		let borrowed_func = function.borrow();

		if borrowed_func.info.is_hidden() {
			return None
		}

		let meta = borrowed_func.comment.clone();
		let attributes = function.attributes().iter().map(Attribute::compose).collect_vec();
		let name = borrowed_func.info.name().clone();
		let visibility = Visibility::compose(borrowed_func.visibility);
		let params =
			borrowed_func.info
				.params()
				.iter()
				.map(|param| FunctionParameter::compose(param))
				.collect_vec();

		Some(Function {
			meta,
			attributes,
			visibility,
			name,
			params,
			return_type: Type::compose(borrowed_func.info.return_type()),
			reciever: None,
			is_mutating: false,
			is_operator: false
		})
	}

	pub fn compose_method(method: &blir::code::MethodRef) -> Option<Self> {
		let borrowed_func = method.borrow();

		if borrowed_func.info.is_hidden() {
			return None
		}

		let meta = borrowed_func.meta.clone();
		let attributes = method.attributes().iter().map(Attribute::compose).collect_vec();
		let name = borrowed_func.info.name().clone();
		let visibility = Visibility::compose(borrowed_func.visibility);
		let params =
			borrowed_func.info
				.params()
				.iter()
				.map(|param| FunctionParameter::compose(param))
				.collect_vec();

		let is_mutating = borrowed_func.is_mutating;
		let reciever = (!borrowed_func.is_static).then(|| Type::compose(&borrowed_func.self_type));

		Some(Function {
			meta,
			attributes,
			visibility,
			name,
			params,
			return_type: Type::compose(borrowed_func.info.return_type()),
			reciever,
			is_mutating,
			is_operator: method.is_operator()
		})
	}

	///
	/// Composes a document function from a compiler function
	/// 
	pub fn compose_extern(function: &blir::code::ExternFunctionRef) -> Option<Self> {
		let borrowed_func = function.borrow();

		if borrowed_func.info.is_hidden() {
			return None
		}

		let meta = borrowed_func.meta.clone();
		let attributes = function.attributes().iter().map(Attribute::compose).collect_vec();
		let name = borrowed_func.info.name().clone();
		let visibility = Visibility::compose(borrowed_func.visibility);
		let params =
			borrowed_func.info
				.params()
				.iter()
				.map(|param| FunctionParameter::compose(param))
				.collect_vec();

		Some(Function {
			meta,
			attributes,
			visibility,
			name,
			params,
			return_type: Type::compose(borrowed_func.info.return_type()),
			reciever: None,
			is_mutating: false,
			is_operator: false
		})
	}
}

impl FunctionParameter {
	///
	/// Composes a function parameter from a function
	/// 
	pub fn compose(param: &FuncParam) -> Self {
		let is_shared = param.is_shared;
		let label = param.label.clone().unwrap_or_else(|| String::from("_"));
		let bind_name = param.bind_name.clone();
		
		FunctionParameter {
			is_shared,
			label,
			bind_name,
			param_type: Type::compose(&param.typ),
		}
	}
}