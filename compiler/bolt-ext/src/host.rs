use crate::{operator::Operator, attribute::Attribute};


/// The bridge between the compiler and extensions
pub trait ExtensionHost {
	///
	/// Registers a new operator with the bolt language
	/// 
	/// host.register_operator(Operator { name: "coalesce",
	/// 								  symbol: "??",
	/// 								  fix: Fix::Infix,
	/// 								  precedence: Precedence::Add })
	/// 
	fn register_operator(
		&mut self,
		operator: Operator);

	
	///
	/// Registers a new attribute with the bolt language
	/// 
	/// host.register_attribute(Box::new(TransparentAttribute::default()))
	/// 
	fn register_attribute(
		&mut self,
		attribute: Box<dyn Attribute>);
}