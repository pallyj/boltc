use blir::{
    pattern::{PatternKind},
    value::{match_::MatchValue, MatchBranch, Value, ValueKind},
    typ::{TypeKind, Type, StructRef},
    Symbol,
    SomeFunction};
use blirssa::{value::LabelValue, code::BlockRef};

use crate::BlirLowerer;

impl BlirLowerer {
	pub (crate) fn lower_match(&mut self, value: &MatchValue, ty: &Type) -> LabelValue {
        let assign_val_ptr = match ty.kind() {
            TypeKind::Void | TypeKind::Divergent => None,
            _ => {
                let ty = self.lower_type(ty);
                Some(self.builder().build_stack_alloc_undef(ty))
            }
        };

		let before_block = self.context.function().append_block("beforeMatch");

		self.lower_some_layer(&value.discriminant, &value.branches, &before_block, assign_val_ptr.as_ref());

		let after_block = self.context.function().append_block("afterMatch");
		self.builder().position_at_end(&before_block);

		self.builder().build_always_branch(&after_block);

        self.builder().position_at_end(&after_block);

        assign_val_ptr.map(|assign_val_ptr| self.builder().build_deref(assign_val_ptr))
            .unwrap_or_else(|| LabelValue::void())
    }

	fn lower_some_layer(
		&mut self,
		value: &Value,
		branches: &[MatchBranch],
		before_branch: &BlockRef,
		output_pointer: Option<&LabelValue>
	) {
		match value.typ.kind() {
			TypeKind::Integer { .. } => {
				let lowered_value = self.lower_value(value);
				self.lower_integer_layer(lowered_value, branches, before_branch, output_pointer)
			}
			TypeKind::Struct(struct_ref) if struct_ref.integer_repr() => {
				// TODO: Get the value
				let lowered_value = self.lower_value(value);
				self.lower_integer_layer(lowered_value, branches, before_branch, output_pointer)
			}
            TypeKind::Enum(_) => {
                let lowered_value = self.lower_value(value);
				self.lower_enum_layer(lowered_value, branches, before_branch, output_pointer)
            }
            TypeKind::Struct(struct_ref) => {
                let lowered_value = self.lower_value(value);
                self.lower_equality_layer(lowered_value, branches, before_branch, struct_ref, output_pointer)
            }
			_ => panic!()
		}
	}

	fn lower_integer_layer(
		&mut self,
		value: LabelValue,
		branches: &[MatchBranch],
		before_branch: &BlockRef,
		output_pointer: Option<&LabelValue>)
	{
        let basic_blocks = (0..branches.len())
            .map(|_| self.context.function().append_block("matchArm"))
            .collect::<Vec<_>>();

        let branch_values = basic_blocks
            .iter()
            .zip(branches)
            .filter_map(|(block, branch)| {
                let value = match &branch.pattern.kind {
                    PatternKind::Literal { value } => self.lower_value(value),
                    _ => return None
                };

                Some((value, block.clone()))
            }).collect();

        let default_branch = basic_blocks
            .iter()
            .zip(branches)
            .find_map(|(block, branch)| {
                match &branch.pattern.kind {
                    PatternKind::Wildcard => Some(block.clone()),
                    _ => None
                }
            })
            .unwrap();

        self
            .builder()
            .build_select_integer(value, branch_values, default_branch);

        for (basic_block, branch) in basic_blocks.iter().zip(branches) {
            self.builder().position_at_end(&basic_block);

            if let Some((yielded_value, assign_val_ptr)) = self.lower_code_block(&branch.code).zip(output_pointer) {
                self.builder.build_assign_ptr(assign_val_ptr.clone(), yielded_value);
            };

            self.builder().build_always_branch(before_branch);
        }
	}

    fn lower_enum_layer(
		&mut self,
		value: LabelValue,
		branches: &[MatchBranch],
		before_branch: &BlockRef,
		output_pointer: Option<&LabelValue>)
	{
        let basic_blocks = (0..branches.len())
            .map(|_| self.context.function().append_block("matchArm"))
            .collect::<Vec<_>>();

        let branch_values = basic_blocks
            .iter()
            .zip(branches)
            .filter_map(|(block, branch)| {
                let value = match &branch.pattern.kind {
                    PatternKind::Literal { value } => match &value.kind {
                        ValueKind::EnumVariant { variant, .. } => variant.name().to_string(),
                        _ => return None
                    }
                    _ => return None
                };

                Some((value, block.clone()))
            }).collect();

        let default_branch = basic_blocks
            .iter()
            .zip(branches)
            .find_map(|(block, branch)| {
                match &branch.pattern.kind {
                    PatternKind::Wildcard => Some(block.clone()),
                    _ => None
                }
            })
            .unwrap();

        self
            .builder()
            .build_select_enum(value, branch_values, default_branch);

        for (basic_block, branch) in basic_blocks.iter().zip(branches) {
            self.builder().position_at_end(&basic_block);

            if let Some((yielded_value, assign_val_ptr)) = self.lower_code_block(&branch.code).zip(output_pointer) {
                self.builder.build_assign_ptr(assign_val_ptr.clone(), yielded_value);
            };

            self.builder().build_always_branch(before_branch);
        }
	}

    fn lower_equality_layer(
		&mut self,
		value: LabelValue,
		branches: &[MatchBranch],
		before_branch: &BlockRef,
        struct_ref: &StructRef,
		output_pointer: Option<&LabelValue>)
	{
        let Some(Symbol::Function(mut equal_function)) = struct_ref.lookup_static_item("op~equal") else {
            println!("operator `==` is not defined for type `{}`", struct_ref.name());
            return;
        };

        equal_function.filter_types(&vec![ TypeKind::Struct(struct_ref.clone()).anon(), TypeKind::Struct(struct_ref.clone()).anon() ]);

        let Some(equal_function) = equal_function.resolve() else {
            println!("operator `==` is not defined for type `{}`", struct_ref.name());
            return;
        };

        let lowered_function = match equal_function {
            SomeFunction::StaticMethod(function) => self.ssa_library().get_function(function.info().link_name()).cloned().unwrap(),
            _ => panic!(),
        };

        let lowered_function = self.builder().build_function(&lowered_function);

        let mut default_branch = None;

        for branch in branches {
            if let PatternKind::Wildcard = &branch.pattern.kind {
                default_branch = Some(branch);
                continue
            }

            let positive_branch = self.context.function().append_block("positive");
            let negative_branch = self.context.function().append_block("negative");

            // Get the equality constraint
            let equal_to = match &branch.pattern.kind {
                PatternKind::Literal { value } => self.lower_value(&value),
                _ => panic!(),
            };
            
            let is_equal = self.builder().build_call(lowered_function.clone(), vec![ value.clone(), equal_to ]);

            self.builder().build_branch(is_equal, &positive_branch, &negative_branch);
            self.builder().position_at_end(&positive_branch);

            if let Some((yielded_value, assign_val_ptr)) = self.lower_code_block(&branch.code).zip(output_pointer) {
                self.builder.build_assign_ptr(assign_val_ptr.clone(), yielded_value);
            };

            self.builder().build_always_branch(&before_branch);

            self.builder().position_at_end(&negative_branch);
        }

        if let Some(default_branch) = default_branch {
            if let Some((yielded_value, assign_val_ptr)) = self.lower_code_block(&default_branch.code).zip(output_pointer) {
                self.builder.build_assign_ptr(assign_val_ptr.clone(), yielded_value);
            };

            self.builder().build_always_branch(&before_branch);
        }
	}

}