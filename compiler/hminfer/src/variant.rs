use blir::typ::StructRef;
use rusttyc::{Variant, Partial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeVariant {
	Top,

	SomeInteger,
	SomeBoolean,
	SomeFloat,

	IntrinsicInteger { bits: u64 },
	IntrinsicBool,
	IntrinsicFloat { bits: u64 },

	Struct(StructRef),

	Void,
	Diverges,

	Error
}

impl Variant for TypeVariant {
    type Err = String;

    fn top() -> Self {
        Self::Top
    }

    fn meet(lhs: rusttyc::Partial<Self>, rhs: rusttyc::Partial<Self>) -> Result<rusttyc::Partial<Self>, Self::Err> {
		let variant = match (lhs.variant, rhs.variant) {
			(Self::Top, x) | (x, Self::Top) => Ok(x),

			(Self::SomeInteger, Self::IntrinsicInteger { bits }) | (Self::IntrinsicInteger { bits }, Self::SomeInteger) => Ok(Self::IntrinsicInteger { bits }),
			(Self::SomeFloat, Self::IntrinsicFloat { bits }) | (Self::IntrinsicFloat { bits }, Self::SomeFloat) => Ok(Self::IntrinsicFloat { bits }),
			(Self::SomeBoolean, Self::IntrinsicBool) | (Self::IntrinsicBool, Self::SomeBoolean) => Ok(Self::IntrinsicBool),

			(Self::SomeInteger, Self::Struct(integer_struct)) | (Self::Struct(integer_struct), Self::SomeInteger) => {
				if integer_struct.integer_repr() {
					Ok(Self::Struct(integer_struct))
				} else {
					Err("error".to_string())
				}
			}

			(Self::SomeBoolean, Self::Struct(bool_struct)) | (Self::Struct(bool_struct), Self::SomeBoolean) => {
				if bool_struct.bool_repr() {
					Ok(Self::Struct(bool_struct))
				} else {
					Err("error".to_string())
				}
			}

			(Self::SomeFloat, Self::Struct(float_struct)) | (Self::Struct(float_struct), Self::SomeFloat) => {
				if float_struct.float_repr() {
					Ok(Self::Struct(float_struct))
				} else {
					Err("error".to_string())
				}
			}

			(_, Self::Error) | (Self::Error, _) => Ok(Self::Error),

			(x, y) if x == y => Ok(x),
			_ => Err("error".to_string())
		}?;

		Ok(Partial { variant, least_arity: 0 })
    }

    fn arity(&self) -> rusttyc::Arity {
        rusttyc::Arity::Fixed(0)
    }
}