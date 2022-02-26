use prelude::*;
use bolt_ast::{
	Statement as AstStatement,
	CodeBlock as AstCodeBlock,
};
use blir::{Statement, StatementKind, CodeBlock};
use type_infer::type_infer_ctx;

use crate::{lower_expr, lower_type};


pub fn lower_smt(smt: WithSource<AstStatement>) -> Try<Statement, ()> {
	let (smt, source) = smt.unwrap();

	let kind = match smt {
		AstStatement::Expr(expr) => StatementKind::Eval(require!(lower_expr(expr.with_source(source.clone())))),

		AstStatement::Bind { name, typ, expr } => {
			let typ = match typ {
				Some(typ) => require!(lower_type(typ)),
				None => type_infer_ctx(),
			};

			let value = match expr {
				Some(expr) => Some(require!(lower_expr(expr))),
				None => None,
			};

			StatementKind::Bind {
				name,
				typ,
				value,
			}
		}

		AstStatement::Break(label) => StatementKind::Break { label },

		AstStatement::Continue(label) => StatementKind::Continue { label },

		AstStatement::Return(value) => {
			match value {
				Some(value) => StatementKind::Return { value: Some(require!(lower_expr(value))) },
				None => StatementKind::Return { value: None },
			}
		}

		AstStatement::Throw(error) => StatementKind::Throw { value: require!(lower_expr(error)) },

		AstStatement::Repeat { code } => {
			let code = require!(lower_code_block(code));

			StatementKind::Repeat { code }
		}

		AstStatement::While { condition, code } => {
			let condition = require!(lower_expr(condition));

			let code = require!(lower_code_block(code));

			StatementKind::While { condition, code }
		}
	};

	Try::Some(kind.sourced(source))
}

pub fn lower_code_block(code: AstCodeBlock) -> Try<CodeBlock, ()> {
	// The bool attached to a statement refers to whether it can return,
	// that is, whether it lacks a semicolon
	let mut statements = vec![];

	for statement in code.into_statements() {
		let can_return = statement.can_return();

		let statement = statement.into_statement();

		statements.push(
			(require!(lower_smt(statement)), can_return)
		)
	}

	Try::Some(CodeBlock::new(statements))
}