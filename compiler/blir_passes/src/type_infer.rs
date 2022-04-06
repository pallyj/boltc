use blir::{Library, code::{FunctionRef, CodeBlock, Statement, StatementKind, MethodRef}, typ::{Type, TypeKind, StructRef}, value::{Value, ValueKind, IfValue, IfBranch, VarRef, ConstantRef}, scope::ScopeRef, BlirContext};
use errors::{debugger::Debugger, error::ErrorCode, Span};
use hminfer::{TypeInferCtx, TypeTable};

pub fn run_pass(library: &mut Library, context: &BlirContext, debugger: &mut Debugger) {
	for r#struct in library.structs.iter() {
		infer_struct(r#struct, context, debugger)
	}

	for func in library.functions.iter() {
		infer_func(func, context, debugger);
	}
}

fn infer_struct(r#struct: &StructRef, context: &BlirContext, debugger: &mut Debugger) {
	// TODO: Have an infer thingy

	let borrowed = r#struct.borrow();

	let vars = borrowed.instance_vars.clone();
	let constants = borrowed.constants.clone();
	let methods = borrowed.methods.clone();

	let scope = borrowed.scope().clone();

	for variable in &vars {
		infer_variable(variable, &scope, context, debugger);
	}

	for constant in &constants {
		infer_constant(constant, &scope, context, debugger);
	}

	for method in &methods {
		infer_method(method, context, debugger);
	}
}

fn infer_variable(variable: &VarRef, scope: &ScopeRef, context: &BlirContext, debugger: &mut Debugger) {
	let ty = variable.borrow().typ.clone();

	let mut variable = variable.borrow_mut();

	if let Some(ref mut value) = &mut variable.default_value {
		let mut infer_ctx = TypeInferCtx::new(debugger);

		infer_ctx.infer_rel(value, &ty, scope);

		let table = infer_ctx.finalize(context);

		replace_value(value, &table, debugger);

		replace_ty(&mut variable.typ, &table, debugger);
	}
}

fn infer_constant(variable: &ConstantRef, scope: &ScopeRef, context: &BlirContext, debugger: &mut Debugger) {
	let mut variable = variable.borrow_mut();

	let mut infer_ctx = TypeInferCtx::new(debugger);

	let ty = variable.typ.clone();

	infer_ctx.infer_rel(&mut variable.value, &ty, scope);

	let table = infer_ctx.finalize(context);

	replace_ty(&mut variable.typ, &table, debugger);
	replace_value(&mut variable.value, &table, debugger);
}

fn infer_method(method: &MethodRef, context: &BlirContext, debugger: &mut Debugger) {
	let mut infer_ctx = TypeInferCtx::new(debugger);

	let mut method = method.borrow_mut();
	let ty = method.info.return_type().clone();

	let scope = method.scope().clone();
	
	let span = method.span.clone();

	infer_ctx.infer_codeblock(&mut method.code, &ty, &scope, &Some(span));

	let type_table = infer_ctx.finalize(context);

	replace_code_block(&mut method.code, &type_table, debugger);
}

fn infer_func(func: &FunctionRef, context: &BlirContext, debugger: &mut Debugger) {
	let mut infer_ctx = TypeInferCtx::new(debugger);

	let mut func = func.borrow_mut();
	let ty = func.info.return_type().clone();

	let scope = func.scope().clone();
	
	let span = func.span.clone();

	infer_ctx.infer_codeblock(&mut func.code, &ty, &scope, &Some(span));

	let type_table = infer_ctx.finalize(context);

	replace_code_block(&mut func.code, &type_table, debugger);
}

fn replace_code_block(codeblock: &mut CodeBlock, table: &TypeTable, debugger: &mut Debugger) {
	for smt in codeblock.statements_mut() {
		replace_smt(smt, table, debugger);
	}
}

fn replace_smt(smt: &mut Statement, table: &TypeTable, debugger: &mut Debugger) {
	match &mut smt.kind {
		StatementKind::Eval { value, escaped: _ } => replace_value(value, table, debugger),
		StatementKind::Return { value } => { value.as_mut().map(|value| replace_value(value, table, debugger)); },
		StatementKind::Bind { name: _, typ, value } => {
			replace_ty(typ, table, debugger);
			value.as_mut().map(|value| replace_value(value, table, debugger));
		}
	}
}

fn replace_value(value: &mut Value, table: &TypeTable, debugger: &mut Debugger) {
	replace_ty_span(&mut value.typ, table, debugger, &value.span);

	match &mut value.kind {
		ValueKind::FuncCall { function, args } => {
			replace_value(function.as_mut(), table, debugger);

			args.args
				.iter_mut()
				.for_each(|arg| replace_value(arg, table, debugger));
		}

		ValueKind::InstanceMethod { reciever, method: _ } => {
			replace_value(reciever, table, debugger);
		}

		ValueKind::InstanceVariable { reciever, .. } => {
			replace_value(reciever, table, debugger);
		}

		ValueKind::If(if_value) => replace_if_value(if_value, table, debugger),

		_ => { }
	}
}

fn replace_if_value(if_value: &mut IfValue, table: &TypeTable, debugger: &mut Debugger) {
	replace_value(&mut if_value.condition, table, debugger);

	replace_code_block(&mut if_value.positive, table, debugger);

	if let Some(else_branch) = if_value.negative.as_mut() {
		match else_branch {
			IfBranch::CodeBlock(block) => replace_code_block(block, table, debugger),
			IfBranch::Else(if_value) => replace_if_value(if_value.as_mut(), table, debugger),
		}
	}
}

fn replace_ty_span(ty: &mut Type, table: &TypeTable, debugger: &mut Debugger, span: &Option<Span>) {
	if let TypeKind::Infer { key } = ty.kind() {
		if let Some(kind) = table.get(key) {
			ty.set_kind(kind.clone());
		} else {
			debugger.throw_single(ErrorCode::AmbiguousTy, span);
		}
	}	
}

fn replace_ty(ty: &mut Type, table: &TypeTable, _debugger: &mut Debugger) {
	if let TypeKind::Infer { key } = ty.kind() {
		if let Some(kind) = table.get(key) {
			ty.set_kind(kind.clone());
		} else {
			//debugger.throw_single(ErrorCode::AmbiguousTy, &ty.span);
		}
	}	
}