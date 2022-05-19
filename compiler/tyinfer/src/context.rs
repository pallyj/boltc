use std::collections::HashMap;

use blir::{code::{CodeBlock, Statement, StatementKind},
           scope::ScopeRef,
           typ::{Type, TypeKind},
           value::{IfBranch, IfValue, Value, ValueKind},
           BlirContext, pattern::{Pattern, PatternKind}};
use errors::{debugger::Debugger, error::ErrorCode};
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};

use crate::{replace::TypeReplaceContext, variant::TypeVariant};

pub struct TypeInferContext<'a, 'b> {
    checker:    VarlessTypeChecker<TypeVariant>,
    infer_keys: HashMap<u64, TcKey>,
    debugger:   &'a mut Debugger<'b>,
    context:    &'a BlirContext,
}

impl<'a, 'b> TypeInferContext<'a, 'b> {
    pub fn new(debugger: &'a mut Debugger<'b>, context: &'a BlirContext) -> Self {
        Self { checker: VarlessTypeChecker::new(),
               infer_keys: HashMap::new(),
               debugger,
               context }
    }

    pub fn replace<'c>(&'c mut self) -> TypeReplaceContext<'c, 'b> {
        let Ok(constraint_table) = self.checker.clone().type_check_preliminary() else {
			panic!()
		};

        TypeReplaceContext { constraint_table,
                             infer_keys: &self.infer_keys,
                             context: self.context,
                             debugger: self.debugger,
                             is_final_run: false }
    }

    pub fn finish<'c>(&'c mut self) -> TypeReplaceContext<'c, 'b> {
        let Ok(constraint_table) = self.checker.clone().type_check_preliminary() else {
			panic!()
		};

        TypeReplaceContext { constraint_table,
                             infer_keys: &self.infer_keys,
                             context: self.context,
                             debugger: self.debugger,
                             is_final_run: true }
    }

    fn constrain_value(&mut self, value: &Value, scope: &ScopeRef) {
        // Rules
        //
        //

        // println!("Constraining {value:?}");

        match &value.kind {
            ValueKind::BoolLiteral(_) => self.constrain_bool(value),
            ValueKind::IntLiteral(_) => self.constrain_int(value),
            ValueKind::FloatLiteral(_) => self.constrain_float(value),
            ValueKind::StringLiteral(lit) => self.constrain_string(value, lit.len()),

            ValueKind::Tuple(tuple_items) => {
                match &value.typ.kind() {
                    TypeKind::Tuple(tuple_items_type, ..) => {
                        for (item, typ) in tuple_items.iter().zip(tuple_items_type) {
                            self.constrain_value(item, scope);
                            self.constrain_two_way(typ, &item.typ);
                        }
                    }
                    _ => panic!(),
                }
            }

            ValueKind::FuncCall { function, args } => {
                let function_type = &function.typ;

                match function_type.kind() {
                    TypeKind::Function { params, .. } | TypeKind::Method { params, .. } => {
						for (param, arg) in params.iter().zip(&args.args) {
							self.constrain_value(arg, scope);

							self.constrain_two_way( param, &arg.typ );
						}
						/*if self.fully_constrained(&params) {
							for (param, arg) in params.iter().zip(&args.args) {
								self.constrain_value(arg, scope);

								self.constrain_one_way( &arg.typ, param );
							}
						} else {
							// Constrain params to args
							for (param, arg) in params.iter().zip(&args.args) {
								self.constrain_value(arg, scope);

								self.constrain_one_way( param, &arg.typ );
							}
						}*/
					}

                    _ => {},
                }
            }

            ValueKind::Member { parent, .. } => {
                self.constrain_value(parent, scope);
            }

            ValueKind::InstanceVariable { reciever, .. } => {
                self.constrain_value(reciever, scope);
            }

            ValueKind::If(if_value) => self.constrain_if_value(if_value, &value.typ, scope),

            ValueKind::Closure(_) => {
                // Constrain the closure's return type to the return type of its code
                self.constrain_func(value);
            }

            ValueKind::TupleField(value, _) => {
                self.constrain_value(value, scope);
            }

            ValueKind::Match(match_value) => {
                self.constrain_value(&match_value.discriminant, scope);

                let match_type = value.typ.clone();

                for branch in &match_value.branches {
                    self.constrain_pattern(&branch.pattern, &match_value.discriminant.typ, scope);

                    self.infer_codeblock(&branch.code, &match_type, scope);
                }
            }

            _ => {}
        }
    }

    fn constrain_if_value(&mut self, if_value: &IfValue, if_type: &Type, scope: &ScopeRef) {
        self.constrain_value(&if_value.condition, scope);
        self.constrain_bool(&if_value.condition);

        self.infer_codeblock(&if_value.positive, if_type, scope);

        match &if_value.negative {
            Some(IfBranch::CodeBlock(else_block)) => {
                self.infer_codeblock(else_block, if_type, scope);
            }
            Some(IfBranch::Else(else_if_block)) => {
                self.constrain_if_value(else_if_block, if_type, scope);
            }
            None => {}
        }
    }

    fn infer_smt(&mut self, smt: &Statement, scope: &ScopeRef) {
        match &smt.kind {
            StatementKind::Bind { typ, value, .. } => {
                if let Some(value) = value.as_ref() {
                    self.constrain_value(value, scope);

                    self.constrain_two_way(&value.typ, typ);
                }
            }

            StatementKind::Eval { value, .. } => self.constrain_value(value, scope),

            StatementKind::Return { value } => {
                if let Some(return_value) = value.as_ref() {
                    let function_return_type = scope.scope_type("return")
                                                    .expect("Compiler Error: Not in a function scope");

                    self.constrain_value(return_value, scope);

                    self.constrain_one_way(&return_value.typ, &function_return_type)
                }
            }
        }
    }

    pub fn infer_variable(&mut self, typ: &Type, value: &Value, scope: &ScopeRef) {
        self.constrain_two_way(typ, &value.typ);

        self.constrain_value(value, scope);
    }

    pub fn infer_codeblock(&mut self, block: &CodeBlock, codeblock_type: &Type, scope: &ScopeRef) {
        let block_implicit_type = block.typ();

        self.constrain_two_way(&block_implicit_type, codeblock_type);

        // I don't think we need to make a block scope

        for smt in block.statements() {
            self.infer_smt(smt, scope);
        }
    }

    pub fn constrain_pattern(&mut self, pattern: &Pattern, typ: &Type, scope: &ScopeRef) {
        self.constrain_two_way(pattern.match_type(), typ);
        
        match &pattern.kind {
            PatternKind::Literal { value } => {
                self.constrain_value(&value, scope);
                self.constrain_two_way(&value.typ, typ);
            },
            PatternKind::Tuple { items, .. } => {
                if let TypeKind::Tuple(tuple, _) = typ.kind() {
                    for (item, tuple_item) in items.iter().zip(tuple) {
                        self.constrain_pattern(item, tuple_item, scope)
                    }
                }
            }
            PatternKind::Variant { variant, items, .. } => {
                self.constrain_value(&variant, scope);
                self.constrain_two_way(&variant.typ, typ);

                if let ValueKind::EnumVariant { variant, .. } = &variant.kind {
                    let tuple = variant.associated_types();
                    for (item, tuple_item) in items.iter().zip(tuple.iter()) {
                        self.constrain_pattern(item, tuple_item, scope)
                    }
                }
            }
            _ => {}
        }
    }

    fn constrain_bool(&mut self, value: &Value) {
        // println!("{value:?} <- some Bool");
        if let Some(infer_key) = self.infer_key(&value.typ) {
            let _constraint = self.checker
                                  .impose(infer_key.concretizes_explicit(TypeVariant::SomeBool));

            // Match constraint for errors
        }
    }

    fn constrain_int(&mut self, value: &Value) {
        // println!("{value:?} <- some Int");
        if let Some(infer_key) = self.infer_key(&value.typ) {
            let constraint = self.checker
                                 .impose(infer_key.concretizes_explicit(TypeVariant::SomeInteger));

            if constraint.is_ok() {
                return;
            }

            self.debugger
                .throw_single(ErrorCode::TypeIsNotAnInteger, &value.span);

            match constraint.err().unwrap() {
                TcErr::KeyEquation(_key1, _key2, _error) => {
                    println!("Incompatible types");
                }

                _ => {}
            }

            // Match constraint for errors
        }
    }

    fn constrain_float(&mut self, value: &Value) {
        // println!("{value:?} <- some Float");
        if let Some(infer_key) = self.infer_key(&value.typ) {
            let _constraint = self.checker
                                  .impose(infer_key.concretizes_explicit(TypeVariant::SomeFloat));

            // Match constraint for errors
        }
    }

    fn constrain_string(&mut self, value: &Value, len: usize) {
        //println!("{value:?} <- some String");

        if let Some(infer_key) = self.infer_key(&value.typ) {
            let variant = if len == 1 { TypeVariant::SomeChar } else { TypeVariant::SomeString };
            
            let _constraint = self.checker
                                  .impose(infer_key.concretizes_explicit(variant));

            // Match constraint for errors
        }
    }

    fn constrain_func(&mut self, value: &Value) {
        // println!("{value:?} <- some Function");
        if let Some(infer_key) = self.infer_key(&value.typ) {
            let _constraint = self.checker
                                  .impose(infer_key.concretizes_explicit(TypeVariant::SomeFunction));

            // Match constraint for errors
        }
    }

    fn constrain_one_way(&mut self, constrain: &Type, absolute: &Type) {
        // println!("{constrain:?} <- {absolute:?}");

        if let (TypeKind::Function { return_type: return_type_1,
                                     params: params_1,
                                    .. },
                TypeKind::Function { return_type: return_type_2,
                                     params: params_2,
                                     .. }) = (constrain.kind(), absolute.kind())
        {
            self.constrain_one_way(return_type_1, return_type_2);

            for (param1, param2) in params_1.iter().zip(params_2) {
                self.constrain_one_way(param1, param2);
            }
        }  else if let (TypeKind::Tuple(tuple_items_1, ..), 
                        TypeKind::Tuple(tuple_items_2, ..)) = (constrain.kind(), absolute.kind())
        {
            for (tuple_item_1, tuple_item_2) in tuple_items_1.iter().zip(tuple_items_2) {
                self.constrain_one_way(tuple_item_1, tuple_item_2);
            }
        }

        let Some(constrain_key) = self.infer_key(constrain) else {
			return
		};

        let _constraint = if let Some(absolute_key) = self.infer_key(absolute) {
            self.checker.impose(constrain_key.concretizes(absolute_key))
        } else {
            let bound = self.variant(absolute);

            self.checker
                .impose(constrain_key.concretizes_explicit(bound))
        };

        // Match against error
    }

    fn constrain_two_way(&mut self, ty1: &Type, ty2: &Type) {
        // println!("{ty1:?} <-> {ty2:?}");
        let _constraint = match (self.infer_key(ty1), self.infer_key(ty2)) {
            (Some(key1), Some(key2)) if key1 != key2 => self.checker.impose(key1.equate_with(key2)),
            (Some(key1), None) => {
                let variant = self.variant(ty2);
                self.checker.impose(key1.concretizes_explicit(variant))
            }
            (None, Some(key2)) => {
                let variant = self.variant(ty1);
                self.checker.impose(key2.concretizes_explicit(variant))
            }
            (None, None) => {
                if let (TypeKind::Function { return_type: return_type_1,
                                             params: params_1,
                                             .. },
                        TypeKind::Function { return_type: return_type_2,
                                             params: params_2,
                                             .. }) = (ty1.kind(), ty2.kind())
                {
                    self.constrain_two_way(return_type_1, return_type_2);

                    for (param1, param2) in params_1.iter().zip(params_2) {
                        self.constrain_two_way(param1, param2);
                    }
                } else if let (TypeKind::Tuple(tuple_items_1, ..), 
                               TypeKind::Tuple(tuple_items_2, ..)) = (ty1.kind(), ty2.kind())
                {
                    for (tuple_item_1, tuple_item_2) in tuple_items_1.iter().zip(tuple_items_2) {
                        self.constrain_two_way(tuple_item_1, tuple_item_2);
                    }
                }

                return;
            }
            _ => return,
        };

        /*match constraint.err() {
            Some(TcErr::CyclicGraph) => {}

            _ => {}
        }*/
    }

    fn variant(&self, ty: &Type) -> TypeVariant {
        match ty.kind() {
            TypeKind::Divergent => TypeVariant::Diverges,
            TypeKind::Void => TypeVariant::Void,

            TypeKind::Integer { bits: 1 } => TypeVariant::LlvmBool,
            TypeKind::Integer { bits } => TypeVariant::LlvmInt { bits: *bits as u32 },
            TypeKind::Float { bits } => TypeVariant::LlvmFloat { bits: *bits as u32 },
            TypeKind::StrSlice => TypeVariant::LlvmString,

            TypeKind::Struct(r#struct) => TypeVariant::Struct(r#struct.clone()),
            TypeKind::Enum(r#enum) => TypeVariant::Enum(r#enum.clone()),

            TypeKind::Function { return_type,
                                 labels,
                                 params, } => TypeVariant::Function { params:      params.clone(),
                                                                      labels:      labels.clone(),
                                                                      return_type: return_type.clone(), },

            TypeKind::Tuple(tuple_items, labels) => TypeVariant::Tuple(tuple_items.clone(), labels.clone()),

            TypeKind::Error => TypeVariant::Error,

            _ => panic!("{ty:?}"),
        }
    }

    fn infer_key(&mut self, ty: &Type) -> Option<TcKey> {
        let TypeKind::Infer { key } = ty.kind() else {
			return None;
		};

        if let Some(tc_key) = self.infer_keys.get(key) {
            Some(*tc_key)
        } else {
            let new_tc_key = self.checker.new_term_key();

            self.infer_keys.insert(*key, new_tc_key);

            Some(new_tc_key)
        }
    }
}
