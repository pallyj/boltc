use std::{fmt::Debug};

use blir::{
    pattern::{PatternKind, Pattern},
    value::{match_::MatchValue, MatchBranch, Value, ValueKind},
    typ::{TypeKind, Type, StructRef},
    Symbol,
    SomeFunction, code::CodeBlock};
use blirssa::{value::LabelValue, code::{BlockRef}};
use errors::{error::ErrorCode, Span};
use patmat::{PatternMatrix, Maranget, DecisionTree};

use crate::BlirLowerer;

impl<'a, 'b> BlirLowerer<'a, 'b> {
	pub (crate) fn lower_match(&mut self, value: &MatchValue, ty: &Type) -> LabelValue {
        let assign_val_ptr = match ty.kind() {
            TypeKind::Void | TypeKind::Divergent => None,
            _ => {
                let ty = self.lower_type(ty);
                Some(self.builder().build_stack_alloc_undef(ty))
            }
        };

		let before_block = self.context.function().append_block("beforeMatch");

        let scrutinee = (*value.discriminant).clone();
        let patterns = value.branches.iter()
                                     .map(|branch| branch.pattern.clone() )
                                     .collect();
        let code_blocks = value.branches.iter()
                                        .map(|branch| branch.code.clone())
                                        .collect::<Vec<_>>();

        let pattern_matrix = PatternMatrix::construct(scrutinee, patterns)
            .expand();

        let decision_tree = pattern_matrix.solve::<Maranget>();

        self.lower_decision_tree(decision_tree, &code_blocks, assign_val_ptr.as_ref(), &before_block);

		let after_block = self.context.function().append_block("afterMatch");
		self.builder().position_at_end(&before_block);
		self.builder().build_always_branch(&after_block);
        self.builder().position_at_end(&after_block);

        assign_val_ptr.map(|assign_val_ptr| self.builder().build_deref(assign_val_ptr))
            .unwrap_or_else(|| LabelValue::void())
    }

    fn lower_decision_tree(
        &mut self,
        tree: DecisionTree,
        leaves: &[CodeBlock],
        pointer: Option<&LabelValue>,
        sink: &BlockRef)
    {
        match tree {
            DecisionTree::Fail => panic!(),
            DecisionTree::Leaf(end, bindings) => {
                for (bind_name, bind_value) in bindings {
                    let bound_value = self.lower_value(&bind_value);
                    self.context.define_var(&bind_name, bound_value);
                }

                let leaf = &leaves[end.index() as usize];

                let code_value = self.lower_code_block(leaf);

                if let Some((pointer, code_value)) = pointer.zip(code_value) {
                    self.builder().build_assign_ptr(pointer.clone(), code_value);
                }

                self.builder().build_always_branch(sink);
            },
            DecisionTree::Switch { scrutinee,
                                   patterns,
                                   default } =>
            {
                let lowered_scrutinee = self.lower_value(&scrutinee);

                let default_block = self.context.function().append_block("default");

                let switch_branches = match scrutinee.typ.kind() {
                    TypeKind::Integer { .. } => self.switch_integer(&lowered_scrutinee, &patterns, &default_block),
                    TypeKind::Struct(struct_ref) if struct_ref.integer_repr() => self.switch_integer(&lowered_scrutinee, &patterns, &default_block),
                    TypeKind::Enum(_) => self.switch_enum(&lowered_scrutinee, &patterns, &default_block),
                    TypeKind::Struct(struct_ref) => self.switch_struct(&lowered_scrutinee, &patterns, &default_block, struct_ref, &scrutinee.span),
                    _ => todo!(),
                };

                self.builder().position_at_end(&default_block);

                if let Some(default) = default {
                    self.lower_decision_tree(*default, leaves, pointer, sink);
                } else {
                    // Check for exhaustiveness
                    let is_exhaustive = true;

                    if is_exhaustive {
                        // Emit a panic
                    } else {
                        // Throw an error
                    }
                    self.builder().build_always_branch(sink);
                }

                for (block, (_, tree)) in switch_branches.iter().zip(patterns) {
                    self.builder().position_at_end(&block);
                    self.lower_decision_tree(tree, leaves, pointer, sink);
                }
            }
        }
    }

    fn switch_integer(
        &mut self,
        lowered_scrutinee: &LabelValue,
        patterns: &Vec<(Pattern, DecisionTree)>,
        default_block: &BlockRef) -> Vec<BlockRef>
    {
        let switch_branches = patterns.iter()
                                      .map(|(pat, _)| {
                                          let block = self.context.function().append_block("case");
                                          let value = match &pat.kind {
                                              PatternKind::Literal { value } => self.lower_value(value),
                                              _ => unreachable!(),
                                          };
                                      
                                          (value, block)
                                      })
                                      .collect::<Vec<_>>();

        self.builder().build_select_integer(lowered_scrutinee.clone(), switch_branches.clone(), default_block.clone());

        switch_branches.into_iter()
                       .map(|branch| branch.1)
                       .collect()
    }

    fn switch_enum(
        &mut self,
        lowered_scrutinee: &LabelValue,
        patterns: &Vec<(Pattern, DecisionTree)>,
        default_block: &BlockRef) -> Vec<BlockRef>
    {
        let switch_branches = patterns.iter()
                                      .map(|(pat, _)| {
                                          let block = self.context.function().append_block("case");
                                          let value = match &pat.kind {
                                              PatternKind::Literal { value } => match &value.kind {
                                                  ValueKind::EnumVariant { variant, .. } => variant.name().to_string(),
                                                  _ => unreachable!(),
                                              }
                                              _ => unreachable!(),
                                          };
                                      
                                          (value, block)
                                      })
                                      .collect::<Vec<_>>();

        self.builder().build_select_enum(lowered_scrutinee.clone(), switch_branches.clone(), default_block.clone());

        switch_branches.into_iter()
                       .map(|branch| branch.1)
                       .collect()
    }

    fn switch_struct(
        &mut self,
        lowered_scrutinee: &LabelValue,
        patterns: &Vec<(Pattern, DecisionTree)>,
        default_block: &BlockRef,
        struct_ref: &StructRef,
        scrutinee_span: &Option<Span>) -> Vec<BlockRef>
    {
        let Some(Symbol::Function(mut equal_function)) = struct_ref.lookup_static_item("op~equal") else {
            self.debugger.throw_single(ErrorCode::OperatorNotDefined("equals".to_string(), struct_ref.name()), scrutinee_span);
            return vec![];
        };

        equal_function.filter_types(&vec![ TypeKind::Struct(struct_ref.clone()).anon(), TypeKind::Struct(struct_ref.clone()).anon() ]);

        let Some(equal_function) = equal_function.resolve() else {
            self.debugger.throw_single(ErrorCode::OperatorNotDefined("equals".to_string(), struct_ref.name()), scrutinee_span);
            return vec![];
        };

        let lowered_function = match equal_function {
            SomeFunction::StaticMethod(function) => self.ssa_library().get_function(function.info().link_name()).cloned().unwrap(),
            _ => unreachable!(),
        };

        let lowered_function = self.builder().build_function(&lowered_function);

        let case_branches = patterns.iter()
                .map(|(pat, _)| {
                    let positive_branch = self.context.function().append_block("case");
                    let negative_branch = self.context.function().append_block("negative");

                    let equal_to = match &pat.kind {
                        PatternKind::Literal { value } => self.lower_value(value),
                        _ => unreachable!(),
                    };

                    let is_equal = self.builder().build_call(lowered_function.clone(), vec![ lowered_scrutinee.clone(), equal_to ]);

                    self.builder()
                        .build_branch(is_equal, &positive_branch, &negative_branch);

                    self.builder().position_at_end(&negative_branch);

                    positive_branch
                })
                .collect();

        self.builder().build_always_branch(default_block);

        case_branches
    }
}

/*
fn lower_equality_layer(
		&mut self,
		value: LabelValue,
		candidate: Candidate,
		before_branch: &BlockRef,
        last_default_branch: Option<&BlockRef>,
        struct_ref: &StructRef,
		output_pointer: Option<&LabelValue>)
	{

        for (pattern, next_branch) in candidate.next_candidates {
            let positive_branch = self.context.function().append_block("positive");
            let negative_branch = self.context.function().append_block("negative");

            // Get the equality constraint
            let equal_to = match &pattern.kind {
                PatternKind::Literal { value } => self.lower_value(&value),
                _ => panic!(),
            };
            
            let is_equal = self.builder().build_call(lowered_function.clone(), vec![ value.clone(), equal_to ]);

            self.builder().build_branch(is_equal, &positive_branch, &negative_branch);
            self.builder().position_at_end(&positive_branch);

            self.lower_some_candidate(next_branch, before_branch, last_default_branch, output_pointer);

            self.builder().build_always_branch(&before_branch);

            self.builder().position_at_end(&negative_branch);
        }

        if let Some(default_branch) = candidate.default_branch {
            if let Some((yielded_value, assign_val_ptr)) = self.lower_code_block(&default_branch).zip(output_pointer) {
                self.builder.build_assign_ptr(assign_val_ptr.clone(), yielded_value);
            };

            self.builder().build_always_branch(&before_branch);
        }
	} */
    