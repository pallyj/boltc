use blir::{Library, code::{FunctionRef, CodeBlock, Statement, StatementKind}, typ::{Type, TypeKind}, value::{Value, ValueKind, IfValue, IfBranch}};
use hminfer::{TypeInferCtx, TypeTable};

pub fn run_pass(library: &mut Library) {
	for func in library.functions.iter() {
		infer_func(func);
	}
}

fn infer_func(func: &FunctionRef) {
	let mut infer_ctx = TypeInferCtx::new();

	let mut func = func.borrow_mut();
	let ty = func.return_type.clone();

	let scope = func.scope().clone();

	infer_ctx.infer_codeblock(&mut func.code, &ty, &scope);

	let type_table = infer_ctx.finalize();

	replace_code_block(&mut func.code, &type_table);
}

fn replace_code_block(codeblock: &mut CodeBlock, table: &TypeTable) {
	for smt in codeblock.statements_mut() {
		replace_smt(smt, table);
	}
}

fn replace_smt(smt: &mut Statement, table: &TypeTable) {
	match &mut smt.kind {
		StatementKind::Eval { value, escaped: _ } => replace_value(value, table),
		StatementKind::Return { value } => { value.as_mut().map(|value| replace_value(value, table)); },
		StatementKind::Bind { name: _, typ, value } => {
			replace_ty(typ, table);
			value.as_mut().map(|value| replace_value(value, table));
		}
	}
}

fn replace_value(value: &mut Value, table: &TypeTable) {
	replace_ty(&mut value.typ, table);

	match &mut value.kind {
		ValueKind::FuncCall { function, args } => {
			replace_value(function.as_mut(), table);

			args.args
				.iter_mut()
				.for_each(|arg| replace_value(arg, table));
		}

		ValueKind::If(if_value) => replace_if_value(if_value, table),

		_ => {}
	}
}

fn replace_if_value(if_value: &mut IfValue, table: &TypeTable) {
	replace_value(&mut if_value.condition, table);

	replace_code_block(&mut if_value.positive, table);

	if let Some(else_branch) = if_value.negative.as_mut() {
		match else_branch {
			IfBranch::CodeBlock(block) => replace_code_block(block, table),
			IfBranch::Else(if_value) => replace_if_value(if_value.as_mut(), table),
		}
	}
}

fn replace_ty(ty: &mut Type, table: &TypeTable) {
	if let TypeKind::Infer { key } = ty.kind() {
		if let Some(kind) = table.get(key) {
			ty.set_kind(kind.clone());
		}
	}	
}