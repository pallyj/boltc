use std::{sync::{Weak, Mutex, Arc}, collections::HashMap};

use blir::{Scope, Symbol, SymbolKind, Visibility, TypeKind};

pub struct CodeBlockScope {
	parent: Weak<dyn Scope>,
	syms: Mutex<HashMap<String, Symbol>>
}

impl CodeBlockScope {
    pub fn new(parent: &Arc<dyn Scope>) -> Arc<CodeBlockScope> {
        Arc::new(
            CodeBlockScope {
                parent: Arc::downgrade(parent),
                syms: Mutex::new(HashMap::new())
            }
        )
    }
}

impl Scope for CodeBlockScope {
    fn parent(&self) -> Option<Arc<dyn Scope>> {
		self.parent.upgrade()
    }

    fn name(&self) -> &str {
        "code_block"
    }

    fn lookup_symbol(&self, name: &String) -> Option<blir::Symbol> {
		if let Some(sym) = self.syms.lock().unwrap().get(name) {
			return Some(sym.clone())
		}

        self.parent
			.upgrade()
			.unwrap()
			.lookup_symbol(name)
    }

    fn define_expr(&self, name: String, value: blir::Expr) {
        self.syms
			.lock()
			.unwrap()
			.insert(name, Symbol::new(SymbolKind::Value(value), Visibility::Public));
    }

    fn scoped_type(&self, name: &str) -> Option<TypeKind> {
        self.parent().unwrap().scoped_type(name)
    }

    fn take_index(&self) -> u64 {
        self.parent.upgrade().unwrap().take_index()
    }

    fn symbol(&self) -> mangle::symbol::Symbol {
        self.parent().unwrap().symbol()
    }
}