use std::{collections::HashMap, ops::Deref, sync::Mutex};

use inkwell::{module::Module, builder::Builder, context::Context, values::{FunctionValue, BasicValueEnum}, OptimizationLevel};

#[derive(Copy, Clone)]
pub struct LibraryGenContext<'a, 'ctx> {
	context: &'ctx Context,
	module: &'a Module<'ctx>,
	builder: &'a Builder<'ctx>,
}

impl<'a, 'ctx> LibraryGenContext<'a, 'ctx> {
	pub fn new(context: &'ctx Context, module: &'a Module<'ctx>, builder: &'a Builder<'ctx>) -> Self {
		Self {
			context,
			module,
			builder
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
}

pub struct FuncGenContext<'a, 'ctx> {
	library: LibraryGenContext<'a, 'ctx>,

	local_vars: Mutex<HashMap<String, BasicValueEnum<'ctx>>>,
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

	pub fn define(&self, key: String, value: BasicValueEnum<'ctx>) {
		self.local_vars
			.lock()
			.unwrap()
			.insert(key, value);
	}

	pub fn get_var(&self, name: &str) -> Option<BasicValueEnum<'ctx>> {
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