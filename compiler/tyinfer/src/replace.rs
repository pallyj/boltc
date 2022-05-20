use std::collections::HashMap;

use blir::{code::{CodeBlock, Statement, StatementKind},
           scope::ScopeRef,
           typ::{Type, TypeKind},
           value::{IfBranch, IfValue, Value, ValueKind},
           BlirContext, SomeFunction, Symbol, pattern::{PatternKind, Pattern}};
use errors::{debugger::Debugger, error::ErrorCode, Span};

use crate::{variant::TypeVariant, context::TypeInferContext};

pub struct TypeReplaceContext<'a, 'b> {
    pub(crate) infer_table:      HashMap<u64, TypeVariant>,
    pub(crate) context:          &'a BlirContext,
    pub(crate) debugger:         &'a mut Debugger<'b>,
    pub(crate) is_final_run:     bool,
}

impl<'a, 'b> TypeReplaceContext<'a, 'b> {
    pub fn replace_codeblock(&mut self, codeblock: &mut CodeBlock, scope: &ScopeRef) {
        for smt in codeblock.statements_mut() {
            self.replace_smt(smt, scope);
        }
    }

    pub fn replace_variable(&mut self, typ: &mut Type, value: &mut Value, scope: &ScopeRef) {
        let span = typ.span();
        self.replace_type(typ, &span);
        self.replace_value(value, scope);
    }

    pub fn replace_smt(&mut self, statement: &mut Statement, scope: &ScopeRef) {
        match &mut statement.kind {
            StatementKind::Eval { value, .. } => self.replace_value(value, scope),
            StatementKind::Bind { typ, value, .. } => {
                let span = typ.span();
                self.replace_type(typ, &span);
                if let Some(value) = value {
                    self.replace_value(value, scope);
                    self.meet_types(typ, &mut value.typ);
                    self.replace_type(typ, &span);
                }
            }
            StatementKind::Return { value } => {
                if let Some(value) = value {
                    self.replace_value(value, scope)
                }
            }
        }
    }

    pub fn replace_value(&mut self, value: &mut Value, scope: &ScopeRef) {
        // println!("Replacing {value:?}");

        self.replace_type(&mut value.typ, &value.span);

        match &mut value.kind {
            ValueKind::Polymorphic(monomorphizer) => {
                let args = match value.typ.kind() {
                    TypeKind::Function { params, .. } => params,
                    TypeKind::Method { params, .. } => params,
                    _ => return,
                };

                //println!("Polymorph with {}", monomorphizer.degrees());

                if let Some(resolved_function) = monomorphizer.resolve() {
                    self.replace_function(value, resolved_function);
                } else {
                    monomorphizer.filter_types(args);
                }
            }

            ValueKind::PolymorphicMethod { reciever, polymorphic } => {
                let args = match value.typ.kind() {
                    TypeKind::Function { params, .. } => params,
                    TypeKind::Method { params, .. } => params,
                    _ => return,
                };

                if let Some(resolved_function) = polymorphic.resolve() {
                    let parent = std::mem::take(reciever.as_mut());
                    self.replace_member_function(value, parent, resolved_function);
                } else {
                    polymorphic.filter_types(args);
                }

                // if polymorphic.degrees() == 0 {
                // self.debugger.throw_single(ErrorCode::FunctionSigNotFound, &value.span);
                // return;
                // }
            }

            ValueKind::Tuple(tuple_items) => {
                for item in tuple_items {
                    self.replace_value(item, scope);

                    self.replace_type(&mut item.typ, &item.span);
                }
            }

            ValueKind::FuncCall { function, args } => {
                if let ValueKind::PolymorphicMethod { polymorphic, .. } |
                       ValueKind::Polymorphic(polymorphic) = &mut function.kind {
                    polymorphic.filter_labels(&args.labels);
                } else if let ValueKind::VariantLiteral(named) = &function.kind {
                    let TypeKind::Enum(enum_type) = value.typ.kind() else {
                        // TODO: Throw an error?
                        return
                    };
    
                    let Some(variant) = enum_type.get_variant(named) else {
                        // TODO: Throw an error?
                        return
                    };
                    let enum_variant_value = ValueKind::EnumVariant { of_enum: enum_type.clone(), variant: variant.clone() };
    
                    function.set_kind(enum_variant_value);

                    // TODO: Add a check for the labels being the same
                    let function_type = TypeKind::Function { return_type: Box::new(value.typ.clone()),
                                                             params: variant.associated_types().clone(),
                                                             labels: variant.labels().clone() };

                    function.set_type(function_type.anon());
                }

                self.replace_value(function, scope);

                let params = match function.typ.kind_mut() {
                    TypeKind::Function { params, .. } => params,
                    TypeKind::Method { params, .. } => params,
                    _ => return,
                };

                for (arg, param) in args.args.iter_mut().zip(params) {
                    self.meet_types(param, &mut arg.typ);

                    self.replace_value(arg, scope);
                    let span = param.span;
                    self.replace_type(param, &span);
                }

                self.replace_value(function, scope);

                let params = match function.typ.kind_mut() {
                    TypeKind::Function { params, .. } => params,
                    TypeKind::Method { params, .. } => params,
                    _ => return,
                };

                for (arg, param) in args.args.iter_mut().zip(params) {
                    self.meet_types(param, &mut arg.typ);

                    self.replace_value(arg, scope);
                    let span = param.span;
                    self.replace_type(param, &span);
                }

                let return_type = match function.typ.kind() {
                                      TypeKind::Function { return_type, .. } => return_type,
                                      TypeKind::Method { return_type, .. } => return_type,
                                      _ => return,
                                  }.as_ref()
                                   .clone();

                value.set_type(return_type);
            }

            ValueKind::Operator(operator) => {
                let operator_params = match value.typ.kind() {
                    TypeKind::Function { params, .. } => params,
                    _ => return,
                };

                let container_type = operator_params.first().unwrap();
                if matches!(container_type.kind(), TypeKind::Infer { .. }) {
                    return;
                }

                // Now we turn it into a polymorphizer
                let operator_name = format!("op~{operator}");

                let Some(Symbol::Function(polymorphizer)) = container_type.lookup_static_item(&operator_name) else {
                    let operator = operator.to_string();
					// Throw an error
                    value.kind = ValueKind::Error;
					self.debugger.throw_single(ErrorCode::OperatorNotDefined(operator, type_to_string(container_type)), &value.span);
					return;
				};

                value.set_kind(ValueKind::Polymorphic(polymorphizer));

                self.replace_value(value, scope);
            }

            ValueKind::Member { parent, member } => {
                self.replace_value(parent, scope);

                if matches!(parent.typ.kind(), TypeKind::Infer { .. }) {
                    return;
                }

                let Some(resolved_member) = parent.typ.lookup_instance_item(member, scope) else {
                    let member = member.clone();
                    value.kind = ValueKind::Error;
                    self.debugger.throw_single(ErrorCode::MemberNotFound { name: member }, &value.span);
                    return
				};

                let parent = std::mem::replace(parent.as_mut(), ValueKind::Unit.anon(TypeKind::Void.anon()));

                self.replace_member(value, parent, resolved_member, scope);
            }

            ValueKind::InstanceVariable { reciever, .. } => {
                self.replace_value(reciever, scope);
            }

            ValueKind::If(if_value) => if let Some(ty) = self.replace_if_value(if_value, scope, 0) {
                value.typ.set_kind(ty);
            }

            ValueKind::Closure(closure) => {
                self.replace_codeblock(&mut closure.code, scope);
            }

            ValueKind::TupleField(value, _) => {
                self.replace_value(value, scope);
            }

            ValueKind::VariantLiteral(named) => {
                let TypeKind::Enum(enum_type) = value.typ.kind() else {
                    return
                };

                let Some(variant) = enum_type.get_variant(named) else {
                    return
                };
                let enum_variant_value = ValueKind::EnumVariant { of_enum: enum_type.clone(), variant };

                value.set_kind(enum_variant_value);
            }

            ValueKind::Match(match_value) => {
                self.replace_value(&mut match_value.discriminant, scope);

                for branch in &mut match_value.branches {
                    self.meet_types(&mut branch.pattern.match_type, &mut match_value.discriminant.typ);

                    self.replace_pattern(&mut branch.pattern, scope);

                    self.replace_codeblock(&mut branch.code, scope);
                }
            }

            _ => {}
        }
    }

    fn meet_types(
        &mut self,
        ty1: &mut Type,
        ty2: &mut Type)
    {
        match (&ty1.kind, &ty2.kind) {
            (TypeKind::Infer { .. }, TypeKind::Infer { .. }) => {},
            (TypeKind::Infer { key }, _) => {
                self.infer_table.insert(*key, TypeInferContext::variant(&ty2));
            },
            (_, TypeKind::Infer { key }) => {
                self.infer_table.insert(*key, TypeInferContext::variant(&ty1));
            },
            _ => {}
        }
    }

    fn replace_pattern(
        &mut self,
        pattern: &mut Pattern,
        scope: &ScopeRef)
    {
        let span = pattern.span.clone();
        self.replace_type(pattern.match_type_mut(), &Some(span));
        match &mut pattern.kind {
            PatternKind::Literal { value } => self.replace_value(value, scope),
            PatternKind::Tuple { items, .. } => {
                for i in items {
                    self.replace_pattern(i, scope);
                }
            }
            PatternKind::Variant { variant, items, .. } => {
                self.replace_value(variant, scope);
                for i in items {
                    self.replace_pattern(i, scope);
                }
            }
            _ => {}
        }
    }

    fn replace_if_value(&mut self, if_value: &mut IfValue, scope: &ScopeRef, n: usize) -> Option<TypeKind> {
        self.replace_value(&mut if_value.condition, scope);

        self.replace_codeblock(&mut if_value.positive, scope);

        match &mut if_value.negative {
            Some(IfBranch::CodeBlock(else_block)) => {
                self.replace_codeblock(else_block, scope);
            }
            Some(IfBranch::Else(else_if_block)) => {
                self.replace_if_value(else_if_block, scope, n + 1);
            }
            None => if n == 0 {
                // Set the type to 0 or diverges
                let ty = if if_value.positive.escapes() {
                    TypeKind::Divergent
                } else {
                    TypeKind::Void
                };

                return Some(ty)
            }
        }

        None
    }

    fn replace_function(&mut self, value: &mut Value, function: SomeFunction) {
        match function {
            SomeFunction::Function(func_ref) => {
                let function_type = func_ref.take_typ();

                value.set_type(function_type);
                value.set_kind(ValueKind::StaticFunc(func_ref))
            }

            SomeFunction::StaticMethod(method_ref) => {
                let method_type = method_ref.take_typ();

                value.set_type(method_type);
                value.set_kind(ValueKind::StaticMethod(method_ref))
            }

            SomeFunction::ExternFunction(func_ref) => {
                let function_type = func_ref.take_typ();

                value.set_type(function_type);
                value.set_kind(ValueKind::ExternFunc(func_ref))
            }

            _ => {}
        }
    }

    fn replace_member(&mut self, value: &mut Value, parent: Value, member: Symbol, scope: &ScopeRef) {
        match member {
            Symbol::Type(ty) => {
                value.set_kind(ValueKind::Metatype(ty.clone()));
                value.set_type(TypeKind::Metatype(Box::new(ty.anon())).anon());
            }
            Symbol::Value(resolved_value) => {
                value.set_kind(resolved_value.kind);
                value.set_type(resolved_value.typ);
            }
            Symbol::InstanceVariable(instance_variable) => {
                value.set_type(instance_variable.borrow().typ.clone());
                value.set_kind(ValueKind::InstanceVariable { reciever: Box::new(parent),
                                                             var:      instance_variable, });
            }
            Symbol::Constant(constant) => {
                let constant_value = constant.borrow().value.clone();

                value.set_type(constant_value.typ);
                value.set_kind(constant_value.kind);
            }
            Symbol::Function(monomorphizer) => {
                value.set_kind(ValueKind::PolymorphicMethod { reciever:    Box::new(parent),
                                                              polymorphic: monomorphizer, });

                self.replace_value(value, scope);
            }
            Symbol::TupleField(ty, field_number) => {
                value.set_kind(ValueKind::TupleField(Box::new(parent), field_number));
                value.set_type(ty);
            }
            Symbol::EnumCase(enum_ref, case_ref) => {
                // If the enum variant is an empty tuple,
                // Set value to an enum variant
                if case_ref.associated_types().len() == 0 {
                    value.set_type(enum_ref.get_type().anon());
                    value.set_kind(ValueKind::EnumVariant { of_enum: enum_ref, variant: case_ref });
                }
                // If the enum variant is a non-empty tuple
                // Set the value to an enum variant
                // With a function type
                else {
                    let enum_type = enum_ref.get_type().anon();

                    let function_type = TypeKind::Function { return_type: Box::new(enum_type),
                                                             params: case_ref.associated_types().clone(),
                                                             labels: case_ref.labels().clone() };

                    value.set_type(function_type.anon());
                    value.set_kind(ValueKind::EnumVariant { of_enum: enum_ref.clone(), variant: case_ref });
                }
            }
        }
    }

    fn replace_member_function(&mut self, value: &mut Value, parent: Value, function: SomeFunction) {
        match function {
            SomeFunction::StaticMethod(method_ref) => {
                let method_type = method_ref.take_typ();

                value.set_type(method_type);
                value.set_kind(ValueKind::StaticMethod(method_ref))
            }

            SomeFunction::InstanceMethod(method) => {
                let method_type = method.take_typ();

                value.set_type(method_type);
                value.set_kind(ValueKind::InstanceMethod { reciever: Box::new(parent),
                                                           method })
            }

            _ => {}
        }
    }

    pub fn replace_type(&mut self, typ: &mut Type, span: &Option<Span>) {
        match typ.kind_mut() {
            TypeKind::Infer { key } => {
                let Some(variant) = self.infer_table.get(key) else {
					// Throw an error
					return;
				};

                if let Some(concrete_type) = self.type_for_variant(variant) {
                    typ.set_kind(concrete_type);
                    self.replace_type(typ, span);
                }
            }

            TypeKind::Function { return_type, params, .. } => {
                self.replace_type(return_type, span);

                for param in params {
                    self.replace_type(param, span);
                }
            }

            TypeKind::Tuple(tuple_items, ..) => {
                for tuple_item in tuple_items {
                    self.replace_type(tuple_item, span);
                }
            }

            TypeKind::Member { parent, .. } => self.replace_type(parent, span),

            TypeKind::Method { reciever, .. } => self.replace_type(reciever, span),

            _ => {}
        }
    }

    fn type_for_variant(&self, variant: &TypeVariant) -> Option<TypeKind> {
        match variant {
            TypeVariant::Diverges => Some(TypeKind::Divergent),
            TypeVariant::Void => Some(TypeKind::Void),
            TypeVariant::LlvmInt { bits } => Some(TypeKind::Integer { bits: *bits as u64 }),
            TypeVariant::LlvmFloat { bits } => Some(TypeKind::Float { bits: *bits as u64 }),
            TypeVariant::LlvmBool => Some(TypeKind::Integer { bits: 1 }),
            TypeVariant::LlvmString => Some(TypeKind::StrSlice),
            TypeVariant::Struct(r#struct) => Some(TypeKind::Struct(r#struct.clone())),
            TypeVariant::Enum(r#enum) => Some(TypeKind::Enum(r#enum.clone())),

            TypeVariant::SomeInteger if self.is_final_run => self.context
                                                                 .default_integer_repr
                                                                 .clone()
                                                                 .map(TypeKind::Struct),
            TypeVariant::SomeFloat if self.is_final_run => self.context
                                                               .default_float_repr
                                                               .clone()
                                                               .map(TypeKind::Struct),
            TypeVariant::SomeBool if self.is_final_run => self.context.default_bool_repr.clone().map(TypeKind::Struct),
            TypeVariant::SomeString if self.is_final_run => self.context.default_string_repr.clone().map(TypeKind::Struct),

            TypeVariant::Function { params,
                                    labels,
                                    return_type, } => Some(TypeKind::Function { return_type: return_type.clone(),
                                                                                params:      params.clone(),
                                                                                labels:      labels.clone(), }),

            TypeVariant::Tuple(tuple_items, labels) => Some(TypeKind::Tuple(tuple_items.clone(), labels.clone())),

            _ => None,
        }
    }
}

fn type_to_string(ty: &Type) -> String {
    match ty.kind() {
        TypeKind::Struct(r#struct) => format!("struct `{}`", r#struct.name()),
        TypeKind::Enum(r#enum) => format!("enum `{}`", r#enum.name()),

        TypeKind::Void => "()".to_string(),
        TypeKind::Divergent => "!".to_string(),

        TypeKind::Integer { bits } => format!("intrinsics.i{bits}"),
        TypeKind::Float { bits } => format!("intrinsics.f{bits}"),

        _ => "unknown".to_string(),
    }
}
