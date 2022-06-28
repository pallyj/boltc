use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Type {
	Void,
	Divergent,
	Function {
		parameters: Vec<Type>, 
		return_type: Box<Type>
	},
	Tuple(Vec<(Option<String>, Type)>),

	Integer(u32),
	Float(u32),
	RawPointer(Box<Type>),

	Temp(String),
	Path(String, Vec<String>)
}

impl Type {
	pub fn compose(from: &blir::typ::Type) -> Type {
		use blir::typ::TypeKind;

		match from.kind() {
			TypeKind::Void => Type::Void,
			TypeKind::Divergent => Type::Divergent,
			TypeKind::Function { return_type, params, .. } => Type::Function {
				parameters: params.iter().map(Type::compose).collect(),
				return_type: Box::new(Type::compose(return_type))
			},
			TypeKind::Method { reciever, return_type, params } =>  Type::Function {
				parameters: std::iter::once(reciever.as_ref()).chain(params).map(Type::compose).collect(),
				return_type: Box::new(Type::compose(return_type))
			},
			TypeKind::Struct(struct_ref) => {
				let name = struct_ref.name();
				let path = struct_ref.borrow().path().clone().into_components();

				Type::Path(name, path)
			}
			TypeKind::Enum(enum_ref) => {
				let name = enum_ref.name().to_string();
				let path = enum_ref.path().clone().into_components();

				Type::Path(name, path)
			}
			TypeKind::Tuple(ty, labels) => Type::Tuple(labels.iter().cloned().zip(ty.iter().map(Self::compose)).collect()),
			TypeKind::Integer { bits } => Type::Integer(*bits as u32),
			TypeKind::Float { bits } => Type::Float(*bits as u32),
			TypeKind::RawPointer { pointer_type } => Type::RawPointer(Box::new(Type::compose(pointer_type))),

			_ => panic!(),
		}
	}
}