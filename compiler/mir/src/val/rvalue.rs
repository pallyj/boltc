use std::fmt::Display;

use errors::Span;
use itertools::Itertools;

use crate::{ty::{Type, TypeKind}, code::{Function, ExternFunction}};

use super::{place::Place, PlaceKind, SoloIntrinsic, DuoIntrinsic};

///
/// A constant value
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum ConstValue {
	///
	/// An integer constant. Negatives are represented with two's complement
	/// 
	Integer(u64),

	///
	/// A float constant
	/// 
	Float(f64),

	///
	/// A string constant
	/// 
	String(String),	
}

#[derive(Debug, Clone, PartialEq)]
pub enum RValueKind {
	///
	/// An integer, float, or string literal
	/// 
	Const(ConstValue),

	///
	/// Moves the value out of the place
	/// 
	Move(Box<Place>),

	/// 
	/// Copies the value from the place
	/// 
	Copy(Box<Place>),

	///
	/// Gets a pointer to the value in the place
	/// 
	Ref(Box<Place>),

	// Struct
	// Enum
	// NOTE: Remove undefined from the specification

	///
	/// Calls a function and yield the return value of the function
	/// 
	Call {
		function: 	Box<RValue>,
		params: 	Vec<RValue>,
	},

	///
	/// Forms a constant tuple
	/// 
	Tuple {
		items: 		Vec<RValue>,
	},

	/// 
	/// Get a function with a name
	/// 
	Function {
		is_extern: 	bool,
		name: 		String,
	},

	/// 
	/// A LLVM intrinsic with one operand
	/// 
	SoloIntrinsic {
		intrinsic: 	SoloIntrinsic,
		operand: 	Box<RValue>
	},

	/// 
	/// A LLVM intrinsic with two operands
	/// 
	DuoIntrinsic {
		intrinsic: 	DuoIntrinsic,
		left: 		Box<RValue>,
		right: 		Box<RValue>
	}
}

///
/// A RValue with a kind, type, and a span
/// 
#[derive(Debug, Clone, PartialEq)]
pub struct RValue {
	pub (super) kind: RValueKind,
	pub (super) ty: Type,
	pub (super) span: Span
}

impl RValue {
	///
	/// Returns a constant integer
	/// 
	pub fn const_int(n: u64, ty: Type, span: Span) -> Self {
		RValue { kind: RValueKind::Const(ConstValue::Integer(n)), ty, span }
	}

	///
	/// Returns a constant float
	/// 

	pub fn const_float(n: f64, ty: Type, span: Span) -> Self {
		RValue { kind: RValueKind::Const(ConstValue::Float(n)), ty, span }
	}

	///
	/// Returns a constant string
	/// 
	pub fn const_string(s: &str, ty: Type, span: Span) -> Self {
		RValue { kind: RValueKind::Const(ConstValue::String(s.to_string())), ty, span }
	}

	///
	/// Calls this value as a functions, with `params` as parameters
	/// 
	pub fn call(&self, params: Vec<RValue>, span: Span) -> Self {
		if let TypeKind::Function { return_type, .. } = self.ty.kind() {
			let return_type = (&**return_type).clone();

			RValue { kind: RValueKind::Call { function: Box::new(self.clone()), params },
					 ty: return_type,
					 span }

		} else {
			panic!()
		}
	}

	///
	/// Creates a tuple with the items `items`
	/// 
	pub fn tuple(items: Vec<RValue>, span: Span) -> Self {
		let tuple_type = Type::tuple(items.iter().map(|item| item.ty.clone()).collect_vec());
		RValue { kind: RValueKind::Tuple { items }, ty: tuple_type, span }
	}

	///
	/// Calls a single param intrinsic
	/// 
	pub fn intrinsic(intrinsic: SoloIntrinsic, operand: RValue, span: Span) -> Self {
		let ty = intrinsic.output_type(&operand.ty);
		
		RValue { kind: RValueKind::SoloIntrinsic { intrinsic, operand: Box::new(operand) }, ty, span }
	}

	/// 
	/// Calls a double param intrinsic
	/// 
	pub fn intrinsic2(intrinsic: DuoIntrinsic, left: RValue, right: RValue, span: Span) -> Self {
		let ty = intrinsic.output_type(&left.ty);

		RValue { kind: RValueKind::DuoIntrinsic { intrinsic,
												  left: Box::new(left),
												  right: Box::new(right) }, ty, span }
	}

	///
	/// 
	/// 
	pub fn function(func: &Function, span: Span) -> RValue {
		RValue { kind: RValueKind::Function { is_extern: false, name: String::from(func.name()) }, ty: func.func_type(), span }
	}

	///
	/// 
	/// 
	pub fn extern_function(func: &ExternFunction, span: Span) -> RValue {
		RValue { kind: RValueKind::Function { is_extern: true, name: String::from(func.name()) }, ty: func.func_type(), span }
	}

	///
	/// Gets a place referring to the value of a pointer
	/// 
	pub fn deref(self, span: Span) -> Place {
		if let TypeKind::Pointer(ty) = self.ty.kind() {
			let ty = (**ty).clone();
			Place::new(PlaceKind::Deref(self), ty, true, span)
		} else {
			panic!("Can't deref a value of type {:?}", self.ty)
		}		
	}

	///
	/// What kind of value this is. Used to match it
	/// 
	pub fn kind(&self) -> &RValueKind {
		&self.kind
	}

	///
	/// The type of the RValue
	/// 
	pub fn ty(&self) -> Type {
		self.ty.clone()
	}

	///
	/// 
	/// 
	pub fn span(&self) -> Span {
		self.span
	}
}

impl Display for ConstValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::Integer(n) => write!(f, "{n}"),
			Self::Float(n) => write!(f, "{n}"),
			Self::String(s) => write!(f, "\"{s}\"")
		}
    }
}

impl Display for RValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			Self::Const(const_value) => write!(f, "const {const_value}"),

			Self::Move(place) => write!(f, "move {place}"),
			Self::Copy(place) => write!(f, "copy {place}"),
			Self::Ref(place) => write!(f, "shared {place}"),

			Self::Call { function, params } => write!(f, "call {function} ({params})", params = params.iter().format(", ")),

			Self::Tuple { items } => write!(f, "tuple ({items})", items = items.iter().format(", ")),
			Self::Function { is_extern, name } => write!(f, "{}function \"{name}\"", if *is_extern { "extern " } else { "" } ),

			Self::SoloIntrinsic { intrinsic, operand } => write!(f, "llvm.{intrinsic} {operand}"),
			Self::DuoIntrinsic { intrinsic, left, right } => write!(f, "llvm.{intrinsic} {left}, {right}"),
		}
    }
}

impl Display for RValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}