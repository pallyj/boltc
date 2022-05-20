use std::cmp::Ordering;

use blir::{code::{CodeBlock, FunctionRef, MethodRef, Statement, StatementKind},
           typ::{StructRef, Type, TypeKind, EnumRef},
           value::{ConstantRef, IfBranch, IfValue, Value, ValueKind, VarRef},
           Library, Monomorphizer, pattern::{Pattern, PatternKind}};
use errors::{debugger::Debugger, error::ErrorCode, Span};

pub struct TypeCheckPass<'a, 'b> {
    debugger: &'a mut Debugger<'b>,
}

impl<'a, 'b> TypeCheckPass<'a, 'b> {
    pub fn new(debugger: &'a mut Debugger<'b>) -> Self { Self { debugger } }

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

        if let Err(error) = self.check_codeblock(func_code, func_return_type, func_return_type) {
            self.handle_return_yield_error(error, &func_return_type.span);
        }
    }

    fn check_method(&mut self, func: &MethodRef) {
        let borrowed = func.borrow();
        let func_return_type = borrowed.info.return_type();
        let func_code = &borrowed.code;

        if let Err(error) = self.check_codeblock(func_code, func_return_type, func_return_type) {
            self.handle_return_yield_error(error, &func_return_type.span);
        }
    }

    fn check_var(&mut self, var: &VarRef) {
        let variable = var.borrow();

        let Some(value) = &variable.default_value else { return };

        if let Err(error) = self.check_type(&variable.typ, &value.typ) {
            self.handle_var_error(error, &variable.span);
        }
    }

    fn check_const(&mut self, var: &ConstantRef) {
        let variable = var.borrow();

        if let Err(error) = self.check_type(&variable.typ, &variable.value.typ) {
            self.handle_var_error(error, &variable.span);
        }
    }

    fn check_codeblock(&mut self, code_block: &CodeBlock, code_block_type: &Type, return_type: &Type) -> Result<(), TypeCheckError> {
        for smt in code_block.statements() {
            self.check_smt(smt, return_type);
        }

        // TODO: Handle the error
        self.check_type(code_block_type, &code_block.typ())
    }

    fn check_smt(&mut self, statement: &Statement, return_type: &Type) {
        match &statement.kind {
            StatementKind::Eval { value, .. } => self.check_value(value, return_type),
            StatementKind::Bind { typ, value, .. } => {
                let Some(value) = value else { return };
                
                self.check_value(value, return_type);

                if let Err(error) = self.check_type(typ, &value.typ) {
                    self.handle_let_error(error, statement);
                }
            }
            StatementKind::Return { value } => {
                let check_result = if let Some(value) = value {
                    self.check_value(value, return_type);
                    self.check_type(return_type, &value.typ)
                } else {
                    self.check_type(return_type, &TypeKind::Void.anon())
                };

                if let Err(error) = check_result {
                    self.handle_return_error(error, statement);
                }
            }
        }
    }

    fn check_value(&mut self, value: &Value, return_type: &Type) {
        // TODO: Move this to another function
        match value.typ.kind() {
            TypeKind::Error => panic!("Compiler Error: error type"),

            TypeKind::SomeBool | TypeKind::SomeInteger | TypeKind::SomeFloat | TypeKind::SomeFunction | TypeKind::Infer { .. } => {
                self.debugger
                    .throw_single(ErrorCode::AmbiguousTy, &value.span);
            }

            TypeKind::Named(name) => self.debugger
                                         .throw_single(ErrorCode::SymNotAType { name: name.clone() }, &value.span),
            TypeKind::Member { member, .. } => self.debugger
                                                   .throw_single(ErrorCode::MemberNotATy { name: member.clone() },
                                                                 &value.span),

            _ => {}
        }

        match &value.kind {
            ValueKind::Assign(place, value) => {
                self.check_value(place, return_type);
                self.check_value(value, return_type);

                // TODO: Check that they have the same type
            }

            ValueKind::Closure(closure) => {
                let TypeKind::Function { return_type, .. } = value.typ.kind() else {
                    self.debugger.throw_single(ErrorCode::IsNotAFunc, &value.span);
                    return
                };

                if let Err(error) = self.check_codeblock(&closure.code, return_type, return_type) {
                    self.handle_return_yield_error(error, &value.span);
                }
            }
            ValueKind::FuncCall { function, args } => {
                self.check_value(function, return_type);

                let params = match &function.typ.kind {
                    TypeKind::Function { params, labels, .. } => {
                        for (i, (label1, label2)) in labels.iter().zip(&args.labels).enumerate() {
                            if label1 != label2 {
                                let label1 = label1.clone().unwrap_or_else(|| "_".to_string());
                                let label2 = label2.clone().unwrap_or_else(|| "_".to_string());
                                // Throw an error
                                self.debugger.throw_single(ErrorCode::ExpectedFound(label1, label2), &args.args[i].span);
                            }
                        }
                        params
                    },
                    TypeKind::Method { params, .. } => {
                        params
                    },
                    _ => {
                        self.debugger
                            .throw_single(ErrorCode::IsNotAFunc, &value.span);
                        return;
                    }
                };

                match params.len().cmp(&args.args.len()) {
                    Ordering::Greater => {
                        // Less params
                        self.debugger
                            .throw_single(ErrorCode::MissingParams, &value.span);
                    }
                    Ordering::Less => {
                        // Extra params
                        let spans = args.args
                                        .iter()
                                        .skip(params.len())
                                        .filter_map(|arg| arg.span)
                                        .collect();
                        self.debugger.throw(ErrorCode::ExtraParams, spans);
                    }
                    Ordering::Equal => {}
                }

                for (param, arg) in params.iter().zip(&args.args) {
                    self.check_value(arg, return_type);

                    if let Err(error) = self.check_type(param, &arg.typ) {
                        self.handle_call_error(error, arg);
                    }
                }
            }
            ValueKind::If(if_value) => self.check_if_value(if_value, &value.typ, return_type, vec![]),
            ValueKind::InstanceMethod { reciever, .. } => self.check_value(reciever, return_type),
            ValueKind::InstanceVariable { reciever, .. } => self.check_value(reciever, return_type),

            ValueKind::Error => panic!("Compiler Error: error value"),
            ValueKind::Member { member, .. } => self.debugger
                                                    .throw_single(ErrorCode::MemberNotAVal { name: member.clone() },
                                                                  &value.span),
            ValueKind::Named(name) => self.debugger
                                          .throw_single(ErrorCode::SymNotAValue { name: name.clone() }, &value.span),
            ValueKind::Operator(name) => {
                self.debugger.throw_single(ErrorCode::OperatorDNE(name.clone()), &value.span);
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
                    match self.check_codeblock(&arm.code, &value.typ, return_type) {
                        Err(e) => self.handle_return_yield_error(e, &arm.code.typ().span),
                        _ => {}
                    };

                    self.check_pattern(scrut_ty, &arm.pattern);
                }
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
                    assoc.len() != items.len() {
                    self.debugger
                        .throw(ErrorCode::AmbiguousTy, vec![ pattern.span ]);
                }

                for ((ty1, label1), (pat, pat_label)) in
                    assoc.iter()
                         .zip(variant_labels.iter())
                         .zip(items.iter().zip(labels.iter()))
                {
                    self.check_pattern(ty1, pat);

                    if let (Some(label1), Some(label2)) = (label1, pat_label) {
                        if label1 != label2 {
                            self.debugger.throw_single(ErrorCode::ExpectedFound(label1.clone(), label2.clone()), &Some(pat.span));
                        }
                    }
                }
            }
            PatternKind::Literal { value } => {
                match value.typ.kind() {
                    TypeKind::Float { .. } => {
                        self.debugger.throw_single(ErrorCode::CantMatchFloat, &value.span);
                    }
                    TypeKind::Struct(struct_ref) => {
                        if struct_ref.float_repr() {
                            self.debugger.throw_single(ErrorCode::CantMatchFloat, &value.span);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        match self.check_type(&ty, &pattern.match_type) {
            Err(e) => self.handle_var_error(e, &pattern.span),
            _ => {}
        }
    }

    fn check_polymorphic(&mut self, polymorphic: &Monomorphizer, span: &Option<Span>) {
        match polymorphic.degrees() {
            0 => {
                self.debugger
                    .throw_single(ErrorCode::FunctionSigNotFound, span);
            }
            _ => {
                let mut spans = polymorphic.open_possibilities()
                                           .iter()
                                           .map(|possibility| possibility.span())
                                           .collect::<Vec<_>>();
                if let Some(span) = span {
                    spans.insert(0, *span);
                }
                self.debugger.throw(ErrorCode::AmbiguousFunc, spans)
            }
        }
    }

    fn check_if_value(&mut self, if_value: &IfValue, if_type: &Type, return_type: &Type, mut spans: Vec<Span>) {
        self.check_value(if_value.condition.as_ref(), &if_value.condition.typ);

        // Get the value of the if
        if self.check_codeblock(&if_value.positive, if_type, return_type).is_err() {
            spans.extend(if_value.positive.span());
        }

        if let Some(negative) = &if_value.negative {
            match negative {
                IfBranch::CodeBlock(negative_block) => {
                    if self.check_codeblock(negative_block, if_type, return_type).is_err() {
                        spans.extend(if_value.positive.span());
                    }
                }
                IfBranch::Else(else_if_branch) => return self.check_if_value(else_if_branch, if_type, return_type, spans),
            }
        }

        if !spans.is_empty() {
            self.debugger
                .throw(ErrorCode::MismatchedIfBranchTypes, spans);
        }
    }

    fn check_type(&self, place: &Type, value: &Type) -> Result<(), TypeCheckError> {
        match (place.kind(), value.kind()) {
            (_, TypeKind::Infer { .. }) | (TypeKind::Infer { .. }, _) => Err(TypeCheckError::CouldNotInfer),

            (ty1, ty2) if ty1 == ty2 => Ok(()),
            (_, TypeKind::Divergent) => Ok(()),

            (TypeKind::Tuple(types1, labels1), TypeKind::Tuple(types2, labels2)) => {
                if types1.len() != labels1.len() ||
                   types2.len() != labels2.len() ||
                   types1.len() != types2.len() {
                    return Err(TypeCheckError::MismatchedTypes(place.clone(), value.clone()))
                }

                for ((ty1, label1), (ty2, label2)) in
                    types1.iter()
                      .zip(labels1.iter())
                      .zip(types2.iter().zip(labels2.iter()))
                {
                    self.check_type(ty1, ty2)?;

                    if let (Some(label1), Some(label2)) = (label1, label2) {
                        if label1 != label2 {
                            // There is an error
                            return Err(TypeCheckError::MismatchedLabel(label1.clone(), label2.clone()))
                        }
                    }
                }

                Ok(())
            }

            _ => Err(TypeCheckError::MismatchedTypes(place.clone(), value.clone())),
        }
    }

    fn handle_let_error(&mut self, error: TypeCheckError, statement: &Statement) {
        match error {
            TypeCheckError::CouldNotInfer => self.debugger
                                                 .throw_single(ErrorCode::AmbiguousTy, &statement.span),
            TypeCheckError::MismatchedTypes(t1, t2) => {
                self.debugger
                    .throw_single(ErrorCode::ExpectedFound(self.type_to_string(&t1), self.type_to_string(&t2)),
                                  &statement.span);
            }
            TypeCheckError::MismatchedLabel(l1, l2) => {
                self.debugger
                    .throw_single(ErrorCode::ExpectedFound(l1, l2),
                                  &statement.span);
            }
        }
    }

    fn handle_var_error(&mut self, error: TypeCheckError, span: &Span) {
        match error {
            TypeCheckError::CouldNotInfer => self.debugger
                                                 .throw(ErrorCode::AmbiguousTy, vec![*span]),
            TypeCheckError::MismatchedTypes(t1, t2) => {
                self.debugger
                    .throw(ErrorCode::ExpectedFound(self.type_to_string(&t1), self.type_to_string(&t2)),
                           vec![*span]);
            }
            TypeCheckError::MismatchedLabel(l1, l2) => {
                self.debugger
                    .throw(ErrorCode::ExpectedFound(l1, l2),
                    vec![*span]);
            }
        }
    }

    fn handle_return_error(&mut self, error: TypeCheckError, statement: &Statement) {
        match error {
            TypeCheckError::CouldNotInfer => self.debugger
                                                 .throw_single(ErrorCode::AmbiguousTy, &statement.span),
            TypeCheckError::MismatchedTypes(t1, t2) => {
                self.debugger
                    .throw_single(ErrorCode::ExpectedFound(self.type_to_string(&t1), self.type_to_string(&t2)),
                                  &statement.span);
            }
            TypeCheckError::MismatchedLabel(l1, l2) => {
                self.debugger
                    .throw_single(ErrorCode::ExpectedFound(l1, l2),
                                  &statement.span);
            }
        }
    }

    fn handle_return_yield_error(&mut self, error: TypeCheckError, span: &Option<Span>) {
        match error {
            TypeCheckError::CouldNotInfer => self.debugger.throw_single(ErrorCode::AmbiguousTy, span),
            TypeCheckError::MismatchedTypes(t1, t2) => {
                self.debugger
                    .throw_single(ErrorCode::ExpectedFound(self.type_to_string(&t1), self.type_to_string(&t2)),
                                  span);
            }
            TypeCheckError::MismatchedLabel(l1, l2) => {
                self.debugger
                    .throw_single(ErrorCode::ExpectedFound(l1, l2),
                                  span);
            }
        }
    }

    fn handle_call_error(&mut self, error: TypeCheckError, arg: &Value) {
        match error {
            TypeCheckError::CouldNotInfer => self.debugger
                                                 .throw_single(ErrorCode::AmbiguousTy, &arg.span),
            TypeCheckError::MismatchedTypes(t1, t2) => {
                self.debugger
                    .throw_single(ErrorCode::ExpectedFound(self.type_to_string(&t1), self.type_to_string(&t2)),
                                  &arg.span);
            }
            TypeCheckError::MismatchedLabel(l1, l2) => {
                self.debugger
                    .throw_single(ErrorCode::ExpectedFound(l1, l2),
                                  &arg.span);
            }
        }
    }

    fn type_to_string(&self, ty: &Type) -> String {
        match ty.kind() {
            TypeKind::Struct(r#struct) => format!("struct `{}`", r#struct.name()),
            TypeKind::Enum(r#enum) => format!("enum `{}`", r#enum.name()),

            TypeKind::Void => "void".to_string(),
            TypeKind::Divergent => "!".to_string(),

            TypeKind::Integer { bits } => format!("intrinsics.i{bits}"),
            TypeKind::Float { bits } => format!("intrinsics.f{bits}"),
            TypeKind::StrSlice => format!("intrinsic.strslice"),
            TypeKind::Infer { .. } => format!("{{}}"),

            TypeKind::Tuple(types, labels) => {
                let types =
                    types.iter()
                        .zip(labels)
                        .map(|(ty, lab)| if let Some(label) = lab {
                            format!("{label}: {ty:?}")
                        } else {
                            format!("{ty:?}")
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    
                format!("({})", types)
            }

            TypeKind::Function { return_type, params, labels } => {
                let types =
                    params.iter()
                        .zip(labels)
                        .map(|(ty, lab)| if let Some(label) = lab {
                            format!("{label}: {}", self.type_to_string(ty))
                        } else {
                            format!("{}", self.type_to_string(ty))
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    
                format!("({}): {}", types, self.type_to_string(return_type))
            }

            _ => "unknown".to_string(),
        }
    }
}

enum TypeCheckError {
    CouldNotInfer,
    MismatchedTypes(Type, Type),
    MismatchedLabel(String, String)
}
