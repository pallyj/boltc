use crate::symbol::{Symbol, SymbolKind};

#[test]
pub fn test_demangle() {
	println!("{:?}", Symbol::parse_list("3L4S5F3langInt64add"));
}

#[test]
pub fn test_mangle() {
	let sym = Symbol::new(SymbolKind::Library("lang".to_string()))
		.append(SymbolKind::Struct("Int64".to_string()))
		.append(SymbolKind::Function("add".to_string()));

	println!("{}", sym.mangle());
}