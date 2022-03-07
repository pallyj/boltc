use std::{marker::PhantomData, sync::Arc};

use crate::{Expr, Type, TypeKind, ExprKind, StructDef, FuncDef, method::MethodDef, Statement, CodeBlock, Library, Scope, VariableDef, ExternFuncDef};

pub trait Walker: Sized {
	type ChildWalker = ChildWalker<Self>;

	/// Walks through a library
	fn walk_library(&self, lib: &Arc<Library>);

	/// Walks through a function
	fn walk_function(&self, func: &Arc<FuncDef>, scope: &Arc<dyn Scope>);

	fn walk_extern_function(&self, func: &Arc<ExternFuncDef>, scope: &Arc<dyn Scope>);

	fn walk_variable(&self, variable: &Arc<VariableDef>, scope: &Arc<dyn Scope>);

	/// Walks through a struct and its children
	fn walk_struct(&self, r#struct: &Arc<StructDef>, scope: &Arc<dyn Scope>);

	fn walk_method(&self, method: &Arc<MethodDef>, scope: &Arc<dyn Scope>);

	/// Walks through a codeblock, applying the same rule to each node
	fn walk_code_block(&self, code_block: &mut CodeBlock, scope: &Arc<dyn Scope>);

	/// Walks through a statement, applying the same rule to each node
	fn walk_statement(&self, smt: &mut Statement, scope: &Arc<dyn Scope>);

	/// Walks through an expression, applying the same rule to each node
	fn walk_expr(&self, expr: &mut Expr, scope: &Arc<dyn Scope>);

	/// Walks through a type, applying the same rule to each node
	fn walk_type(&self, typ: &mut Type, scope: &Arc<dyn Scope>);
}

pub struct ChildWalker<T: Walker> { _phantom: PhantomData<T> }

impl<T: Walker> ChildWalker<T> {
	pub fn walk_type<'a, 'b: 'a>(walker: &'a T,  typ: &mut Type, scope: &'b Arc<dyn Scope>) {
		match typ.kind_mut() {
			TypeKind::Tuple(ref mut tuple_items) => {
				tuple_items
					.iter_mut()
					.for_each(|tuple_item| walker.walk_type(tuple_item, scope))
			}
			TypeKind::Func(ref mut sig) => {
				walker.walk_type(sig.return_type_mut(), scope);

				sig.parameters_mut().iter_mut().for_each(|par| walker.walk_type(par, scope))
			}
			_ => {}
		}
	}

	pub fn walk_expr<'a>(walker: &'a T, expr: &mut Expr, scope: &'a Arc<dyn Scope>) {
		match expr.kind_mut() {
			ExprKind::FuncCall { ref mut func, ref mut args } => {
				walker.walk_expr(&mut *func, scope);

				args.iter_mut()
					.for_each(|arg| walker.walk_expr(arg.value_mut(), scope));
			}
			ExprKind::Select { branches, finally } => {
				for b in branches {
					walker.walk_expr(b.condition_mut(), scope);
					walker.walk_code_block(b.code_mut(), scope);
				}

				if let Some(finally) = finally {
					walker.walk_code_block(finally.as_mut(), scope);
				}
			}
			ExprKind::Member(par, _) => {
				walker.walk_expr(&mut *par, scope);
			}
			ExprKind::Method { method: _, reciever } => {
				walker.walk_expr(&mut *reciever, scope);
			}
			_ => {}
		}
	}

	pub fn walk_struct<'a>(walker: &'a T, r#struct: &Arc<StructDef>, _scope: &'a Arc<dyn Scope>) {
		let scope2: Arc<dyn Scope> = r#struct.clone();


		for substruct in r#struct.substructs().iter() {
			walker.walk_struct(substruct, &scope2);
		}

		for variable in r#struct.variables().iter() {
			walker.walk_variable(variable, &scope2);
		}

		for method in r#struct.methods().iter() {
			walker.walk_method(method, &scope2);
		}
	
	}
}