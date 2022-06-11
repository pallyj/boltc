use std::{sync::atomic::{AtomicUsize, Ordering}, collections::HashSet};

use blir::{value::{IfValue, IfBranch, match_::MatchValue, ValueKind, Value}, typ::{Type, TypeKind, StructRef}, code::CodeBlock, pattern::{Pattern, PatternKind}, SomeFunction, Symbol};
use errors::Span;
use mir::{val::{Place, RValue}, instr::{Terminator, SwitchArm}, code::BasicBlockId};
use patmat::{DecisionTree, Maranget, PatternMatrix};

use crate::BlirLowerer;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl<'a> BlirLowerer<'a> {
	pub fn lower_if_value(
		&mut self,
		if_value: &IfValue,
		place: Option<&Place>)
	{
		let before = self.builder.append_block();

		self.lower_if_value_inner(if_value, place, before);

		let finally = self.builder.append_block();

		self.builder.position_at_end(before);
		self.builder.build_terminator(Terminator::goto(finally));

		self.builder.position_at_end(finally);
	}

	fn lower_if_value_inner(
		&mut self,
		value: &IfValue,
		place: Option<&Place>,
		finally: BasicBlockId)
	{
		use blir::typ::TypeKind::*;
		let condition = match value.condition.typ.kind() {
			Integer { bits: _ } => self.lower_rvalue(&value.condition),
			Struct(struct_ref) if struct_ref.bool_repr() => {
				let field_name = struct_ref.borrow().instance_vars[0].borrow().name.clone();

				self.lower_place(&value.condition).field(&field_name, mir::ty::Type::int(1), Span::empty()).copy(Span::empty())
			}
			_ => unreachable!()
		};

		let positive_block = self.builder.append_block();

		if let Some(negative) = &value.negative {
			// Create a branch between the positive and negative blocks
			let negative_block = self.builder.append_block();
			self.builder.build_terminator(Terminator::branch_if(condition, positive_block, negative_block));

			// Lower the code for the negative block
			self.builder.position_at_end(negative_block);
			use IfBranch::*;
				
			match negative {
				// Lowers an else branch and branches to the end
				CodeBlock(block) => {
					let rvalue = self.lower_code_block(block);

					if let Some((place, value)) = place.zip(rvalue) {
						self.builder.build_assign(place, value);
					}

					if !block.escapes() {
						self.builder.build_terminator(Terminator::goto(finally));
					}
				}

				// Lowers an elseif branch
				Else(else_branch) => self.lower_if_value_inner(&else_branch, place, finally),
			}
		} else {
			self.builder.build_terminator(Terminator::branch_if(condition, positive_block, finally));
		}

		self.builder.position_at_end(positive_block);

		let rvalue = self.lower_code_block(&value.positive);

		if let Some((place, value)) = place.zip(rvalue) {
			self.builder.build_assign(place, value);
		}

		if !value.positive.escapes() {
			self.builder.build_terminator(Terminator::goto(finally));
		}
	}

	pub fn has_else_covered(&self, negative: &Option<IfBranch>) -> bool {
        match negative {
            Some(IfBranch::CodeBlock(_)) => true,
            Some(IfBranch::Else(else_branch)) => self.has_else_covered(&else_branch.negative),
            None => false,
        }
    }

	pub (crate) fn lower_match(
		&mut self,
		value: &MatchValue,
		ty: &Type,
		output_place: Option<&Place>)
	{
		let before_block = self.builder.append_block();

		let scrut = self.lower_place(&*value.discriminant);

        let idx = COUNTER.fetch_add(1, Ordering::Relaxed);
        let match_name = format!("match{}", idx);
        self.function_ctx.insert(match_name.clone(), scrut);

        let scrutinee = ValueKind::LocalVariable(match_name)
            .spanned(value.discriminant.typ.clone(), value.discriminant.span.unwrap());

        let patterns = value.branches.iter()
                                     .map(|branch| branch.pattern.clone() )
                                     .collect();
        let code_blocks = value.branches.iter()
                                        .map(|branch| branch.code.clone())
                                        .collect::<Vec<_>>();

        let pattern_matrix = PatternMatrix::construct(scrutinee, patterns)
            .expand();

        let decision_tree = pattern_matrix.solve::<Maranget>();

        self.lower_decision_tree(decision_tree, &code_blocks, output_place, before_block, value.discriminant.span);

		let after_block = self.builder.append_block();
		self.builder.position_at_end(before_block);
		self.builder.build_terminator(Terminator::goto(after_block));
        self.builder.position_at_end(after_block);
    }

    fn lower_decision_tree(
        &mut self,
        tree: DecisionTree,
        leaves: &[CodeBlock],
        pointer: Option<&Place>,
        sink: BasicBlockId,
        span: Option<Span>)
    {

        match tree {
            DecisionTree::Fail => panic!(),
            DecisionTree::Leaf(end, bindings) => {
                for (bind_name, bind_value) in bindings {
                    let bound_value = self.lower_place(&bind_value);
					self.function_ctx.insert(bind_name.clone(), bound_value);
                }

                let leaf = &leaves[end.index() as usize];

                let code_value = self.lower_code_block(leaf);

                if let Some((pointer, code_value)) = pointer.zip(code_value) {
					self.builder.build_assign(pointer, code_value);
                }

                if !leaf.escapes() {
                    self.builder.build_terminator(Terminator::goto(sink));
                }
            },
            DecisionTree::Switch { scrutinee,
                                   patterns,
                                   default } =>
            {
                let default_block = self.builder.append_block();

                let switch_branches = match scrutinee.typ.kind() {
                    TypeKind::Integer { .. } => {
						let rvalue = self.lower_rvalue(&scrutinee);
						self.switch_integer(rvalue, &patterns, default_block)
					}
                    TypeKind::Struct(struct_ref) if struct_ref.integer_repr() => {
						// todo: Get the actual name
                        let place = self.lower_place(&scrutinee);
						let place = self.builder.build_field(&place, "repr", Span::empty());
						self.switch_integer(place.copy(Span::empty()), &patterns, default_block)
					}
                    TypeKind::Enum(_) => {
						let place = self.lower_place(&scrutinee);
						self.switch_enum(&place, &patterns, default_block)
					}
                    TypeKind::Struct(struct_ref) => {
						let place = self.lower_place(&scrutinee);
						self.switch_struct(&place, &patterns, default_block, struct_ref, &scrutinee.span)
					}
                    _ => todo!(),
                };

                self.builder.position_at_end(default_block);

                if let Some(default) = default {
                    self.lower_decision_tree(*default, leaves, pointer, sink, span);
                } else {
                    // Check for exhaustiveness
                    self.exhaustiveness_check(&scrutinee.typ, &patterns, &span);

                    // todo: Build unreachable
                    self.builder.build_terminator(Terminator::goto(sink));
                }

                for (block, (_, tree)) in switch_branches.iter().zip(patterns) {
                    self.builder.position_at_end(*block);
                    self.lower_decision_tree(tree, leaves, pointer, sink, span);
                }
            }
        }
    }

	///
	/// Check if the match is exhaustive
	/// 
    fn exhaustiveness_check(
        &mut self,
        typ: &Type,
        patterns: &Vec<(Pattern, DecisionTree)>,
        span: &Option<Span>)
    {
        match typ.kind() {
            TypeKind::Enum(enum_ref) => {
                if enum_ref.variants().len() == patterns.len() {
                    return
                }
        
                let mut variants = HashSet::new();
        
                for var in enum_ref.variants().iter() {
                    variants.insert(var.name().clone());
                }
        
                for pat in patterns.iter() {
                    match &pat.0.kind {
                        PatternKind::Variant { variant: Value { kind: ValueKind::EnumVariant { variant, .. }, .. }, .. } |
                        PatternKind::Literal { value: Value { kind: ValueKind::EnumVariant { variant, .. }, .. }, .. } => {
                            variants.remove(variant.name());
                        }
                        _ => {}
                    }
                }
        
                //self.debugger
                //    .throw_single(ErrorCode::NonExhaustiveMatch(variants.iter().map(|var| format!(".{var}")).collect()), span);
            }

            TypeKind::Struct(struct_ref) if struct_ref.bool_repr() => {
                if patterns.len() >= 2 {
                    return
                }

                let mut variants = HashSet::new();

                variants.insert(true);
                variants.insert(false);

                for pat in patterns.iter() {
                    match &pat.0.kind {
                        PatternKind::Literal { value: Value { kind: ValueKind::BoolLiteral(b), .. }, .. } => {
                            variants.remove(b);
                        },
                        _ => {}
                    }
                }

                //self.debugger
                //    .throw_single(ErrorCode::NonExhaustiveMatch(variants.iter().map(|var| format!("{var}")).collect()), span);
            }

            _ => {}//self.debugger.throw_single(ErrorCode::NonExhaustiveMatch(vec!["default".to_string()]), span)
        }
    }

    fn switch_integer(
        &mut self,
        lowered_scrutinee: RValue,
        patterns: &Vec<(Pattern, DecisionTree)>,
        default_block: BasicBlockId) -> Vec<BasicBlockId>
    {
        let switch_branches = patterns.iter()
                                      .map(|(pat, _)| {
                                          let block = self.builder.append_block();
                                          let value = *match &pat.kind {
                                              PatternKind::Integer { value } => value,
                                              //PatternKind::Literal { value } => self.lower_value(value),
                                              _ => unreachable!(),
                                          };

										  SwitchArm { match_value: value, arm_block: block }
                                      })
                                      .collect::<Vec<_>>();

		self.builder.build_terminator(Terminator::switch(lowered_scrutinee, switch_branches.clone(), default_block));

        switch_branches.into_iter()
                       .map(|branch| branch.arm_block)
                       .collect()
    }

    fn switch_enum(
        &mut self,
        lowered_scrutinee: &Place,
        patterns: &Vec<(Pattern, DecisionTree)>,
        default_block: BasicBlockId) -> Vec<BasicBlockId>
    {
        let switch_branches = patterns.iter()
                                      .map(|(pat, _)| {
                                          let block = self.builder.append_block();
                                          let value = match &pat.kind {
                                              	  PatternKind::Literal { value } => match &value.kind {
													  ValueKind::EnumVariant { variant, .. } => variant.tag() as u64,
													  _ => unreachable!(),
												  }
                                              _ => unreachable!(),
                                          };
                                      
                                          SwitchArm { match_value: value, arm_block: block }
                                      })
                                      .collect::<Vec<_>>();

		let discriminant = self.builder.build_discriminant(lowered_scrutinee, lowered_scrutinee.span());

		self.builder.build_terminator(Terminator::switch(discriminant.copy(lowered_scrutinee.span()), switch_branches.clone(), default_block));

        switch_branches.into_iter()
                       .map(|branch| branch.arm_block)
                       .collect()
    }

	///
	/// Switch on a value by checking its equality
	/// 
    fn switch_struct(
        &mut self,
        lowered_scrutinee: &Place,
        patterns: &Vec<(Pattern, DecisionTree)>,
        default_block: BasicBlockId,
        struct_ref: &StructRef,
        scrutinee_span: &Option<Span>) -> Vec<BasicBlockId>
    {
        let Some(Symbol::Function(mut equal_function)) = struct_ref.lookup_static_item("op~equal") else {
            //self.debugger.throw_single(ErrorCode::OperatorNotDefined("equals".to_string(), struct_ref.name()), scrutinee_span);
            return vec![];
        };

        equal_function.filter_types(&vec![ TypeKind::Struct(struct_ref.clone()).anon(), TypeKind::Struct(struct_ref.clone()).anon() ]);

        let Some(equal_function) = equal_function.resolve() else {
            //self.debugger.throw_single(ErrorCode::OperatorNotDefined("equals".to_string(), struct_ref.name()), scrutinee_span);
            return vec![];
        };

        let equality_function = match equal_function {
            SomeFunction::StaticMethod(function) => self.builder.build_function(function.info().link_name(), lowered_scrutinee.span()),
            _ => unreachable!(),
        };

        let case_branches = patterns.iter()
                .map(|(pat, _)| {
                    let positive_branch = self.builder.append_block();
                    let negative_branch = self.builder.append_block();

                    let equal_to = match &pat.kind {
                        PatternKind::Literal { value } => self.lower_rvalue(value),
                        _ => unreachable!(),
                    };

                    let is_equal = equality_function.call(vec![ lowered_scrutinee.copy(lowered_scrutinee.span()), equal_to ], lowered_scrutinee.span() );

                    self.builder.build_terminator(Terminator::branch_if(is_equal, positive_branch, negative_branch));

                    self.builder.position_at_end(negative_branch);

                    positive_branch
                })
                .collect();

        self.builder.build_terminator(Terminator::goto(default_block));

        case_branches
    }

    pub fn lower_loop(
        &mut self,
        loop_code_block: &CodeBlock,
        label: &str)
    {
        // todo: have an after and before loop
        let break_block = self.builder.append_block();
        let loop_block = self.builder.append_block();
        self.builder.build_terminator(Terminator::goto(loop_block));
        self.builder.position_at_end(loop_block);

        self.break_labels.insert(label.to_string(), break_block);
        self.continue_labels.insert(label.to_string(), loop_block);

        self.lower_code_block(loop_code_block);

        self.break_labels.remove(label);
        self.continue_labels.remove(label);

        if !loop_code_block.escapes() {
            self.builder.build_terminator(Terminator::goto(loop_block));
        }
        self.builder.position_at_end(break_block);

        let after_block = self.builder.append_block();
        self.builder.build_terminator(Terminator::goto(after_block));
        self.builder.position_at_end(after_block);
    }
}