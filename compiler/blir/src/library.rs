use std::{sync::{Mutex, Arc, MutexGuard}, collections::HashMap, fmt::{Display}};

use crate::{sym::Symbol, func::FuncDef, structdef::StructDef, metadata::Metadata, Type, Visibility, SymbolKind, TypeKind, ExprKind, Expr, Scope, ScopeKind, FileScope};

pub struct Library {
	/// The library's name
	name: String,

	/// Metadata of the library
	metadata: Mutex<Metadata>,

	/// Symbols in the library
	symbols: Mutex<HashMap<String, Symbol>>,

	/// The structs in a library
	structs: Mutex<Vec<Arc<Mutex<StructDef>>>>,

	/// Global functions in a library
	funcs: Mutex<Vec<Arc<FuncDef>>>,

	files: Mutex<Vec<Arc<FileScope>>>
}

impl Library {
	/// Creates a new library
	pub fn new(name: String) -> Arc<Library> {
		let lib = Library {
			name,
			metadata: Mutex::new(Metadata::new()),
			symbols: Mutex::new(HashMap::new()),
			structs: Mutex::new(Vec::new()),
			funcs: Mutex::new(Vec::new()),
			files: Mutex::new(Vec::new())
		};

		Arc::new(lib)
	}

	/// 
	/// Adds metadata to the library
	/// 
	/// Common metadata keys are 
	/// 
	/// `description`
	/// `authors`
	/// `version`
	/// `comment`
	/// 
	pub fn insert_metadata(&self, key: String, value: String) {
		self.metadata
			.lock()
			.unwrap()
			.insert(key, value);
	}

	///
	/// Adds a doc comment to the library
	/// 
	pub fn doc_comment(&self, comment: &str) {
		self.metadata
			.lock()
			.unwrap()
			.doc_comment(comment);
	}

	///
	/// Defines a new symbol in the library's scope
	/// 
	/// # Example
	/// 
	/// func define_alias(lib: &blir::Library, alias: String, type: Type) {
	/// 	lib.define_symbol(alias, Symbol::Type(type))
	/// }
	/// 
	pub fn define_symbol(&self, key: String, sym: Symbol) {
		self.symbols
			.lock()
			.unwrap()
			.insert(key, sym);
	}

	///
	/// Does the library contain the symbol?
	/// 
	/// # Example
	/// 
	/// let lib = Library::new("FooBar");
	/// add_ast_to_lib(lib)
	/// 
	/// if lib.has_symbol("Foo") {
	/// 	println!("Foo")
	/// }
	/// 
	pub fn has_symbol(&self, key: &str) -> bool {
		self.symbols
			.lock()
			.unwrap()
			.contains_key(key)
	}

	/// 
	/// Gets a symbol in the library, or returns None if it doesn't exist
	/// 
	pub fn get_symbol(&self, key: &str) -> Option<Symbol> {
		self.symbols
			.lock()
			.unwrap()
			.get(key)
			.cloned()
	}

	pub fn add_file(&self, file: Arc<FileScope>) {
		self.files
			.lock()
			.unwrap()
			.push(file);
	}

	pub fn define_struct(&self, r#struct: Arc<Mutex<StructDef>>) {
		self.structs
			.lock()
			.unwrap()
			.push(r#struct.clone());

		// Something with visibility
		let name = r#struct.lock().unwrap().name().clone();

		self.symbols
			.lock()
			.unwrap()
			.insert(name, Symbol::new(SymbolKind::Type(Type::new_anon(TypeKind::StructRef(r#struct))), Visibility::Public));
	}

	pub fn define_function(&self, func: Arc<FuncDef>) {
		self.funcs
			.lock()
			.unwrap()
			.push(func.clone());

		let name = func.name().clone();

		let f = Expr::new_anon(ExprKind::Function(func), Type::new_anon(TypeKind::Infer(u64::MAX)));

		self.symbols
			.lock()
			.unwrap()
			.insert(name, Symbol::new(SymbolKind::Function(f), Visibility::Public));
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn funcs(&self) -> MutexGuard<Vec<Arc<FuncDef>>> {
		self.funcs.lock().unwrap()
	}
}

impl Scope for Library {
    fn parent(&self) -> Option<&dyn Scope> {
        None
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> crate::ScopeKind {
        ScopeKind::Library
    }

    fn lookup_symbol(&self, name: &String) -> Option<Symbol> {
        self.get_symbol(name)
    }

    fn define_expr(&self, name: String, value: Expr) {
        todo!()
    }
	
	fn scoped_type(&self, name: &str) -> Option<TypeKind> {
		None
	}

	fn take_index(&self) -> u64 {
		0
	}
}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "library {}\n", self.name)?;

		writeln!(f, "{:?}", self.metadata.lock().unwrap())?;

		/*for s in self.structs.lock().unwrap().iter() {
			writeln!(f, "{:?}", s.lock().unwrap())?;
		}*/

		for func in self.funcs.lock().unwrap().iter() {
			writeln!(f, "{}", func)?;
		}

		Ok(())
    }
}