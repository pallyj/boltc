pub trait GenericToken: Clone + PartialEq {
	fn eof() -> Self;
}