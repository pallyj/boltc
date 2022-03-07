use std::{sync::{Mutex, Arc, MutexGuard}};

use blir::{Walker, TypeKind, ExprKind, Scope, StatementKind, Library, Type, StructDef, MethodDef, FuncSig, FuncArg};

use crate::{TypeInferenceCtx, constraint::Constraint};

pub struct InferWalker {
	infer_ctx: Arc<Mutex<TypeInferenceCtx>>
}

impl Walker for InferWalker {
    type ChildWalker = blir::ChildWalker<Self>;

    fn walk_library(&self, library: &Arc<Library>) {
		for r#struct in library.structs().iter() {
            let parent = r#struct.parent().unwrap();

            self.walk_struct(&r#struct, &parent);
        }

		for func in library.funcs().iter() {
			let parent = func.parent();

			self.walk_function(&func, &parent);
		}
	}

    fn walk_function(&self, func: &Arc<blir::FuncDef>, _scope: &Arc<dyn Scope>) {
		let func_scope: Arc<dyn Scope> = func.clone();

        self.walk_code_block(&mut func.code(), &func_scope);
    }

    fn walk_extern_function(&self, func: &Arc<blir::ExternFuncDef>, scope: &Arc<dyn Scope>) {
        todo!()
    }

    fn walk_variable(&self, variable: &Arc<blir::VariableDef>, scope: &Arc<dyn Scope>) {
		self.walk_type(&mut variable.typ(), scope) 
    }

    fn walk_struct(&self, r#struct: &Arc<StructDef>, scope: &Arc<dyn Scope>) {
        Self::ChildWalker::walk_struct(self, r#struct, scope)
    }

    fn walk_method(&self, method: &Arc<MethodDef>, _scope: &Arc<dyn Scope>) {
        let func_scope: Arc<dyn Scope> = method.clone();

        self.walk_code_block(&mut method.code(), &func_scope);
    }

    fn walk_code_block(&self, code_block: &mut blir::CodeBlock, scope: &Arc<dyn Scope>) {
        for smt in code_block.statements_mut().iter_mut().rev() {
			self.walk_statement(&mut smt.0, scope)
		}
    }

    fn walk_statement(&self, smt: &mut blir::Statement, scope: &Arc<dyn Scope>) {
        match smt.kind_mut() {
			StatementKind::Eval(e) => self.walk_expr(e, scope),

			StatementKind::Bind {
				name: _,
				typ,
				value
			} => {
				self.walk_type(typ, scope);

				if let Some(value) = value {
					self.walk_expr(value, scope);

					if let TypeKind::Infer(ctx) = typ.kind() {
						let assigned_typ = value.typ();

						let constraint = match assigned_typ.kind() {
							TypeKind::Infer(i) => Constraint::Equality(*i),
							_ => Constraint::Absolute(assigned_typ),
						};

						self.infer_ctx
							.lock()
							.unwrap()
							.add_constraint(*ctx, constraint)
					}
				}
			}

			StatementKind::Throw { value } => self.walk_expr(value, scope),
			StatementKind::Return { value } => {
				if let Some(value) = value {
					self.walk_expr(value, scope);

					if let TypeKind::Infer(ctx) = value.typ_ref().kind() {
						if let Some(return_type) = scope.scoped_type("return-type") {
							self.infer_ctx
								.lock()
								.unwrap()
								.add_constraint(*ctx, Constraint::Suggestion(Type::new_anon(return_type)))
						}
					}
				}
			}

			StatementKind::Repeat { code } => {
				self.walk_code_block(code, scope)
			}
			StatementKind::While { condition, code } => {
				self.walk_expr(condition, scope);
				self.walk_code_block(code, scope)
			}
			_ => {}
		}
    }

    fn walk_expr(&self, expr: &mut blir::Expr, scope: &Arc<dyn Scope>) {
		Self::ChildWalker::walk_expr(self, expr, scope);

		let mut constraint = None;

		match expr.kind_mut() {
			ExprKind::IntLiteral(_) => {
				constraint = Some(Constraint::SomeInteger)
			}
			ExprKind::FloatLiteral(_) => {
				constraint = Some(Constraint::SomeFloat)
			}
			ExprKind::StringLiteral(_) => {
				constraint = Some(Constraint::SomeString)
			}

			ExprKind::FuncCall { func, ref mut args } => {
				self.walk_expr(func.as_mut(), scope);

				let sig = match func.typ_ref().kind() {
					TypeKind::Func(sig) => sig.as_ref(),
					_ => return
				};

				let first_par = match func.kind() {
					ExprKind::Method { method: _, reciever } => Some(&**reciever),
					_ => None,
				};

				let mapped_pars = args.iter().map(|arg| arg.value());

				let args_iter = first_par.into_iter().chain(mapped_pars);

				let par_args = sig.parameters().iter().zip(args_iter);

				for p in par_args {
					match (p.0.kind(), p.1.typ().kind()) {
						(TypeKind::Infer(par), TypeKind::Infer(arg)) => {
							self.infer_ctx
								.lock()
								.unwrap()
								.add_constraint(*par, Constraint::Equality(*arg));
						}
						(_, TypeKind::Infer(arg)) => {
							self.infer_ctx
								.lock()
								.unwrap()
								.add_constraint(*arg, Constraint::Suggestion(p.0.clone()));
						}
						_ => {}
					}
				}

				constraint = Some(Constraint::Absolute(sig.return_type().clone()))
			}

			ExprKind::ExternFunction(def) => {
				let sig = def.signature();

				*expr.typ_mut() = Type::new_anon(TypeKind::Func(Box::new(sig)));
			}

			ExprKind::Function(def) => {
				let sig = def.signature();

				*expr.typ_mut() = Type::new_anon(TypeKind::Func(Box::new(sig)));
			}

			ExprKind::StaticMethod(def) => {
				let sig = def.signature();

				*expr.typ_mut() = Type::new_anon(TypeKind::Func(Box::new(sig)));
			}

			ExprKind::Select { branches: _branches, finally: _finally } => {
				*expr.typ_mut() = Type::new_anon(TypeKind::Unit);
			}

			_ => { Self::ChildWalker::walk_expr(self, expr, scope) }
		}

		let TypeKind::Infer(infer_ctx) = expr.typ_ref().kind() else {
			return
		};

		let infer_ctx = *infer_ctx;
		
		if let Some(constraint) = constraint {
			self.infer_ctx
				.lock()
				.unwrap()
				.add_constraint(infer_ctx, constraint);
		}
    }

    fn walk_type(&self, typ: &mut blir::Type, scope: &Arc<dyn Scope>) {
        Self::ChildWalker::walk_type(self, typ, scope)
    }
}

impl InferWalker {
	pub fn new() -> InferWalker {
		InferWalker {
			infer_ctx: Arc::new(Mutex::new(TypeInferenceCtx::new()))
		}
	}

	pub fn context<'a>(&'a self) -> MutexGuard<'a, TypeInferenceCtx> {
		self.infer_ctx
			.lock()
			.unwrap()
	}
}

pub struct ReplacementWalker {
	infer_ctx: Arc<Mutex<TypeInferenceCtx>>
}

impl Walker for ReplacementWalker {
    fn walk_library(&self, library: &Arc<Library>) {
		for r#struct in library.structs().iter() {
            let parent = r#struct.parent().unwrap();

            self.walk_struct(&r#struct, &parent);
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

    fn walk_variable(&self, variable: &Arc<blir::VariableDef>, scope: &Arc<dyn Scope>) {
        self.walk_type(&mut variable.typ(), scope) 
    }

    fn walk_struct<'a>(&'a self, r#struct: &Arc<StructDef>, scope: &'a Arc<dyn Scope>) {
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

    type ChildWalker = blir::ChildWalker<Self>;

    fn walk_code_block(&self, code_block: &mut blir::CodeBlock, scope: &Arc<dyn Scope>) {
		for smt in code_block.statements_mut() {
            self.walk_statement(&mut smt.0, scope);
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
            StatementKind::Bind { name: _, ref mut typ, ref mut value } => {
                self.walk_type(typ, scope);

                if let Some(ref mut value) = value {
                    self.walk_expr(value, scope);
                }
            }
            _ => {}
        }
    }

    fn walk_expr<'a>(&'a self, expr: &mut blir::Expr, scope: &'a Arc<dyn Scope>) {
        Self::ChildWalker::walk_expr(self, expr, scope);

		self.walk_type(expr.typ_mut(), scope);
    }

    fn walk_type<'a>(&'a self, typ: &mut blir::Type, scope: &'a Arc<dyn Scope>) {
		Self::ChildWalker::walk_type(self, typ, scope);

		match typ.kind_mut() {
			TypeKind::Infer(ctx) => {
				if let Some(replacement) = self.infer_ctx.lock().unwrap().get_type(*ctx) {
					let _ =std::mem::replace(typ.kind_mut(), replacement.kind().clone());
				}
			}

			_ => {}
		}
    }

    fn walk_extern_function(&self, func: &Arc<blir::ExternFuncDef>, scope: &Arc<dyn Scope>) {
        todo!()
    }
}

impl ReplacementWalker {
	pub fn new(infer_walker: InferWalker) -> ReplacementWalker {
		ReplacementWalker {
			infer_ctx: infer_walker.infer_ctx
		}
	}
}