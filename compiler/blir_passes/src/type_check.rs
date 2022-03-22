use blir::{Library, typ::Type, code::{FunctionRef, CodeBlock, Statement, StatementKind}, value::{Value, ValueKind}};

pub fn run_pass(library: &mut Library) {
	for func in library.functions.iter() {
		check_function(func);
	}
}

fn check_function(func: &FunctionRef) {
	let func = func.borrow();
	check_codeblock(&func.code, &func.return_type);
}

fn check_codeblock(code_block: &CodeBlock, return_type: &Type) {
	for smt in code_block.statements() {
		check_smt(smt, return_type);
	}
}

fn check_smt(smt: &Statement, return_type: &Type) {
	match &smt.kind {
		StatementKind::Eval { value, escaped: _ } => {
			check_value(value, return_type);
		}
		StatementKind::Bind { name: _, typ, value } => {
			if let Some(value) = value.as_ref() {
				check_value(&value, return_type);

				if !is_assignable_from(typ, &value.typ) {
					// Error
					println!("Error: Not assignable from");
				}
			}
		}
		StatementKind::Return { value } => {
			if let Some(value) = value.as_ref() {
				check_value(&value, return_type);

				if !is_assignable_from(return_type, &value.typ) {
					// Error
					println!("Error");
				}
			}
		}
	}
}

fn check_value(value: &Value, _return_type: &Type) {
	/*if &value.typ != return_type {
		println!("Error Values");
	}*/

	match &value.kind {
		ValueKind::FuncCall { function: _, args: _ } => {
			// Match function args
		}

		ValueKind::If(_if_value) => {
			// Match if
		}

		_ => {}
	}
}

/// Whether ty1 can be assigned from ty2
pub fn is_assignable_from(ty1: &Type, ty2: &Type) -> bool {
	if ty1.kind() == ty2.kind() {
		return true
	}

	match (ty1.kind(), ty2.kind()) {
		_ => false
	}
}