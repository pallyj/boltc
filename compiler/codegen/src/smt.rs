use blir::{Statement, StatementKind};
use inkwell::values::{BasicValueEnum, BasicValue};

use crate::{context::{FuncGenContext}, expr::generate_expr};

pub fn generate_smt(smt: &Statement, context: &FuncGenContext) {
	match smt.kind() {
		StatementKind::Return { value } => {
			let Some(value) = value else {
				context.builder().build_return(None);
				return;
			};

			let Some(value) = generate_expr(&value, context) else  {
				context.builder().build_return(None);
				return;
			};

			let value: &dyn BasicValue = &BasicValueEnum::try_from(value).unwrap();

			context.builder().build_return(Some(value));
		}

		StatementKind::Bind { name, typ: _, value } => {
			let value = generate_expr(value.as_ref().unwrap(), context).unwrap();

			context.define(name.clone(), value);
		}

		StatementKind::Eval(value) => {
			generate_expr(&value, context);
		}
		_ => panic!(),
	}
}