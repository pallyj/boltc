use blir::{Library,
	code::{FunctionRef, MethodRef, CodeBlock, Statement, StatementKind},
	typ::{StructRef, Type, TypeKind},
	scope::ScopeRef,
	value::VarRef,
	Symbol};

pub fn run_pass(library: &mut Library) {
	let scope = library
		.scope();

	for r#struct in &library.structs {
		walk_struct(r#struct, &scope);
	}

	for func in &library.functions {
		walk_function(&func, &scope);
	}
}

fn walk_struct(r#struct: &StructRef, _scope: &ScopeRef) {
	let r#struct = r#struct.borrow();
	let scope = r#struct.scope();

	for substruct in &r#struct.substructs {
		walk_struct(&substruct, &scope);
	}

	for variable in &r#struct.instance_vars {
		walk_variable(&variable, &scope);		
	}

	for method in &r#struct.methods {
		walk_method(&method, &scope);
	}
}

fn walk_variable( var: &VarRef, scope: &ScopeRef ) {
	walk_type(&mut (var.borrow_mut().typ), scope)
}

fn walk_method( method: &MethodRef, scope: &ScopeRef ) {
	let mut method = method.borrow_mut();

	walk_type(&mut method.return_type, scope);

	method.params
		.iter_mut()
		.for_each(|param| walk_type(&mut param.typ, scope));

	walk_code_block(&mut method.code, scope);
}

fn walk_function( function: &FunctionRef, scope: &ScopeRef ) {
	let mut function = function.borrow_mut();

	walk_type(&mut function.return_type, scope);

	function.params
		.iter_mut()
		.for_each(|param| walk_type(&mut param.typ, scope));

	function.add_params();

	walk_code_block(&mut function.code, scope);
}

fn walk_code_block( code: &mut CodeBlock, scope: &ScopeRef ) {
	for smt in code.statements_mut() {
		walk_statement(smt, scope);
	}
}

fn walk_statement(smt: &mut Statement, scope: &ScopeRef) {
	match &mut smt.kind {
		StatementKind::Bind { name, typ, value: _ } => {
			walk_type(typ, scope);

			*name = scope.define_variable(&name, typ.clone());
		}
		_ => {}
	}
}

fn walk_type( typ: &mut Type, scope: &ScopeRef ) {
	match typ.kind_mut() {
		TypeKind::Named(symbol_name) => {
			let Some(resolved_sym) = scope.lookup_symbol(symbol_name) else {
				// Throw an error, type was not found
				println!("Error: type {symbol_name} was not found");
				return
			};

			let Symbol::Type(resolved_typ) = resolved_sym.resolve() else {
				// Throw an error, symbol is not a type
				println!("Error: type {symbol_name} is not a type");
				return
			};

			typ.set_kind(resolved_typ);
		}

		TypeKind::Function { return_type, params, labels: _ } => {
			walk_type(return_type.as_mut(), scope);

			params
				.iter_mut()
				.for_each(|param| walk_type(param, scope));
		}

		_ => {}
	}
}