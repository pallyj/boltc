use std::{collections::HashMap, ops::Deref, sync::Mutex};

use inkwell::{module::Module, builder::Builder, context::Context, values::{FunctionValue, AnyValueEnum}, passes::PassManager, types::BasicTypeEnum};

pub struct TypeContainer<'ctx> {
	types: Mutex<HashMap<String, BasicTypeEnum<'ctx>>>,
}

impl<'ctx> TypeContainer<'ctx> {
	pub fn new() -> Self {
		Self {
			types: Mutex::new(HashMap::new())
		}
	}

	pub fn get_type(&self, name: &str) -> BasicTypeEnum<'ctx> {
		self.types.lock().unwrap().get(name).unwrap().clone()
	}

	pub fn define_type(&self, name: String, ty: BasicTypeEnum<'ctx>) {
		self.types.lock().unwrap().insert(name, ty);
	}
}

#[derive(Copy, Clone)]
pub struct LibraryGenContext<'a, 'ctx> {
	context: &'ctx Context,
	module: &'a Module<'ctx>,
	builder: &'a Builder<'ctx>,
	pub fpm: &'a PassManager<FunctionValue<'ctx>>,
	pub types: &'a TypeContainer<'ctx>

}

impl<'a, 'ctx> LibraryGenContext<'a, 'ctx> {
	pub fn new(context: &'ctx Context, module: &'a Module<'ctx>, builder: &'a Builder<'ctx>, fpm: &'a PassManager<FunctionValue<'ctx>>, types: &'a TypeContainer<'ctx>) -> Self {
		Self {
			context,
			module,
			builder,
			fpm,
			types
		}
	}
	pub fn context(&self) -> &'ctx Context {
		&self.context
	}

	pub fn module(&self) -> &'a Module<'ctx> {
		&self.module
	}

	pub fn builder(&self) -> &'a Builder<'ctx> {
		&self.builder
	}

	pub fn types(&self) -> &'a TypeContainer<'ctx> {
		&self.types
	}
}

pub struct FuncGenContext<'a, 'ctx> {
	library: LibraryGenContext<'a, 'ctx>,

	local_vars: Mutex<HashMap<String, AnyValueEnum<'ctx>>>,
	function: FunctionValue<'ctx>,
}

impl<'a, 'ctx> FuncGenContext<'a, 'ctx> {
	pub fn new(library: LibraryGenContext<'a, 'ctx>, function: FunctionValue<'ctx>) -> Self {
		Self {
			library,
			function,
			local_vars: Mutex::new(HashMap::new()),
		}
	}

	pub fn library(&self) -> LibraryGenContext<'a, 'ctx> {
		self.library
	}

	pub fn function(&self) -> &FunctionValue<'ctx> {
		&self.function
	}

	pub fn define(&self, key: String, value: AnyValueEnum<'ctx>) {
		self.local_vars
			.lock()
			.unwrap()
			.insert(key, value);
	}

	pub fn get_var(&self, name: &str) -> Option<AnyValueEnum<'ctx>> {
		self.local_vars
			.lock()
			.unwrap()
			.get(name)
			.cloned()
	}
}

impl<'a, 'ctx> Deref for FuncGenContext<'a, 'ctx> {
    type Target = LibraryGenContext<'a, 'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.library
    }
}