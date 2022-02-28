use blir::{Statement, StatementKind};

use crate::{context::{FuncGenContext}, expr::generate_expr};

pub fn generate_smt(smt: &Statement, context: &FuncGenContext) {
	match smt.kind() {
		StatementKind::Return { value } => {
			let Some(value) = value else {
				context.builder().build_return(None);
				return;
			};

			let value = generate_expr(&value, context);

			context.builder().build_return(Some(&value));
		}

		StatementKind::Bind { name, typ: _, value } => {
			let value = generate_expr(value.as_ref().unwrap(), context);

			context.define(name.clone(), value);
		}

		StatementKind::Eval(value) => {
			generate_expr(&value, context);
		}
		_ => panic!(),
	}
}