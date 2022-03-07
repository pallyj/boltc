use std::{collections::HashMap, sync::{Arc, Mutex, Weak, MutexGuard}, fmt::Display};

use prelude::Source;

use crate::{typ::Type, Visibility, method::MethodDef, var::VariableDef, Symbol, SymbolKind, TypeKind, Scope};

pub struct StructDef {
	// Attributes
	
	visibility: Visibility,

	name: String,

	link_name: Mutex<String>,

	static_symbols: Mutex<HashMap<String, Symbol>>,

	instance_symbols: Mutex<HashMap<String, Symbol>>,

	implements: Vec<Type>,

	variables: Mutex<Vec<Arc<VariableDef>>>,

	methods: Mutex<Vec<Arc<MethodDef>>>,

	substructs: Mutex<Vec<Arc<StructDef>>>,

	parent: Weak<dyn Scope>,

	me: Weak<StructDef>,
}

impl StructDef {
	pub fn new(parent: &Arc<dyn Scope>, visibility: Visibility, name: String, source: Source) -> Arc<StructDef> {
		Arc::new_cyclic(|me| {
			Self {
				visibility,
				name: name.clone(),
				link_name: Mutex::new(name),
				implements: vec![],
				static_symbols: Mutex::new(HashMap::new()),
				instance_symbols: Mutex::new(HashMap::new()),
				variables: Mutex::new(Vec::new()),
				methods: Mutex::new(Vec::new()),
				substructs: Mutex::new(Vec::new()),
				parent: Arc::downgrade(parent),
				me: me.clone()
			}
		})
	}

	pub fn visibility(&self) -> Visibility {
		self.visibility.clone()
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn link_name(&self) -> MutexGuard<String> {
		self.link_name
			.lock()
			.unwrap()
	}

	pub fn set_link_name(&self, name: String) {
		*self.link_name
			.lock()
			.unwrap() = name;
	}

	pub fn add_substruct(&self, substruct: Arc<StructDef>) -> Result<(), ()> {
		let name = substruct.name().clone();
		//let visibility = substruct.visibility();
		let visibility = Visibility::Public;

		self.substructs
			.lock()
			.unwrap()
			.push(substruct.clone());

		let struct_type = Type::new_anon(TypeKind::StructRef(substruct));
		let struct_sym = Symbol::new(SymbolKind::Type(struct_type), visibility);

		if self.static_symbols
			.lock()
			.unwrap()
			.insert(name, struct_sym).is_some() {
				Err(())
			} else {
				Ok(())
			}
	}

	pub fn add_method(&self, method: Arc<MethodDef>) -> Result<(), ()> {
		let name = method.name().clone();
		//let visibility = substruct.visibility();
		let visibility = Visibility::Public;

		self.methods
			.lock()
			.unwrap()
			.push(method.clone());

		if method.is_static() {
			let method_sym = Symbol::new(SymbolKind::StaticMethod(method), visibility);

			if self.static_symbols
				.lock()
				.unwrap()
				.insert(name, method_sym).is_some() {
					Err(())
				} else {
					Ok(())
				}
		} else {
			let method_sym = Symbol::new(SymbolKind::InstanceMethod(method), visibility);

			if self.instance_symbols
				.lock()
				.unwrap()
				.insert(name, method_sym).is_some() {
					Err(())
				} else {
					Ok(())
				}
		}
	}

	pub fn add_variable(&self, variable: Arc<VariableDef>) -> Result<(), ()> {
		let name = variable.name().clone();
		let visibility = Visibility::Public;

		let mut variables = self.variables();

		variable.set_field_index(variables.len());
		variables.push(variable.clone());

		let var_sym = Symbol::new(SymbolKind::InstanceVariable(variable), visibility);

		if self.instance_symbols
			.lock()
			.unwrap()
			.insert(name, var_sym).is_some() {
				Err(())
			} else {
				Ok(())
			}
	}

	pub fn substructs(&self) -> MutexGuard<Vec<Arc<StructDef>>> {
		self.substructs.lock().unwrap()
	}

	pub fn methods(&self) -> MutexGuard<Vec<Arc<MethodDef>>> {
		self.methods.lock().unwrap()
	}

	pub fn variables(&self) -> MutexGuard<Vec<Arc<VariableDef>>> {
		self.variables.lock().unwrap()
	}

	pub fn lookup_static_member(&self, name: &String) -> Option<Symbol> {
		self.static_symbols
			.lock()
			.unwrap()
			.get(name)
			.cloned()
	}

	pub fn lookup_instance_member(&self, name: &String) -> Option<Symbol> {
		self.instance_symbols
			.lock()
			.unwrap()
			.get(name)
			.cloned()
	}
}

impl Scope for StructDef {
    fn parent(&self) -> Option<Arc<dyn Scope>> {
        self.parent.upgrade()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn symbol(&self) -> mangle::symbol::Symbol {
		self.parent()
			.unwrap()
			.symbol()
			.append(mangle::symbol::SymbolKind::Struct(self.name.clone()))
    }

    fn lookup_symbol(&self, name: &String) -> Option<Symbol> {
		if name == "Self" {
			return Some(Symbol::new(SymbolKind::Type(TypeKind::StructRef(self.me.upgrade().unwrap()).anon()), Visibility::Public))
		}

		// Look for a static item
		// Instance method will implement intance items themselves

        self.parent().unwrap().lookup_symbol(name)
    }

    fn define_expr(&self, _name: String, _value: crate::Expr) {
        todo!()
    }

    fn scoped_type(&self, _name: &str) -> Option<TypeKind> {
        None
    }

    fn take_index(&self) -> u64 {
        todo!()
    }
}

impl Display for StructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} struct {} {{", self.visibility, self.name)?;

		for substruct in self.substructs.lock().unwrap().iter() {
			writeln!(f, "\t{}", substruct.to_string().replace("\n", "\n\t"))?;
		}

		for method in self.methods.lock().unwrap().iter() {
			writeln!(f, "\t{}", method.to_string().replace("\n", "\n\t"))?;
		}

		write!(f, "}}")
    }
}