#![feature(let_else)]

mod scope;

// Type resolver

use std::{sync::{Arc}};

use blir::{Walker, ExprKind, TypeKind, Expr, Scope, Type, SymbolKind, MethodDef, StatementKind, Library, StructDef};
use prelude::ErrorCtx;
use scope::CodeBlockScope;
use type_infer::type_infer_ctx;

pub struct SymbolResolver {
    _error_ctx: ErrorCtx,
}

impl SymbolResolver {
    pub fn new() -> SymbolResolver {
        SymbolResolver {
            _error_ctx: ErrorCtx::new()
        }
    }
}

impl Walker for SymbolResolver {
    type ChildWalker = blir::ChildWalker<Self>;

    fn walk_library(&self, library: &Arc<Library>) {
        for r#struct in library.structs().iter() {
            let parent = r#struct.parent().unwrap();

            self.walk_struct(&r#struct, &parent);
        }

        for func in library.extern_funcs().iter() {
            let parent = func.parent();

			self.walk_extern_function(&func, &parent);
        }

		for func in library.funcs().iter() {
			let parent = func.parent();

			self.walk_function(&func, &parent);
		}
	}

    fn walk_function(&self, func: &Arc<blir::FuncDef>, scope: &Arc<dyn Scope>) {
        for t in func.params().iter_mut() {
            self.walk_type(t.typ_mut(), scope);
        }
        self.walk_type(&mut func.return_type(), scope);

        let func_scope: Arc<dyn Scope> = func.clone();

        self.walk_code_block(&mut func.code(), &func_scope);
    }

    fn walk_struct(&self, r#struct: &Arc<StructDef>, scope: &Arc<dyn Scope>) {
        Self::ChildWalker::walk_struct(self, r#struct, scope)
    }

    fn walk_method(&self, method: &Arc<MethodDef>, scope: &Arc<dyn Scope>) {
        for t in method.params().iter_mut() {
            self.walk_type(t.typ_mut(), scope);
        }
        self.walk_type(&mut method.return_type(), scope);

        let func_scope: Arc<dyn Scope> = method.clone();

        self.walk_code_block(&mut method.code(), &func_scope);
    }

    fn walk_code_block(&self, code_block: &mut blir::CodeBlock, scope: &Arc<dyn Scope>) {
        let scope = CodeBlockScope::new(scope);
        let as_scope: Arc<dyn Scope> = scope;

        for smt in code_block.statements_mut() {
            self.walk_statement(&mut smt.0, &as_scope);
        }
    }

    fn walk_statement(&self, smt: &mut blir::Statement, scope: &Arc<dyn Scope>) {
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

    fn walk_expr(&self, expr: &mut Expr, scope: &Arc<dyn Scope>) {
        match expr.kind_mut() {
            ExprKind::Named(ref name) => {
                let Some(lookup) = scope.lookup_symbol(name) else {
                    // Throw an error
                    // ResolverError::UnresolvedSymbol(name)
                    return;
                };

                let mut resolved = match lookup.kind() {
                    //Some(SymbolKind::Expr(e)) => e,
                    SymbolKind::Function(f) => f.clone(),
                    SymbolKind::Value(v) => v.clone(),
                    SymbolKind::Type(t) => Expr::new_anon(ExprKind::Type(t.clone()), TypeKind::Type(Box::new(t.kind().clone())).anon()),
                    _n => {
                        // Throw an error
                        // ResolverError::ExpectedExprSymbol(n)
                        return;
                    }
                };

                resolved.set_source(expr.source());

                *expr = resolved;

            }

            ExprKind::FuncCall { func, args } => {
                self.walk_expr(&mut *func, scope);

                for arg in args.iter_mut() {
                    self.walk_expr(arg.value_mut(), scope);
                }

                match func.kind() {
                    ExprKind::Type(ty) => {
                        let ty = ty.clone();
                        
                        *func.kind_mut() = ExprKind::Init(ty.clone());
                        *expr.typ_mut() = ty;
                    }
                    _ => {
                        match func.typ().kind() {
                            TypeKind::Func(sig) => {
                                *expr.typ_mut() = sig.return_type().clone();
                            }
                            _ => {}
                        }
                    }
                }
            }

            ExprKind::Member(parent, member) => {
                self.walk_expr(&mut *parent, scope);

                match parent.kind() {
                    ExprKind::Type(ty) => {
                        // Maybe move this to after type resolution
                        // Lookup static symbol
                        let Some(sym) = ty.lookup_static_member(member) else {
                            // Not necessarily an error
                            return;
                        };

                        let member = match sym.kind() {
                            SymbolKind::Type(ty) => {
                                Expr::new_anon(ExprKind::Type(ty.clone()), TypeKind::Type(Box::new(ty.kind().clone())).anon())
                            }

                            SymbolKind::StaticMethod(method_def) => {
                                let ty = TypeKind::Func(Box::new(method_def.signature())).anon();
                                
                                Expr::new_anon(ExprKind::StaticMethod(method_def.clone()), ty)
                            }

                            _ => { return }
                        };

                        *expr = member;
                    }
                    _ => {

                        // Maybe move this to after type resolution
                        // Lookup static symbol
                        let Some(sym) = parent.typ().lookup_instance_member(member) else {
                            // Not necessarily an error
                            return;
                        };

                        let member = match sym.kind().clone() {
                            SymbolKind::InstanceVariable(var) => {
                                println!("IV");
                                let parent = std::mem::replace(parent.as_mut(), Expr::new_anon(ExprKind::Unit,TypeKind::Unit.anon()));
    
                                Expr::new_anon(ExprKind::InstanceVariable { instance: Box::new(parent), variable: var.clone() }, var.typ().clone())
                            }

                            _ => { return }
                        };

                        *expr = member;
                    }
                }
            }

            _ => Self::ChildWalker::walk_expr(self, expr, scope)
        }
    }

    fn walk_type(&self, typ: &mut Type, scope: &Arc<dyn Scope>) {
        match typ.kind() {
            TypeKind::Named(ref name) => {
                let Some(resolved) = scope.lookup_symbol(name) else {
                    // Throw an error
                    // ResolverError::UnresolvedSymbol(name)
                    return;
                };

                let resolved = match resolved.kind() {
                    SymbolKind::Type(t) => t,
                    _n => {
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

    fn walk_variable(&self, variable: &Arc<blir::VariableDef>, scope: &Arc<dyn Scope>) {
        self.walk_type(&mut variable.typ(), scope) 
    }

    fn walk_extern_function(&self, func: &Arc<blir::ExternFuncDef>, scope: &Arc<dyn Scope>) {
        for t in func.params().iter_mut() {
            self.walk_type(t.typ_mut(), scope);
        }
        self.walk_type(&mut func.return_type(), scope);
    }
}

fn mangle_bind_name(name: &mut String, idx: u64) {
    let new_name = format!("var{:x}_{}", idx, name);

    *name = new_name
}

pub struct ManglePass {}

impl ManglePass {
    pub fn new() -> ManglePass {
        ManglePass {}
    }
}

impl Walker for ManglePass {
    type ChildWalker = blir::ChildWalker<Self>;

    fn walk_library(&self, lib: &Arc<Library>) {
        for r#struct in lib.structs().iter() {
            let parent = r#struct.parent().unwrap();

            self.walk_struct(&r#struct, &parent);
        }

        for func in lib.funcs().iter() {
            let parent = func.parent();

			self.walk_function(&func, &parent);
		}
    }

    fn walk_function(&self, func: &Arc<blir::FuncDef>, _scope: &Arc<dyn Scope>) {
        func.set_link_name(func.symbol().mangle());
    }

    fn walk_variable(&self, variable: &Arc<blir::VariableDef>, scope: &Arc<dyn Scope>) {
    }

    fn walk_struct(&self, r#struct: &Arc<blir::StructDef>, scope: &Arc<dyn Scope>) {
        Self::ChildWalker::walk_struct(self, r#struct, scope);
        r#struct.set_link_name(r#struct.symbol().mangle());
    }

    fn walk_method(&self, method: &Arc<MethodDef>, _scope: &Arc<dyn Scope>) {
        method.set_link_name(method.symbol().mangle());
    }

    fn walk_code_block(&self, _code_block: &mut blir::CodeBlock, _scope: &Arc<dyn Scope>) {
        todo!()
    }

    fn walk_statement(&self, _smt: &mut blir::Statement, _scope: &Arc<dyn Scope>) {
        todo!()
    }

    fn walk_expr(&self, _expr: &mut Expr, _scope: &Arc<dyn Scope>) {
        todo!()
    }

    fn walk_type(&self, _typ: &mut Type, _scope: &Arc<dyn Scope>) {
        todo!()
    }

    fn walk_extern_function(&self, func: &Arc<blir::ExternFuncDef>, scope: &Arc<dyn Scope>) {
        todo!()
    }
}

