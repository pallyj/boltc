#![feature(let_else)]

mod scope;

// Type resolver

use std::{sync::{Arc, Mutex, MutexGuard}, ops::DerefMut};

use blir::{Walker, ExprKind, TypeKind, Expr, Scope, Type, SymbolKind, MethodDef, Statement, StatementKind, Library};
use prelude::ErrorCtx;
use rand::Rng;
use scope::CodeBlockScope;

pub struct SymbolResolver {
    error_ctx: ErrorCtx,
}

impl SymbolResolver {
    pub fn new() -> SymbolResolver {
        SymbolResolver {
            error_ctx: ErrorCtx::new()
        }
    }
}

impl Walker for SymbolResolver {
    type Context = Arc<dyn Scope>;
    type ChildWalker = blir::ChildWalker<Self>;

    fn walk_function(&self, func: &Arc<blir::FuncDef>, scope: &Self::Context) {
        for t in func.params().iter_mut() {
            self.walk_type(t.typ_mut(), scope);
        }
        self.walk_type(&mut func.return_type(), scope);

        let func_scope: Arc<dyn Scope> = func.clone();

        self.walk_code_block(&mut func.code(), &func_scope);
    }

    fn walk_struct(&self, r#struct: &mut blir::StructDef, scope: &Self::Context) {
        //Self::ChildWalker::walk_struct(self, r#struct, scope)
    }

    fn walk_method(&self, method: &mut MethodDef, scope: &Self::Context) {
        todo!()
    }

    fn walk_code_block(&self, code_block: &mut blir::CodeBlock, scope: &Self::Context) {
        let scope = CodeBlockScope::new(scope);
        let as_scope: Arc<dyn Scope> = scope;

        for smt in code_block.statements_mut() {
            self.walk_statement(&mut smt.0, &as_scope);
        }
    }

    fn walk_statement(&self, smt: &mut blir::Statement, scope: &Self::Context) {
        match smt.kind_mut() {
            StatementKind::Eval(ref mut expr) => self.walk_expr(expr, scope),
            StatementKind::Return { ref mut value } => {
                if let Some(ref mut value) = value {
                    self.walk_expr(value, scope)
                } 
            }
            StatementKind::Throw { value } => self.walk_expr(value, scope),
            StatementKind::Repeat { ref mut code } => self.walk_code_block(code, scope),
            StatementKind::While { ref mut condition, ref mut code } => {
                self.walk_expr(condition, scope);
                self.walk_code_block(code, scope);
            }
            StatementKind::Bind { name, ref mut typ, ref mut value } => {
                let var_name = name.clone();
                
                mangle_bind_name(name, scope.take_index());
                scope.define_expr(var_name, Expr::new_anon(ExprKind::LocalVariable(name.clone()), typ.clone()));
                self.walk_type(typ, scope);

                if let Some(ref mut value) = value {
                    self.walk_expr(value, scope);
                }
            }
            _ => {}
        }
    }

    fn walk_expr(&self, expr: &mut Expr, scope: &Self::Context) {
        match expr.kind() {
            ExprKind::Named(ref name) => {
                let Some(lookup) = scope.lookup_symbol(name) else {
                    // Throw an error
                    // ResolverError::UnresolvedSymbol(name)
                    return;
                };

                let resolved = match lookup.kind() {
                    //Some(SymbolKind::Expr(e)) => e,
                    SymbolKind::Function(f) => f,
                    SymbolKind::Value(v) => v,
                    n => {
                        // Throw an error
                        // ResolverError::ExpectedExprSymbol(n)
                        return;
                    }
                };

                let mut resolved = resolved
                    .clone();
                resolved.set_source(expr.source());

                *expr = resolved;

            }

            _ => Self::ChildWalker::walk_expr(self, expr, scope)
        }
    }

    fn walk_type(&self, typ: &mut Type, scope: &Self::Context) {
        match typ.kind() {
            TypeKind::Named(ref name) => {
                let Some(resolved) = scope.lookup_symbol(name) else {
                    // Throw an error
                    // ResolverError::UnresolvedSymbol(name)
                    return;
                };

                let resolved = match resolved.kind() {
                    SymbolKind::Type(t) => t,
                    n => {
                        // Throw an error
                        // ResolverError::ExpectedTypeSymbol(n)
                        return;
                    }
                };

                *typ.kind_mut() = resolved.kind().clone();
            }
            _ => Self::ChildWalker::walk_type(self, typ, scope),
        }
    }

    fn walk_library(&self, library: &Arc<Library>) {
		for func in library.funcs().iter() {
			let parent = func.parent();

			self.walk_function(&func, &parent);
		}
	}
}

fn mangle_bind_name(name: &mut String, idx: u64) {
    let new_name = format!("var{:x}_{}", idx, name);

    *name = new_name
}