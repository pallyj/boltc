use bolt_ast::{
	Expression as AstExpression
};
use blir::{
	Expr as BlirExpr,
	ExprKind as BlirExprKind, FuncArg, Type, TypeKind, FuncSig, SelectBranch,
};
use prelude::*;
use type_infer::type_infer_ctx;

use crate::lower_code_block;

pub fn lower_expr(expr: WithSource<AstExpression>) -> Try<BlirExpr, ()> {
	let (expr, source) = expr.unwrap();

	let (kind, typ) = match expr {
		AstExpression::IntLiteral(i) => (BlirExprKind::IntLiteral(i), type_infer_ctx()),
		AstExpression::FloatLiteral(f) => (BlirExprKind::FloatLiteral(f), type_infer_ctx()),
		AstExpression::StringLiteral(s) => (BlirExprKind::StringLiteral(s), type_infer_ctx()),

		AstExpression::Unit => (BlirExprKind::Unit, Type::new_anon(TypeKind::Unit)),

		AstExpression::Tuple(items) => {
			let mut blir_items = vec![];
			let mut tuple_ty_items = vec![];

			for ast_item in items.into_iter() {
				let expr = require!(lower_expr(ast_item));

				tuple_ty_items.push(expr.typ());
				blir_items.push(expr);
			}

			(BlirExprKind::Tuple(blir_items), Type::new_anon(TypeKind::Tuple(tuple_ty_items)))
		}

		AstExpression::Named(name) => (BlirExprKind::Named(name), type_infer_ctx()),

		AstExpression::FixOperator(expr, op) => {
			let expr = require!(lower_expr(*expr));

			let op_func_kind = BlirExprKind::FixOperator(expr.typ(), op);
			let ret_typ = type_infer_ctx();
			let op_func_ty_kind = TypeKind::Func(Box::new( FuncSig::new(vec![expr.typ()], ret_typ.clone()) ));
			let op_func = BlirExpr::new_anon(op_func_kind, Type::new_anon(op_func_ty_kind));

			(BlirExprKind::FuncCall {
				func: Box::new(op_func),
				args: vec![ FuncArg::new(expr, None) ]
			}, ret_typ)
		}
		AstExpression::InfixOperator(l, op, r) => {
			let l = require!(lower_expr(*l));
			let r = require!(lower_expr(*r));

			let op_func_kind = BlirExprKind::InfixOperator(l.typ(), r.typ(), op);
			let ret_typ = type_infer_ctx();
			let op_func_ty_kind = TypeKind::Func( Box::new( FuncSig::new(vec![l.typ(), r.typ()], ret_typ.clone()) ) );
			let op_func = BlirExpr::new_anon(op_func_kind, Type::new_anon(op_func_ty_kind));

			(BlirExprKind::FuncCall {
				func: Box::new(op_func),
				args: vec![
					FuncArg::new(l, None),
					FuncArg::new(r, None),
				]
			}, ret_typ)
		}

		/*AstExpression::CollectionLiteral() =>*/
		/*AstExpression::RecordLiteral() => */
		
		AstExpression::FuncCall(func, args) => {
			let func = require!(lower_expr(*func));

			let mut blir_args = vec![];

			for arg in args.into_iter() {
				let arg = arg.unwrap().0;

				let (label, arg_val) = arg.label_and_value();

				let blir_val = require!(lower_expr(arg_val));

				blir_args.push(FuncArg::new(blir_val, label));
			}

			(BlirExprKind::FuncCall { func: Box::new(func), args: blir_args }, type_infer_ctx())

			
		}

		AstExpression::If { condition, positive, negative } => {
			let mut branches = vec![];

			let first_branch = SelectBranch::new(require!( lower_expr(*condition)), require!(lower_code_block(positive.unwrap().0)));
			branches.push(first_branch);

			let mut next_branch = &negative;

			let mut finally = None;

			while let Some(next_branch_positive) = &next_branch {
				let next_branch_value = next_branch_positive.value();
				
				if next_branch_value.statements().len() == 1 {
					let first_statement = &next_branch_value.statements()[0].statement();

					if let bolt_ast::Statement::Expr(e) = first_statement.value() {
						if let AstExpression::If { condition, positive, negative } = e {
							let later_branch = SelectBranch::new(require!( lower_expr((**condition).clone())), require!(lower_code_block(positive.clone().unwrap().0)));
							branches.push(later_branch);

							next_branch = negative;
							continue;
						}
					}
				}

				finally = Some(Box::new(require!(lower_code_block(next_branch_positive.value().clone()))));

				break;
			}

			(BlirExprKind::Select { branches, finally }, type_infer_ctx())
		}

		AstExpression::Member(parent, member) => {
			(BlirExprKind::Member(Box::new(require!(lower_expr(*parent))), member), type_infer_ctx())
		}

		_ => (BlirExprKind::Unit, Type::new_anon(TypeKind::Unit))
	};

	Try::Some(BlirExpr::new(kind, typ, source))
}