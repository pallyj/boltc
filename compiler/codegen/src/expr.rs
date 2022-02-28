use blir::{Expr, ExprKind};
use inkwell::values::{BasicValueEnum, BasicMetadataValueEnum, IntValue};

use crate::{context::{LibraryGenContext, FuncGenContext}, typ::generate_type};

pub fn generate_expr<'a, 'ctx>(expr: &Expr, context: &FuncGenContext<'a, 'ctx>) -> BasicValueEnum<'ctx> {
	match expr.kind() {
		ExprKind::IntLiteral(value) => {
			BasicValueEnum::IntValue(
				generate_type(expr.typ_ref(), context.library())
					.unwrap()
					.into_int_type()
					.const_int(*value, false)
			)
		}

		ExprKind::FuncCall { func, args } => {
			let args = args
				.iter()
				.map(|arg| arg.value())
				.map(|arg| generate_expr(arg, context).into())
				.collect::<Vec<_>>();
			
			generate_func_call(&func, &args, context.library())
		}

		ExprKind::FunctionParameter(n) => {
			context.function().get_nth_param(*n as u32).unwrap()
		}

		ExprKind::LocalVariable(v) => {
			context.get_var(v).unwrap()
		}

		_ => panic!(),
	}
}

fn generate_func_call<'a, 'ctx>(func: &Expr, args: &Vec<BasicValueEnum<'ctx>>, context: LibraryGenContext<'a, 'ctx>) -> BasicValueEnum<'ctx> {
	match func.kind() {
		ExprKind::IntrinsicFunc(name) => {
			let a = args[0].into_int_value();
			let b = args[1].into_int_value();

			match name.as_str() {
				"integer8Add" | "integer16Add" | "integer32Add" | "integer64Add" => BasicValueEnum::IntValue(context.builder().build_int_add(a, b, "sum")),
				"integer8Mul" | "integer16Mul" | "integer32Mul" | "integer64Mul" => BasicValueEnum::IntValue(context.builder().build_int_mul(a, b, "mul")),
				_ => panic!(),
			}
		}
		_ => {
			println!("{}", func);
			panic!()
		},
	}
}