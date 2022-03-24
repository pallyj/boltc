use blir::{Library,
	code::{FunctionRef, MethodRef, CodeBlock, Statement, StatementKind, ExternFunctionRef},
	typ::{StructRef, Type, TypeKind},
	scope::ScopeRef,
	value::{VarRef, Value, ValueKind},
	Symbol};

pub fn run_pass(library: &mut Library) {
	let scope = library
		.scope();

	for r#struct in &library.structs {
		walk_struct(r#struct, &scope);
	}

	for func in &library.extern_functions {
		walk_extern_function(&func, &scope);
	}

	for func in &library.functions {
		walk_function(&func, &scope);
	}

	for func in &library.functions {
		walk_function_code(&func);
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

	for method in &r#struct.methods {
		walk_method_code(&method);
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

	method.add_params();
}

fn walk_method_code( method: &MethodRef) {
	let mut method = method.borrow_mut();

	let scope = method.scope().clone();

	walk_code_block(&mut method.code, &scope);
}

fn walk_function( function: &FunctionRef, scope: &ScopeRef ) {
	let mut function = function.borrow_mut();

	walk_type(&mut function.return_type, scope);

	function.params
		.iter_mut()
		.for_each(|param| walk_type(&mut param.typ, scope));

	function.add_params();
}

fn walk_extern_function( function: &ExternFunctionRef, scope: &ScopeRef ) {
	let mut function = function.borrow_mut();

	walk_type(&mut function.return_type, scope);

	function.params
		.iter_mut()
		.for_each(|param| walk_type(&mut param.typ, scope));
}

fn walk_function_code( function: &FunctionRef ) {
	let mut function = function.borrow_mut();

	let scope = function.scope().clone();

	walk_code_block(&mut function.code, &scope);
}

fn walk_code_block( code: &mut CodeBlock, scope: &ScopeRef ) {
	for smt in code.statements_mut() {
		walk_statement(smt, scope);
	}
}

fn walk_statement(smt: &mut Statement, scope: &ScopeRef) {
	match &mut smt.kind {
		StatementKind::Bind { name, typ, value } => {
			walk_type(typ, scope);

			*name = scope.define_variable(&name, typ.clone());

			value.as_mut().map(|value| walk_value(value, scope));
		}

		StatementKind::Eval { value, escaped: _ } => {
			walk_value(value, scope);
		}

		StatementKind::Return { value } => {
			value.as_mut().map(|value| walk_value(value, scope));
		}
	}
}

fn walk_value(value: &mut Value, scope: &ScopeRef) {
	match &mut value.kind {
		ValueKind::Named(name) => {
			let Some(sym) = scope.lookup_symbol(name).map(|sym| sym.resolve()) else {
				println!("Error: can't find symbol {name}");
				return
			};

			match sym {
				Symbol::Type(ty) => {
					value.set_kind(ValueKind::Metatype(ty.clone()));
					value.typ.set_kind(TypeKind::Metatype(Box::new(ty)));
				}

				Symbol::Value(res_val) => {
					value.set_kind(res_val.kind);
					value.typ = res_val.typ;
				}

				Symbol::Function(function) => {
					value.set_type(function.take_typ());
					value.set_kind(ValueKind::StaticFunc(function));
				}

				Symbol::ExternFunction(function) => {
					value.set_type(function.take_typ());
					value.set_kind(ValueKind::ExternFunc(function));
				}

				_ => {
					println!("Error: Invalid symbol");
				}
			}
		}

		ValueKind::FuncCall { function, args } => {
			walk_value(function.as_mut(), scope);

			if let ValueKind::Metatype(t) = &mut function.kind {
				let t = std::mem::replace(t, TypeKind::Void);

				function.set_kind(ValueKind::Init(t.anon()));
			}

			args.args
				.iter_mut()
				.for_each(|arg| walk_value(arg, scope));
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

		TypeKind::Member { parent, member } => {
			walk_type(parent.as_mut(), scope);

			let Some(sym) = parent.lookup_static_item(member.as_str()) else {
				println!("Error: member not found");
				return;
			};

			let Symbol::Type(tk) = sym else {
				println!("Error: member not a type");
				return;
			};

			typ.set_kind(tk);
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