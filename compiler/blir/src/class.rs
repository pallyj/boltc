use crate::typ::Type;

#[derive(Clone)]
pub struct ClassDef {
	// Add attributes

	// Add visibility

	name: String,

	// Add generic params

	/// The class this class inherits from
	inherits: Option<Type>,

	/// Protocols this class implements
	implements: Vec<Type>,

	variables: (),

	properties: (),

	methods: (),

	static_methods: (),

}