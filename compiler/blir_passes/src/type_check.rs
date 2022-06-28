use std::cmp::Ordering;

use blir::{code::{CodeBlock, FunctionRef, MethodRef, Statement, StatementKind},
           typ::{StructRef, Type, TypeKind, EnumRef},
           value::{ConstantRef, IfBranch, IfValue, Value, ValueKind, VarRef},
           Library, Monomorphizer, pattern::{Pattern, PatternKind}, Symbol};
use errors::{error::ErrorCode, Span, IntoDiagnostic, DiagnosticReporter, Diagnostic, DiagnosticLevel, CodeLocation};

pub struct TypeCheckPass<'a, 'b> {
    debugger: &'a mut DiagnosticReporter<'b>,
}

impl<'a, 'b> TypeCheckPass<'a, 'b> {
    pub fn new(debugger: &'a mut DiagnosticReporter<'b>) -> Self { Self { debugger } }

    pub fn run_pass(&mut self, library: &mut Library) {
        for func in &library.functions {
            self.check_function(func);
        }

        for r#struct in &library.structs {
            self.check_struct(r#struct);
        }

        for r#enum in &library.enums {
            self.check_enum(r#enum);
        }

        for constant in &library.constants {
            self.check_const(constant);
        }
    }

    fn check_struct(&mut self, r#struct: &StructRef) {
        let struct_ref = r#struct.borrow();

        for substruct in &struct_ref.substructs {
            self.check_struct(substruct);
        }

        for subenum in &struct_ref.subenums {
            self.check_enum(subenum);
        }

        for method in &struct_ref.methods {
            self.check_method(method);
        }

        for instance_vars in &struct_ref.instance_vars {
            self.check_var(instance_vars);
        }

        for constant in &struct_ref.constants {
            self.check_const(constant);
        }
    }

    fn check_enum(&mut self, r#enum: &EnumRef) {
        for substruct in r#enum.substructs().iter() {
            self.check_struct(substruct);
        }

        for subenum in r#enum.subenums().iter() {
            self.check_enum(subenum);
        }

        for method in r#enum.methods().iter() {
            self.check_method(method);
        }
    }

    fn check_function(&mut self, func: &FunctionRef) {
        let borrowed = func.borrow();
        let func_return_type = borrowed.info.return_type();
        let func_code = &borrowed.code;

        self.check_codeblock(func_code, Some(func_return_type), func_return_type);
    }

    fn check_method(&mut self, func: &MethodRef) {
        let borrowed = func.borrow();
        let func_return_type = borrowed.info.return_type();
        let func_code = &borrowed.code;

        self.check_codeblock(func_code, Some(func_return_type), func_return_type);
    }

    fn check_var(&mut self, var: &VarRef) {
        let variable = var.borrow();

        let Some(value) = &variable.default_value else { return };

        self.check_type(&variable.typ, &value.typ);
    }

    fn check_const(&mut self, var: &ConstantRef) {
        let variable = var.borrow();

        self.check_type(&variable.typ, &variable.value.typ);
    }

    fn check_codeblock(&mut self, code_block: &CodeBlock, code_block_type: Option<&Type>, return_type: &Type) {
        let mut did_diverge = false;
        let mut diverging_smt = None;

        for smt in code_block.statements() {
            if did_diverge {
                // error: raise a warning
                self.debugger.throw_diagnostic(TypeCheckError::UnreachableCode(smt.span.unwrap_or_default()));
            }
            if let Some(smt) = diverging_smt.take() {
                self.debugger.throw_diagnostic(TypeCheckError::CodeAfterUnreachable(smt));
            }

            self.check_smt(smt, return_type);
            let did_it_diverge = smt.diverges();
            if did_it_diverge && !did_diverge {
                diverging_smt = smt.span;
            }

            did_diverge = did_it_diverge;
        }

        if let Some(code_block_type) = code_block_type {
            self.check_type(code_block_type, &code_block.typ());
        }
    }

    fn check_smt(&mut self, statement: &Statement, return_type: &Type) {
        match &statement.kind {
            StatementKind::Eval { value, .. } => self.check_value(value, return_type),
            StatementKind::Bind { typ, value, pattern } => {
                self.check_pattern(typ, pattern);
                let Some(value) = value else { return };
                
                self.check_value(value, return_type);

                self.check_type(typ, &value.typ);
            }
            StatementKind::Return { value } => {
                if let Some(value) = value {
                    self.check_value(value, return_type);
                    self.check_type(return_type, &value.typ)
                } else {
                    self.check_type(return_type, &TypeKind::Void.anon())
                };
            }

            StatementKind::Guard { condition, otherwise } => {
                self.check_value(&condition, return_type); // todo: check that its a bool
                self.check_codeblock(otherwise, None, return_type); // todo: check that it diverges
            }

            StatementKind::GuardLet { pattern, value, otherwise } => {
                self.check_pattern(&value.typ, pattern);
                self.check_value(value, return_type);
                self.check_codeblock(otherwise, None, return_type);
            }

            StatementKind::Break(_) |
            StatementKind::Continue(_) => {}
        }
    }

    fn check_value(&mut self, value: &Value, return_type: &Type) {
        // TODO: Move this to another function
        match value.typ.kind() {
            TypeKind::Error => panic!("Compiler Error: error type"),

            TypeKind::SomeBool | TypeKind::SomeInteger | TypeKind::SomeFloat | TypeKind::SomeFunction | TypeKind::Infer { .. } => {
                self.debugger.throw_diagnostic(TypeCheckError::CouldNotInfer(value.span.clone().unwrap_or_else(Span::empty)));
            }

            TypeKind::Named(name) =>  self.debugger.throw_diagnostic(TypeCheckError::CouldNotInfer(value.span.clone().unwrap_or_else(Span::empty))),
            TypeKind::Member { member, .. } =>  self.debugger.throw_diagnostic(TypeCheckError::CouldNotInfer(value.span.clone().unwrap_or_else(Span::empty))),

            _ => {}
        }

        match &value.kind {
            ValueKind::Assign(place, value) => {
                self.check_value(place, return_type);
                self.check_value(value, return_type);

                self.check_type(&place.typ, &value.typ);
            }

            ValueKind::Closure(closure) => {
                let TypeKind::Function { return_type, .. } = value.typ.kind() else {
                    self.debugger.throw_diagnostic(TypeCheckError::IsNotAFunc(value.span.unwrap_or_default()));
                    return
                };

                self.check_codeblock(&closure.code, Some(return_type), return_type);
            }
            ValueKind::FuncCall { function, args } => {
                self.check_value(function, return_type);

                let params = match &function.typ.kind {
                    TypeKind::Function { params, labels, .. } => {
                        for (i, (label1, label2)) in labels.iter().zip(&args.labels).enumerate() {
                            if label1 != label2 {
                                let label1 = label1.clone().unwrap_or_else(|| "_".to_string());
                                let label2 = label2.clone().unwrap_or_else(|| "_".to_string());

                                self.debugger.throw_diagnostic(TypeCheckError::ExpectedLabel { expected: label1, found: label2, span: args.args[i].span.unwrap_or_default()});
                            }
                        }
                        params
                    },
                    TypeKind::Method { params, .. } => {
                        params
                    },
                    _ => {
                        self.debugger.throw_diagnostic(TypeCheckError::IsNotAFunc(value.span.unwrap_or_default()));
                        return;
                    }
                };

                match params.len().cmp(&args.args.len()) {
                    Ordering::Greater => {
                        // Less params
                        self.debugger.throw_diagnostic(TypeCheckError::MissingParams(value.span.unwrap_or_default()));
                    }
                    Ordering::Less => {
                        // Extra params
                        let spans = args.args
                                        .iter()
                                        .skip(params.len())
                                        .filter_map(|arg| arg.span)
                                        .collect();
                        self.debugger.throw_diagnostic(TypeCheckError::ExtraParams(spans));
                    }
                    Ordering::Equal => {}
                }

                for (param, arg) in params.iter().zip(&args.args) {
                    self.check_value(arg, return_type);

                    self.check_type(param, &arg.typ);
                }
            }
            ValueKind::If(if_value) => self.check_if_value(if_value, &value.typ, return_type, vec![]),
            ValueKind::InstanceMethod { reciever, .. } => self.check_value(reciever, return_type),
            ValueKind::InstanceVariable { reciever, .. } => self.check_value(reciever, return_type),

            ValueKind::Error => panic!("Compiler Error: error value"),
            ValueKind::Member { member, parent } => self.debugger.throw_diagnostic(TypeCheckError::MemberNotAValue(parent.typ.clone(), member.clone(), value.span.unwrap_or_default())),
            ValueKind::Named(name) => self.debugger.throw_diagnostic(TypeCheckError::SymbolNotAValue(name.clone(), value.span.unwrap_or_default())),
            ValueKind::Operator(name) => {
                self.debugger.throw_diagnostic(TypeCheckError::OperatorDNE( name.clone(), value.span.unwrap_or_default() ));
            }
            ValueKind::Polymorphic(polymorphic) | 
            ValueKind::PolymorphicMethod { polymorphic, .. } => {
                self.check_polymorphic(polymorphic, &value.span)
            }

            ValueKind::Match(match_value) => {
                self.check_value(&*match_value.discriminant, return_type);

                let scrut_ty = &match_value.discriminant.typ;

                // Check that scrut_ty can be matched
                for arm in &match_value.branches {
                    self.check_codeblock(&arm.code, Some(&value.typ), return_type);

                    self.check_pattern(scrut_ty, &arm.pattern);
                }
            }

            ValueKind::Loop { code: code_block, .. } => {
                self.check_codeblock(code_block, None, return_type);
            }

            _ => { /* Do nothing */ }
        }
    }

    fn check_pattern(&mut self, ty: &Type, pattern: &Pattern) {
        match &pattern.kind {
            PatternKind::Variant { variant: Value { kind: ValueKind::EnumVariant { variant, .. }, .. }, items, labels } => {
                let assoc = variant.associated_types();
                let variant_labels = variant.labels();

                // Test the lengths
                // Test the labels
                // Check each pattern
                if assoc.len() != variant_labels.len() ||
                    items.len() != labels.len() ||
                    assoc.len() != items.len()
                {
                    self.debugger.throw_diagnostic(TypeCheckError::CouldNotInfer(pattern.span));
                }

                for ((ty1, label1), (pat, pat_label)) in
                    assoc.iter()
                         .zip(variant_labels.iter())
                         .zip(items.iter().zip(labels.iter()))
                {
                    self.check_pattern(ty1, pat);

                    if let (Some(label1), Some(label2)) = (label1, pat_label) {
                        if label1 != label2 {
                            self.debugger.throw_diagnostic(TypeCheckError::ExpectedLabel { expected: label1.clone(), found: label2.clone(), span: pat.span });
                        }
                    }
                }
            }
            PatternKind::Literal { value } => {
                match value.typ.kind() {
                    TypeKind::Float { .. } => {
                        self.debugger.throw_diagnostic(TypeCheckError::CantMatchFloat(value.span.clone().unwrap_or_else(Span::empty)));
                    }
                    TypeKind::Struct(struct_ref) => {
                        if struct_ref.float_repr() {
                            self.debugger.throw_diagnostic(TypeCheckError::CantMatchFloat(value.span.clone().unwrap_or_else(Span::empty)));
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        self.check_type(&ty, &pattern.match_type);
    }

    fn check_polymorphic(&mut self, polymorphic: &Monomorphizer, span: &Option<Span>) {
        match polymorphic.degrees() {
            0 => {
                self.debugger.throw_diagnostic(TypeCheckError::FuncNotFound(span.unwrap_or_default()));
            }
            _ => {
                let mut spans = polymorphic.open_possibilities()
                                           .iter()
                                           .map(|possibility| possibility.span())
                                           .collect::<Vec<_>>();
                if let Some(span) = span {
                    spans.insert(0, *span);
                }
                self.debugger.throw_diagnostic(TypeCheckError::AmbiguousFunc(spans));
            }
        }
    }

    fn check_if_value(&mut self, if_value: &IfValue, if_type: &Type, return_type: &Type, mut spans: Vec<Span>) {
        self.check_value(if_value.condition.as_ref(), &if_value.condition.typ);

        // Get the value of the if
        /*if */self.check_codeblock(&if_value.positive, Some(if_type), return_type);//.is_err() {
        /*    spans.extend(if_value.positive.span());
        }*/

        if let Some(negative) = &if_value.negative {
            match negative {
                IfBranch::CodeBlock(negative_block) => {
                    /*if */self.check_codeblock(negative_block, Some(if_type), return_type);//.is_err() {
                    //    spans.extend(if_value.positive.span());
                    //}
                }
                IfBranch::Else(else_if_branch) => return self.check_if_value(else_if_branch, if_type, return_type, spans),
                IfBranch::ElseLet(match_value) => {
                    self.check_value(&*match_value.discriminant, return_type);

                    let scrut_ty = &match_value.discriminant.typ;

                    // Check that scrut_ty can be matched
                    for arm in &match_value.branches {
                        self.check_codeblock(&arm.code, Some(if_type), return_type);

                        self.check_pattern(scrut_ty, &arm.pattern);
                    }
                }
            }
        }

        // todo: add the if thingy back
        /*if !spans.is_empty() {
            self.debugger.throw
                .throw(ErrorCode::MismatchedIfBranchTypes, spans);
        }*/
    }

    fn check_type(&self, place: &Type, value: &Type)  {
        match (place.kind(), value.kind()) {
            (_, TypeKind::Infer { .. }) | (TypeKind::Infer { .. }, _) => self.debugger.throw_diagnostic(TypeCheckError::CouldNotInfer(value.span().unwrap_or_default())),

            (ty1, ty2) if ty1 == ty2 => {}
            (_, TypeKind::Divergent) => {}

            (TypeKind::Tuple(types1, labels1), TypeKind::Tuple(types2, labels2)) => {
                if types1.len() != labels1.len() ||
                   types2.len() != labels2.len() ||
                   types1.len() != types2.len()
                {
                    self.debugger.throw_diagnostic(TypeCheckError::MismatchedTypes(place.clone(), value.clone()));
                }

                for ((ty1, label1), (ty2, label2)) in
                    types1.iter()
                      .zip(labels1.iter())
                      .zip(types2.iter().zip(labels2.iter()))
                {
                    self.check_type(ty1, ty2);

                    if let (Some(label1), Some(label2)) = (label1, label2) {
                        if label1 != label2 {
                            // There is an error
                            self.debugger.throw_diagnostic(TypeCheckError::MismatchedLabel(label1.clone(), label2.clone()));
                        }
                    }
                }
            }

            _ => self.debugger.throw_diagnostic(TypeCheckError::MismatchedTypes(place.clone(), value.clone())),
        }
    }
}

#[derive(Debug)]
enum TypeCheckError {
    CouldNotInfer(Span),
    MismatchedTypes(Type, Type),
    MismatchedLabel(String, String),

    CantMatchFloat(Span),

    ExpectedLabel { expected: String, found: String, span: Span },

    IsNotAFunc(Span),

    MissingParams(Span),
    ExtraParams(Vec<Span>),

    FuncNotFound(Span),
    AmbiguousFunc(Vec<Span>),

    OperatorDNE(String, Span),

    SymbolNotAValue(String, Span),
    MemberNotAValue(Type, String, Span),
    CodeAfterUnreachable(Span),
    UnreachableCode(Span),
}

impl IntoDiagnostic for TypeCheckError {
    fn into_diagnostic(self) -> errors::Diagnostic {
        match self {
            Self::CouldNotInfer(span) => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "could_not_infer",
                                format!("could not infer type for"),
                                vec![ CodeLocation::new(span, None) ])
            }

            Self::MismatchedTypes(t1, t2) => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "mismatched_types",
                                format!("cannot assign {t2} to {t1}"),
                                t1.span().into_iter().chain(t2.span()).map(|span| CodeLocation::new(span, None)).collect())
            }
            TypeCheckError::MismatchedLabel(_, _) => todo!(),
            TypeCheckError::CantMatchFloat(_) => todo!(),
            TypeCheckError::ExpectedLabel { expected, found, span } => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "expected_label",
                                format!("expected label `{expected}`, found `{found}`"),
                                vec![ CodeLocation::new(span, None) ])
            }
            TypeCheckError::IsNotAFunc(_) => todo!(),
            TypeCheckError::MissingParams(_) => todo!(),
            TypeCheckError::ExtraParams(_) => todo!(),
            TypeCheckError::FuncNotFound(span) => {
                Diagnostic::new(DiagnosticLevel::Error,
                                "func_not_found",
                                format!("could not find function in scope"),
                                vec![ CodeLocation::new(span, None) ])
            }
            TypeCheckError::AmbiguousFunc(_) => todo!(),
            TypeCheckError::OperatorDNE(_, _) => todo!(),
            TypeCheckError::SymbolNotAValue(_, _) => todo!(),
            TypeCheckError::MemberNotAValue(_, _, _) => todo!(),
            TypeCheckError::CodeAfterUnreachable(_) => todo!(),
            TypeCheckError::UnreachableCode(_) => todo!(),
        }
    }
}