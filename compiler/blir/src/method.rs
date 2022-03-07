use std::{sync::{Weak, Mutex, atomic::{AtomicU64, Ordering}, Arc, MutexGuard}, fmt::Display};

use prelude::Source;

use crate::{Visibility, scope::Scope, func::FuncParam, typ::Type, CodeBlock, FuncSig, Expr, ExprKind, Symbol, SymbolKind, TypeKind};

pub struct MethodDef {
	// TODO: Add attributes

	/// The name of the function
	name: String,

	link_name: Mutex<String>,

	// TODO: Add generic parameters

	/// Parameters to the function
	parameters: Mutex<Vec<FuncParam>>,

	/// The type this function returns
	return_type: Mutex<Type>,

	code: Mutex<CodeBlock>,

	/// The source code defining the function
	source: Option<Source>,

	/// The scope the function is defined in
	parent: Weak<dyn Scope>,

	index: AtomicU64,
}

impl MethodDef {
	/// 
	/// Creates a new MethodDef in `library` named `name` accepting parameters `parameters`, and returning `return_type`
	/// 
	/// # Example
	/// 
	/// let lib = Library::new("FooBar");
	/// let foo: Arc<Mutex<MethodDef>> = MethodDef::new("foo".to_string(), vec![], Type::Unit, &lib);
	/// 
	pub fn new(name: String, parameters: Vec<FuncParam>, return_type: Type, code: CodeBlock, parent: &Arc<dyn Scope>) -> Arc<MethodDef> {
		Arc::new(
			MethodDef {
				name: name.clone(),
				link_name: Mutex::new(name),

				parameters: Mutex::new(parameters),
				return_type: Mutex::new(return_type),

				code: Mutex::new(code),

				source: None,
				parent: Arc::downgrade(parent),

				index: AtomicU64::new(1),
			}
		)
	}

	/// 
	/// Sets the source of the function in source code
	/// 
	pub fn set_source(&mut self, source: Source) {
		self.source = Some(source)
	}

	///
	/// The function's name
	/// 
	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn link_name(&self) -> MutexGuard<String> {
		self.link_name
			.lock()
			.unwrap()
	}

	pub fn set_link_name(&self, link_name: String) {
		*self.link_name
			.lock()
			.unwrap() = link_name
	}

	/// 
	/// The function's parameters
	/// 
	pub fn params(&self) -> MutexGuard<Vec<FuncParam>> {
		self.parameters.lock().unwrap()
	}

	///
	/// The function's return type
	/// 
	pub fn return_type(&self) -> MutexGuard<Type> {
		self.return_type.lock().unwrap()
	}

	pub fn code(&self) -> MutexGuard<CodeBlock> {
		self.code.lock().unwrap()
	}

	pub fn signature(&self) -> FuncSig {
		FuncSig::new(self.parameters.lock().unwrap().iter().map(|par| par.typ().clone()).collect(), self.return_type.lock().unwrap().clone())
	}

	pub fn parent(&self) -> Arc<dyn Scope> {
		self.parent.upgrade().unwrap()
	}
}

impl Scope for MethodDef {
    fn parent(&self) -> Option<Arc<dyn Scope>> {
		self.parent.upgrade()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn symbol(&self) -> mangle::symbol::Symbol {
		self.parent()
			.symbol()
			.append(mangle::symbol::SymbolKind::Function(self.name().clone()))
	}

    fn lookup_symbol(&self, name: &String) -> Option<crate::Symbol> {
        for (i, param) in self.params().iter().enumerate() {
			if param.bind_name() == name {
				let typ = param.typ().clone();

				let expr = Expr::new_anon(ExprKind::FunctionParameter(i), typ);

				return Some(Symbol::new(SymbolKind::Value(expr), Visibility::Public))
			}
		}

		self.parent().lookup_symbol(name)
    }

    fn define_expr(&self, _name: String, _value: Expr) {
        todo!()
    }

	fn scoped_type(&self, name: &str) -> Option<TypeKind> {
		if name == "return-type" {
			Some(self.return_type.lock().unwrap().kind().clone())
		} else {
			self.parent().scoped_type(name)
		}
	}

	fn take_index(&self) -> u64 {
		self.index.fetch_add(1, Ordering::Relaxed)
	}
}

/*
#[derive(Clone)]
pub struct FunctionPrototypeDef {
	is_static: bool,
	is_mutating: bool,

	name: String,

	// TODO: Generic Parameters

	/// Parameters to the method
	parameters: Vec<FuncParam>,

	/// The return type of the method
	return_type: Type,

	/// The source code defining the function
	source: Option<Source>,
}*/

impl Display for MethodDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self.params().iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");

		writeln!(f, "func {}({}): {} {{", self.name, params, self.return_type.lock().unwrap())?;

		writeln!(f, "\t{}", self.code.lock().unwrap().to_string().replace("\n", "\n\t"))?;

		write!(f, "}}")
    }
}