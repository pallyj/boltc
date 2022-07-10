use blir::{attributes::AttributeFactory,
           code::{CodeBlock, StatementKind},
           scope::ScopeRef,
           typ::{StructRef, EnumRef},
           value::{IfBranch, IfValue, Value, ValueKind},
           BlirContext, Library};
use errors::DiagnosticReporter;
use parser::operators::OperatorFactory;

use crate::{TypeInferPass, TypeResolvePass};

pub struct ClosureResolvePass<'a, 'l> {
    factory:          &'a AttributeFactory,
    context:          &'a mut BlirContext,
    debugger:         &'a mut DiagnosticReporter<'l>,
    operator_factory: &'a OperatorFactory,
}

impl<'a, 'l> ClosureResolvePass<'a, 'l> {
    pub fn new(factory: &'a AttributeFactory, operator_factory: &'a OperatorFactory, context: &'a mut BlirContext, debugger: &'a mut DiagnosticReporter<'l>) -> Self {
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

        for r#enum in &library.enums {
            self.resolve_enum(r#enum);
        }

        for global in &library.globals {
            self.resolve_value(&mut global.default_value_mut(), &scope);
        }
    }

    fn resolve_enum(&mut self, r#enum: &EnumRef) {
        for method in r#enum.methods().iter() {
            let scope = method.borrow().scope().clone();

            self.resolve_code_block(&mut method.borrow_mut().code, &scope)
        }

        for substruct in r#enum.substructs().iter() {
            self.resolve_struct(substruct);
        }

        for subenum in r#enum.subenums().iter() {
            self.resolve_enum(subenum);
        }
    }

    fn resolve_struct(&mut self, r#struct: &StructRef) {
        let scope = r#struct.borrow().scope().clone();

        for r#struct in &r#struct.borrow().substructs {
            self.resolve_struct(r#struct);
        }

        for r#enum in &r#struct.borrow().subenums {
            self.resolve_enum(r#enum);
        }

        for constant in &r#struct.borrow().constants {
            self.resolve_value(&mut constant.borrow_mut().value, &scope);
        }

        for global in &r#struct.borrow().globals {
            self.resolve_value(&mut global.default_value_mut(), &scope);
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
                StatementKind::Bind { value: Some(value), .. } |
                StatementKind::Return { value: Some(value) } |
                StatementKind::Eval { value, .. } |
                StatementKind::Break(Some(value), _) => {
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

            ValueKind::If(if_statement) => self.resolve_if_statement(if_statement, scope),

            ValueKind::InstanceMethod { reciever, .. } => self.resolve_value(reciever, scope),

            ValueKind::InstanceVariable { reciever, .. } => self.resolve_value(reciever, scope),

            ValueKind::Assign(left, right) => {
                self.resolve_value(left.as_mut(), scope);
                self.resolve_value(right.as_mut(), scope);
            }

            ValueKind::Match(match_value) => {
                self.resolve_value(match_value.discriminant.as_mut(), scope);

                for branch in &mut match_value.branches {
                    self.resolve_code_block(&mut branch.code, scope)
                }
            }

            ValueKind::Loop { code: code_block, .. } => self.resolve_code_block(code_block, scope),

            ValueKind::TupleField(of_tuple, _) => self.resolve_value(of_tuple.as_mut(), scope),

            ValueKind::Tuple(fields) => fields.iter_mut().for_each(|field| self.resolve_value(field, scope)),

            ValueKind::Member { parent, .. } => self.resolve_value(parent, scope),
            ValueKind::PolymorphicMethod { reciever, .. } => self.resolve_value(reciever, scope),
            ValueKind::MonomorphizeFn { function, .. } => self.resolve_value(function, scope),

            ValueKind::SequenceLiteral(seq) => {
                for seq_item in seq {
                    self.resolve_value(seq_item, scope);
                }
            }

            ValueKind::RepeatingLiteral { repeating, .. } => {
                self.resolve_value(repeating, scope);
            }

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
                IfBranch::ElseLet(match_value) => {
                    self.resolve_value(match_value.discriminant.as_mut(), scope);

                    for branch in &mut match_value.branches {
                        self.resolve_code_block(&mut branch.code, scope)
                    }
                }
            }
        }
    }
}
