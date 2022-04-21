use blir::{attributes::AttributeFactory,
           code::{CodeBlock, StatementKind},
           scope::ScopeRef,
           typ::StructRef,
           value::{IfBranch, IfValue, Value, ValueKind},
           BlirContext, Library};
use errors::debugger::Debugger;
use parser::operators::OperatorFactory;

use crate::{TypeInferPass, TypeResolvePass};

pub struct ClosureResolvePass<'a, 'l> {
    factory:          &'a AttributeFactory,
    context:          &'a mut BlirContext,
    debugger:         &'a mut Debugger<'l>,
    operator_factory: &'a OperatorFactory,
}

impl<'a, 'l> ClosureResolvePass<'a, 'l> {
    pub fn new(factory: &'a AttributeFactory, operator_factory: &'a OperatorFactory, context: &'a mut BlirContext, debugger: &'a mut Debugger<'l>) -> Self {
        Self { factory,
               context,
               debugger,
               operator_factory }
    }

    pub fn run_pass(&mut self, library: &mut Library) {
        let scope = library.scope().clone();

        for constant in &library.constants {
            self.resolve_value(&mut constant.borrow_mut().value, &scope);
        }

        for func in &library.functions {
            let scope = func.borrow().scope().clone();

            self.resolve_code_block(&mut func.borrow_mut().code, &scope)
        }

        for r#struct in &library.structs {
            self.resolve_struct(r#struct);
        }
    }

    fn resolve_struct(&mut self, r#struct: &StructRef) {
        let scope = r#struct.borrow().scope().clone();

        for r#struct in &r#struct.borrow().substructs {
            self.resolve_struct(r#struct);
        }

        for constant in &r#struct.borrow().constants {
            self.resolve_value(&mut constant.borrow_mut().value, &scope);
        }

        for var in &r#struct.borrow().instance_vars {
            if let Some(value) = &mut var.borrow_mut().default_value {
                self.resolve_value(value, &scope);
            }
        }

        for func in &r#struct.borrow().methods {
            let scope = func.borrow().scope().clone();

            self.resolve_code_block(&mut func.borrow_mut().code, &scope)
        }
    }

    fn resolve_code_block(&mut self, code_block: &mut CodeBlock, scope: &ScopeRef) {
        for statement in code_block.statements_mut() {
            match &mut statement.kind {
                StatementKind::Bind { value: Some(value), .. } | StatementKind::Return { value: Some(value) } | StatementKind::Eval { value, .. } => {
                    self.resolve_value(value, scope)
                }

                _ => {}
            }
        }
    }

    fn resolve_value(&mut self, value: &mut Value, scope: &ScopeRef) {
        match &mut value.kind {
            ValueKind::Closure(closure) => {
                // Run the resolve pass
                let mut resolve_pass = TypeResolvePass::new(self.factory,
                                                            self.operator_factory,
                                                            self.context,
                                                            self.debugger);

                resolve_pass.resolve_closure(closure, &mut value.typ, scope);

                // Infer the codeblock
                let mut infer_pass = TypeInferPass::new(self.context, self.debugger);

                infer_pass.infer_closure(closure, &mut value.typ, scope);
            }

            ValueKind::FuncCall { function, args } => {
                self.resolve_value(function, scope);

                for arg in &mut args.args {
                    self.resolve_value(arg, scope);
                }
            }

            // TODO: Fill this out
            ValueKind::If(if_statement) => self.resolve_if_statement(if_statement, scope),

            ValueKind::InstanceMethod { reciever, .. } => self.resolve_value(reciever, scope),

            ValueKind::InstanceVariable { reciever, .. } => self.resolve_value(reciever, scope),

            _ => {}
        }
    }

    fn resolve_if_statement(&mut self, if_statement: &mut IfValue, scope: &ScopeRef) {
        self.resolve_value(&mut if_statement.condition, scope);

        self.resolve_code_block(&mut if_statement.positive, scope);

        if let Some(negative) = &mut if_statement.negative {
            match negative {
                IfBranch::CodeBlock(negative_block) => self.resolve_code_block(negative_block, scope),
                IfBranch::Else(else_if_branch) => self.resolve_if_statement(else_if_branch, scope),
            }
        }
    }
}
