use crate::mangled::{MangleComponent, Mangled};

#[test]
pub fn test_demangle() {
    println!("{:?}", Mangled::parse_list("3L4S5F3langInt64add"));
}

#[test]
pub fn test_mangle() {
    let sym = Mangled::new(MangleComponent::Library("lang".to_string())).append(MangleComponent::Struct("Int64".to_string()))
                                                                        .append(MangleComponent::Function("add".to_string()));

    println!("{}", sym.mangle());
}
