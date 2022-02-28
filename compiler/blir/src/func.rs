use std::{sync::{Weak, Arc, Mutex, MutexGuard, atomic::{AtomicU64, Ordering}}, fmt::Display};
use prelude::*;
use crate::{typ::Type, CodeBlock, Scope, ScopeKind, Symbol, SymbolKind, Expr, ExprKind, Visibility, TypeKind};

pub struct FuncDef {
	// TODO: Add attributes

	/// The name of the function
	name: String,

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

impl FuncDef {
	/// 
	/// Creates a new FuncDef in `library` named `name` accepting parameters `parameters`, and returning `return_type`
	/// 
	/// # Example
	/// 
	/// let lib = Library::new("FooBar");
	/// let foo: Arc<Mutex<FuncDef>> = FuncDef::new("foo".to_string(), vec![], Type::Unit, &lib);
	/// 
	pub fn new(name: String, parameters: Vec<FuncParam>, return_type: Type, code: CodeBlock, parent: &Arc<dyn Scope>) -> Arc<FuncDef> {
		Arc::new(
			FuncDef {
				name,

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
		FuncSig::new(self.parameters.lock().unwrap().iter().map(|par| par.typ.clone()).collect(), self.return_type.lock().unwrap().clone())
	}

	pub fn parent(&self) -> Arc<dyn Scope> {
		self.parent.upgrade().unwrap()
	}
}

impl Scope for FuncDef {
    fn parent(&self) -> Option<&dyn Scope> {
		None
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn kind(&self) -> crate::ScopeKind {
        ScopeKind::Function
    }

    fn lookup_symbol(&self, name: &String) -> Option<crate::Symbol> {
        for (i, param) in self.params().iter().enumerate() {
			if &param.bind_name == name {
				let typ = param.typ.clone();

				let expr = Expr::new_anon(ExprKind::FunctionParameter(i), typ);

				return Some(Symbol::new(SymbolKind::Value(expr), Visibility::Public))
			}
		}

		self.parent().lookup_symbol(name)
    }

    fn define_expr(&self, name: String, value: Expr) {
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

#[derive(Clone)]
pub struct FuncParam {
	label: Option<String>,
	bind_name: String,
	typ: Type,
}

impl FuncParam {
	pub fn new(label: Option<String>, bind_name: String, typ: Type) -> FuncParam {
		FuncParam {
			label,
			bind_name,
			typ
		}
	}

	pub fn typ(&self) -> &Type {
		&self.typ
	}

	pub fn typ_mut(&mut self) -> &mut Type {
		&mut self.typ
	}
}

#[derive(Clone)]
pub struct FuncSig {
	parameters: Vec<Type>,
	return_type: Type
}

impl FuncSig {
	pub fn new(parameters: Vec<Type>, return_type: Type) -> Self {
		Self {
			parameters,
			return_type
		}
	}

	pub fn parameters(&self) -> &Vec<Type> {
		&self.parameters
	}

	pub fn parameters_mut(&mut self) -> &mut Vec<Type> {
		&mut self.parameters
	}

	pub fn return_type(&self) -> &Type {
		&self.return_type
	}

	pub fn return_type_mut(&mut self) -> &mut Type {
		&mut self.return_type
	}
}

impl Display for FuncSig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pars = self.parameters
			.iter()
			.map(|p| p.to_string())
			.collect::<Vec<_>>()
			.join(", ");

		let return_type = &self.return_type;

		write!(f, "({pars}): {return_type}")
    }
}

impl Display for FuncParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.label {
			write!(f, "{} ", label)?;
		}

		write!(f, "{}: {}", self.bind_name, self.typ)
    }
}

impl Display for FuncDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self.params().iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");

		writeln!(f, "func {}({}): {} {{", self.name, params, self.return_type.lock().unwrap())?;

		writeln!(f, "\t{}", self.code.lock().unwrap().to_string().replace("\n", "\n\t"))?;

		write!(f, "}}")
    }
}