use blir::typ::{StructRef, TypeKind};
use rusttyc::{Variant, Partial, Constructable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeVariant {
	Unconstrained,

	SomeInteger,
	SomeFloat,
	SomeBool,
	SomeFunction,

	LlvmInt { bits: u32 },
	LlvmFloat { bits: u32 },
	LlvmBool,

	Struct(StructRef),

	// TODO: Add function type
	Function,

	Void,
	Diverges,

	Error
}

impl Variant for TypeVariant {
	type Err = String;

	fn top() -> Self { Self::Unconstrained }

	fn meet(
		lhs: rusttyc::Partial<Self>,
		rhs: rusttyc::Partial<Self>)
	-> Result<rusttyc::Partial<Self>, Self::Err>
	{
        let variant = match (lhs.variant, rhs.variant) {
			(Self::Unconstrained, x) | (x, Self::Unconstrained) => Ok(x),

			(Self::Diverges, x) | (x, Self::Diverges) => Ok(x),

			(Self::SomeInteger, Self::LlvmInt { bits }) |
			(Self::LlvmInt { bits }, Self::SomeInteger)
				=> Ok(Self::LlvmInt { bits }),

			(Self::SomeFloat, Self::LlvmFloat { bits }) |
			(Self::LlvmFloat { bits }, Self::SomeFloat)
				=> Ok(Self::LlvmFloat { bits }),

			(Self::SomeBool, Self::LlvmBool) |
			(Self::LlvmBool, Self::SomeBool)
				=> Ok(Self::LlvmBool),

			(Self::SomeInteger, Self::Struct(integer_struct)) |
			(Self::Struct(integer_struct), Self::SomeInteger) => {
				if integer_struct.integer_repr() {
					Ok(Self::Struct(integer_struct))
				} else {
					Err(format!("struct '{}' is not representable by an integer", integer_struct.name()))
				}
			}

			(Self::SomeFloat, Self::Struct(float_struct)) |
			(Self::Struct(float_struct), Self::SomeFloat) => {
                if float_struct.float_repr() {
                    Ok(Self::Struct(float_struct))
                } else {
                    Err(format!("struct '{}' is not representable by a float", float_struct.name()))
                }
            }

			(Self::SomeBool, Self::Struct(bool_struct)) |
			(Self::Struct(bool_struct), Self::SomeBool) => {
				if bool_struct.bool_repr() {
					Ok(Self::Struct(bool_struct))
				} else {
					Err(format!("struct '{}' is not representable by a boolean", bool_struct.name()))
				}
			}

			(_, Self::Error) | (Self::Error, _) => Ok(Self::Error),

			(x, y) if x == y => Ok(x),
			_ => Err("types don't match".to_string()),


		}?;

		Ok(Partial { variant, least_arity: 0 })
    }

	fn arity(&self) -> rusttyc::Arity {
        rusttyc::Arity::Fixed(0)
    }
}

impl Constructable for TypeVariant {
    type Type = TypeKind;

    fn construct(&self, _children: &[Self::Type]) -> Result<Self::Type, <Self as rusttyc::ContextSensitiveVariant>::Err> {
        todo!()
    }
}