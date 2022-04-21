use blir::{attributes::AttributeFactory, BlirContext, Library, code::{CodeBlock, StatementKind}, value::{Value, ValueKind}, scope::ScopeRef};
use errors::debugger::Debugger;
use parser::operators::OperatorFactory;

use crate::{TypeResolvePass, TypeInferPass};

pub struct ClosureResolvePass<'a, 'l> {
    factory: &'a AttributeFactory,
    context: &'a mut BlirContext,
    debugger: &'a mut Debugger<'l>,
	operator_factory: &'a OperatorFactory,
}

impl<'a, 'l> ClosureResolvePass<'a, 'l> {
    pub fn new(
		factory: &'a AttributeFactory,
		operator_factory: &'a OperatorFactory,
		context: &'a mut BlirContext,
		debugger: &'a mut Debugger<'l>) -> Self
	{
        Self {
            factory,
            context,
            debugger,
			operator_factory,
        }
    }

	pub fn run_pass(
		&mut self,
		library: &mut Library)
	{
		for func in &mut library.functions {
			let scope = func.borrow().scope().clone();

			self.resolve_code_block(&mut func.borrow_mut().code, &scope)
		}
	}

	fn resolve_code_block(
		&mut self,
		code_block: &mut CodeBlock,
		scope: &ScopeRef)
	{
		for statement in code_block.statements_mut() {
			match &mut statement.kind {
				StatementKind::Bind { value: Some(value), .. } |
				StatementKind::Return { value: Some(value) } |
				StatementKind::Eval { value, .. } => self.resolve_value(value, scope),

				_ => {}
			}
		}
	}

	fn resolve_value(
		&mut self,
		value: &mut Value,
		scope: &ScopeRef)
	{
		match &mut value.kind {
			ValueKind::Closure(closure) => {
				// Run the resolve pass
				let mut resolve_pass = TypeResolvePass::new(self.factory,
															self.operator_factory,
															self.context,
															self.debugger);

				resolve_pass.resolve_closure(closure, &mut value.typ, scope);

				// Infer the codeblock
				let mut infer_pass = TypeInferPass::new(self.context,
														self.debugger);

				infer_pass.infer_closure(closure, &mut value.typ, scope);
			}

			ValueKind::FuncCall { function, args } => {
				self.resolve_value(function, scope);

				for arg in &mut args.args {
					self.resolve_value(arg, scope);
				}
			}

			//TODO: Fill this out
			ValueKind::If(if_statement) => {

			}

			_ => { }
		}
	}
}