use prelude::WithSource;

#[derive(Debug)]
pub enum Type {
	Named(String),

	Struct(Option<String>, Vec<Box<WithSource<TypeDecl>>>),

	Union(Option<String>, Vec<Box<WithSource<TypeDecl>>>),

	StructRef(String),
	Enum(String),
	UnionRef(String),

	Intrinsic(String),
	Pointer(Box<WithSource<Type>>),
	Const(Box<WithSource<Type>>),

	Array(Box<WithSource<Type>>, u64),
	FuncPtr(Box<WithSource<Type>>, Vec<Box<WithSource<TypeDecl>>>),

	Unit,
}

#[derive(Debug)]
pub struct TypeDecl {
	pub typ: Type,
	pub name: Option<String>
}