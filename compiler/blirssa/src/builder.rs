use crate::{code::{BlockRef, ExternFunctionRef, FunctionRef},
            typ::Type,
            value::{BinaryIntrinsicFn, Instruction, IntrinsicFnOutput, LabelValue, UnaryIntrinsicFn, Value}};

pub struct Builder {
    current_func:       Option<FunctionRef>,
    current_block:      Option<BlockRef>,
    current_block_head: usize,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { current_func:       None,
                  current_block:      None,
                  current_block_head: 0, }
    }

    pub fn position_at_end(&mut self, block: &BlockRef) {
        self.current_func = block.function().upgrade();
        self.current_block = Some(block.clone());
        self.current_block_head = block.instructions().len();
    }

    pub fn build_integer_literal(&mut self, bits: u32, value: u64) -> LabelValue {
        let typ = Type::Integer { bits };

        let value = Value::IntegerLiteral { value, typ };

        return self.build_av(value);
    }

    pub fn build_float_literal(&mut self, bits: u32, value: f64) -> LabelValue {
        let typ = Type::Float { bits };

        let value = Value::FloatLiteral { value, typ };

        return self.build_av(value);
    }

    pub fn build_string_literal(&mut self, value: String) -> LabelValue {
        let typ = Type::StrSlice;

        let value = Value::GlobalString { value, typ };

        return self.build_av(value);
    }

    pub fn build_unary_intrinsic(&mut self, intrinsic: UnaryIntrinsicFn, arg: LabelValue) -> LabelValue {
        let output = intrinsic.output_type();

        let output_typ = match output {
            IntrinsicFnOutput::Boolean => Type::Integer { bits: 1 },
            IntrinsicFnOutput::Same => arg.typ(),
            IntrinsicFnOutput::Integer(bits) => Type::Integer { bits },
            IntrinsicFnOutput::Float(bits) => Type::Float { bits },
        };

        let unary_op = Value::UnaryIntrinsic { name: intrinsic,
                                               arg,
                                               return_type: output_typ };

        return self.build_av(unary_op);
    }

    pub fn build_binary_intrinsic(&mut self, intrinsic: BinaryIntrinsicFn, left: LabelValue, right: LabelValue) -> LabelValue {
        // Check for type equality

        let output = intrinsic.output_type();

        let output_typ = match output {
            IntrinsicFnOutput::Boolean => Type::Integer { bits: 1 },
            IntrinsicFnOutput::Same => left.typ(),
            IntrinsicFnOutput::Integer(bits) => Type::Integer { bits },
            IntrinsicFnOutput::Float(bits) => Type::Float { bits },
        };

        let binary_op = Value::BinaryIntrinsic { name: intrinsic,
                                                 left,
                                                 right,
                                                 return_type: output_typ };

        return self.build_av(binary_op);
    }

    pub fn build_return(&mut self, value: Option<LabelValue>) {
        let instruction = match value {
            Some(value) => {
                if value.typ_ref() == &Type::Void {
                    Instruction::Return { value: None }
                } else if let Type::Pointer { .. } = value.typ_ref() {
                    let pointee_deref = self.build_deref(value);

                    return self.build_return(Some(pointee_deref));
                } else {
                    Instruction::Return { value: Some(value) }
                }
            }
            None => Instruction::Return { value: None },
        };

        self.build_i(instruction);
    }

    pub fn build_assign_ptr(&mut self, pointer: LabelValue, value: LabelValue) {
        let Type::Pointer { pointee } = pointer.typ_ref() else {
			panic!();
		};

        if pointee.as_ref() != value.typ_ref() {
            panic!("{pointee} <=> {value}");
        }

        let instruction = Instruction::AssignPtr { pointer, value };

        self.build_i(instruction);
    }

    pub fn build_deref(&mut self, pointer: LabelValue) -> LabelValue {
        let Type::Pointer { pointee } = pointer.typ_ref() else {
			panic!();
		};

        let typ = pointee.as_ref().clone();

        let value = Value::Deref { pointer, typ };

        self.build_av(value)
    }

    pub fn build_deref_struct_field(&mut self, r#struct: LabelValue, field: &str) -> LabelValue {
        match r#struct.typ() {
            Type::Pointer { pointee } => {
                let Type::Struct { container } = pointee.as_ref() else {
					panic!();
				};

                let typ = container.get_field_type(field);

                let value = Value::DerefStructField { r#struct,
                                                      field: field.to_string(),
                                                      typ };

                self.build_av(value)
            }
            Type::Struct { container } => {
                let typ = container.get_field_type(field);

                let value = Value::DerefStructField { r#struct,
                                                      field: field.to_string(),
                                                      typ };

                self.build_av(value)
            }
            _ => panic!(),
        }
    }

    pub fn build_access_struct_field(&mut self, r#struct: LabelValue, field: &str) -> LabelValue {
        let Type::Pointer { pointee } = r#struct.typ_ref() else {
			panic!();
		};

        let Type::Struct { container } = pointee.as_ref() else {
			panic!();
		};

        let typ = container.get_field_type(field).pointer();

        let value = Value::AccessStructField { r#struct,
                                               field: field.to_string(),
                                               typ };

        self.build_av(value)
    }

    pub fn build_access_tuple_field(&mut self, tuple: LabelValue, field: usize) -> LabelValue {
        let Type::Pointer { pointee } = tuple.typ_ref() else {
			panic!();
		};

        let Type::Tuple(items) = pointee.as_ref() else {
			panic!();
		};

        let typ = items[field].clone().pointer();

        let value = Value::AccessTupleField { r#tuple,
                                              field,
                                              typ };

        self.build_av(value)
    }

    pub fn build_deref_tuple_field(&mut self, tuple: LabelValue, field: usize) -> LabelValue {
        let Type::Tuple(items) = tuple.typ_ref() else {
			panic!();
		};

        let typ = items[field].clone().pointer();

        let value = Value::DerefTupleField { r#tuple,
                                             field,
                                             typ };

        self.build_av(value)
    }

    pub fn build_stack_alloc_undef(&mut self, typ: Type) -> LabelValue {
        let value = Value::AllocOnStackUndef { typ: Type::Pointer { pointee: Box::new(typ) }, };

        self.build_av(value)
    }

    pub fn build_stack_alloc(&mut self, value: LabelValue) -> LabelValue {
        let typ = Type::Pointer { pointee: Box::new(value.typ()), };

        let value = Value::AllocOnStack { value, typ };

        self.build_av(value)
    }

    pub fn build_function(&mut self, function: &FunctionRef) -> LabelValue {
        let function_value = Value::Function { function: function.clone(), };

        return self.build_av(function_value);
    }

    pub fn build_extern_function(&mut self, function: &ExternFunctionRef) -> LabelValue {
        let function_value = Value::ExternFunction { function: function.clone(), };

        return self.build_av(function_value);
    }

    pub fn build_function_pointer(&mut self, function: LabelValue) -> LabelValue {
        let func_type = function.typ();

        let Type::Function { .. } = &func_type else {
			panic!()
		};

        let function_ptr_value = Value::BuildFunctionPointer { function, func_type };

        self.build_av(function_ptr_value)
    }

    pub fn build_call(&mut self, function: LabelValue, args: Vec<LabelValue>) -> LabelValue {
        let function_type = match function.typ_ref() {
            Type::Function { return_type, .. } => return_type.as_ref().clone(),
            t => panic!("Can't call value of type {t}"),
        };

        // Check arg types

        let call_value = Value::Call { function,
                                       args,
                                       typ: function_type };

        return self.build_av(call_value);
    }

    pub fn build_create_enum_variant(&mut self, enum_type: Type, variant: &str) -> LabelValue {
        let value = Value::CreateEnumVariant { variant: variant.to_string(), typ: enum_type };

        self.build_av(value)
    }

    pub fn build_branch(&mut self, condition: LabelValue, positive: &BlockRef, negative: &BlockRef) {
        let instruction = Instruction::Branch { condition,
                                                positive: positive.clone(),
                                                negative: negative.clone() };

        self.build_i(instruction);
    }

    pub fn build_always_branch(&mut self, block: &BlockRef) {
        let instruction = Instruction::AlwaysBranch { block: block.clone() };

        self.build_i(instruction);
    }

    pub fn build_select_integer(&mut self, discriminant: LabelValue, branches: Vec<(LabelValue, BlockRef)>, default: BlockRef) {
        let instruction = Instruction::SelectInteger {
            value: discriminant,
            branches,
            default };

        self.build_i(instruction);
    }

    pub fn build_select_enum(&mut self, discriminant: LabelValue, branches: Vec<(String, BlockRef)>, default: BlockRef) {
        let instruction = Instruction::SelectEnumTag {
            value: discriminant,
            branches,
            default };

        self.build_i(instruction);
    }

    fn build_av(&mut self, value: Value) -> LabelValue {
        let label = self.function().next_index();

        let typ = value.typ();

        let label = LabelValue { label, typ };

        let instruction = Instruction::Assign { label: label.clone(),
                                                value };

        self.block()
            .insert_instruction(self.current_block_head, instruction);

        self.current_block_head += 1;

        label
    }

    fn build_i(&mut self, instruction: Instruction) {
        self.block()
            .insert_instruction(self.current_block_head, instruction);

        self.current_block_head += 1;
    }

    fn block(&self) -> &BlockRef {
        self.current_block
            .as_ref()
            .expect("Tried to use builder before positioning it")
    }

    fn function(&self) -> &FunctionRef {
        self.current_func
            .as_ref()
            .expect("Tried to use builder before positioning it")
    }
}
