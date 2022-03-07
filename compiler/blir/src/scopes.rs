use std::sync::{Arc, Mutex, Weak};

use crate::{Scope, Library, Intrinsics, Metadata, TypeKind};

pub struct FileScope {
	imports: Mutex<Vec<Arc<dyn Scope>>>,
	library: Weak<Library>,
	_metadata: Metadata,
}

impl FileScope {
	pub fn new(library: &Arc<Library>) -> Arc<FileScope> {
		Arc::new(
			FileScope {
				imports: Mutex::new(vec![]),
				library: Arc::downgrade(library),
				_metadata: Metadata::new(),
			}
		)
	}

	pub fn library(&self) -> Arc<Library> {
		self.library.upgrade().unwrap()
	}

	pub fn import(&self, import_name: &String) {
		let import = match import_name.as_str() {
			"intrinsics" => {
				let mut intrinsics = Intrinsics::new();
				intrinsics.populate();
				Some(Arc::new(intrinsics))
			}
			/*
			"lang" => {

			}
			*/
			_ => {
				None
			}
		};

		if let Some(import) = import {
			self.imports.lock().unwrap().push(import);
		}
	}
}

impl Scope for FileScope {
    fn parent(&self) -> Option<Arc<dyn Scope>> {
        let scope: Arc<dyn Scope> = self.library.upgrade().unwrap();

		Some(scope)
    }

    fn name(&self) -> &str {
        ""
    }

	fn symbol(&self) -> mangle::symbol::Symbol {
        self.parent().unwrap().symbol()
    }

    fn lookup_symbol(&self, name: &String) -> Option<crate::Symbol> {
        if let Some(sym) = self.library().lookup_symbol(name) {
			Some(sym)
		} else {
			for import in self.imports.lock().unwrap().iter() {
				if let Some(sym) = import.lookup_symbol(name) {
					return Some(sym)
				}
			}

			None
		}
    }

    fn define_expr(&self, _name: String, _value: crate::Expr) {
        todo!()
    }

	fn scoped_type(&self, _name: &str) -> Option<TypeKind> {
		None
	}

	fn take_index(&self) -> u64 {
		0
	}
}