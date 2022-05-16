use std::{fmt::Debug};

use blir::{
    pattern::{PatternKind, Pattern, self},
    value::{match_::MatchValue, MatchBranch, Value, ValueKind},
    typ::{TypeKind, Type, StructRef},
    Symbol,
    SomeFunction, code::CodeBlock};
use blirssa::{value::LabelValue, code::{BlockRef, Block}};
use patmat::{PatternMatrix, Maranget, DecisionTree};

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

        /*let mut candidate_tree = self.generate_candidates(&value.discriminant, &value.branches);

        self.optimize_candidate(&mut candidate_tree);

		self.lower_some_candidate(candidate_tree, &before_block, None, assign_val_ptr.as_ref());*/

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
            DecisionTree::Leaf(end) => {
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
                let scrutinee = self.lower_value(&scrutinee);

                if let Some(default) = default {
                    let default_block = self.context.function().append_block("default");

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

                    self.builder().build_select_integer(scrutinee, switch_branches.clone(), default_block.clone());

                    self.builder().position_at_end(&default_block);
                    self.lower_decision_tree(*default, leaves, pointer, sink);

                    for ((_, block), (_, tree)) in switch_branches.iter().zip(patterns) {
                        self.builder().position_at_end(&block);
                        self.lower_decision_tree(tree, leaves, pointer, sink);
                    }
                } else {
                    panic!()
                }
            }
        }
    }

    fn optimize_candidate(
        &mut self,
        candidate: &mut Candidate)
    {
        let mut old_candidates = std::mem::take(&mut candidate.next_candidates);

        while old_candidates.len() > 0 {
            let mut last_candidate = old_candidates.pop().unwrap();

            // Get a list of the patterns that overlap
            let duplicate_trees = old_candidates
                .drain_filter(|possible_duplicate| {
                    // Drain it if its pattern is equivalent to last_candidate
                    Self::patterns_match(&possible_duplicate.0, &last_candidate.0)
                }).collect::<Vec<_>>();

            // Merge the patterns and insert last_candidate
            for duplicate in duplicate_trees {
                Self::merge_candidate(&mut last_candidate.1, duplicate.1);
            }

            candidate.next_candidates.insert(0, last_candidate);
        }

        // Now optimize the children
        for next_candidate in &mut candidate.next_candidates {
            self.optimize_candidate(&mut next_candidate.1);
        }
    }

    fn patterns_match(pattern1: &Pattern, pattern2: &Pattern) -> bool {
        match (&pattern1.kind, &pattern2.kind) {
            (PatternKind::Bind(_), PatternKind::Bind(_)) |
            (PatternKind::Bind(_), PatternKind::Wildcard) |
            (PatternKind::Wildcard, PatternKind::Bind(_)) => true,

            (PatternKind::Literal { value: value1 }, PatternKind::Literal { value: value2 }) => {
                // Check if the values match
                match (&value1.kind, &value2.kind) {
                    (ValueKind::IntLiteral(n), ValueKind::IntLiteral(n2)) => n == n2,
                    (ValueKind::StringLiteral(s), ValueKind::StringLiteral(s2)) => s == s2,
                    (ValueKind::EnumVariant { variant: v1, .. }, ValueKind::EnumVariant { variant: v2, .. }) => v1.name() == v2.name(),

                    _ => false,
                }
            }

            _ => false
        }
    }

    fn merge_candidate(into: &mut Candidate, from: Candidate) {
        into.next_candidates.extend(from.next_candidates);

        if into.default_branch.is_some() {
            if from.default_branch.is_some() {
                panic!()
            }
            return
        }
        into.default_branch = from.default_branch;
    }

	fn lower_some_candidate(
		&mut self,
		candidate_tree: Candidate,
		before_branch: &BlockRef,
        last_default_branch: Option<&BlockRef>,
		output_pointer: Option<&LabelValue>
	) {
        let Some(match_value) = &candidate_tree.match_value else {
            // Generate the default branch
            let default_branch = candidate_tree.default_branch.unwrap();
            if let Some((yielded_value, assign_val_ptr)) = self.lower_code_block(&default_branch).zip(output_pointer) {
                self.builder.build_assign_ptr(assign_val_ptr.clone(), yielded_value);
            };
            return
        };

        let next_default_branch =
            if candidate_tree.default_branch.is_some() ||
               candidate_tree.next_candidates.iter()
                   .any(|(pat, _)| pat.matches_any())
        {
            None
        } else {
            last_default_branch
        };

        // We are in a block to do the candidate
        match candidate_tree.match_type.clone() {
            MatchType::Integer => {
                self.lower_integer_layer(match_value.clone(),
                                         candidate_tree,
                                         before_branch,
                                         next_default_branch,
                                         output_pointer)
            },
            MatchType::Enum => {
                self.lower_enum_layer(match_value.clone(),
                                      candidate_tree,
                                      before_branch,
                                      next_default_branch,
                                      output_pointer);
            },
            MatchType::Equality(struct_ref) => {
                let csr = struct_ref.clone();
                self.lower_equality_layer(match_value.clone(),
                                          candidate_tree,
                                          before_branch,
                                          next_default_branch,
                                          &csr,
                                          output_pointer)
            },
            MatchType::None => {}
        }
        
        // Codegen for the candidate tree



		/*match value.typ.kind() {
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
		}*/
	}

	fn lower_integer_layer(
		&mut self,
		value: LabelValue,
		tree: Candidate,
		before_branch: &BlockRef,
        last_default_branch: Option<&BlockRef>,
		output_pointer: Option<&LabelValue>)
	{
        let basic_blocks = (0..tree.next_candidates.len())
            .map(|_| self.context.function().append_block("matchArm"))
            .collect::<Vec<_>>();

        let mut default_branch_implicit = None;

        let branch_values = basic_blocks
            .iter()
            .zip(tree.next_candidates.iter())
            .filter_map(|(block, (pattern, _))| {
                let value = match &pattern.kind {
                    PatternKind::Literal { value } => self.lower_value(value),
                    PatternKind::Bind(_) | PatternKind::Wildcard => {
                        default_branch_implicit = Some(block.clone());
                        return None
                    }
                    _ => return None
                };

                Some((value, block.clone()))
            }).collect();

        let default_branch = if let Some(last_default_branch) = last_default_branch {
            last_default_branch.clone()
        } else if let Some(new_default_branch) = default_branch_implicit {
            new_default_branch
        }
        else {
            self.context.function().append_block("defaultArm")
        };

        self
            .builder()
            .build_select_integer(value, branch_values, default_branch.clone());

        if let Some(default_code_block) = tree.default_branch {
            self.builder().position_at_end(&default_branch);

            if let Some((yielded_value, assign_val_ptr)) = self.lower_code_block(&default_code_block).zip(output_pointer) {
                self.builder.build_assign_ptr(assign_val_ptr.clone(), yielded_value);
            };

            self.builder().build_always_branch(before_branch);
        }

        for (basic_block, (_, branch)) in basic_blocks.iter().zip(tree.next_candidates) {
            self.builder().position_at_end(&basic_block);

            self.lower_some_candidate(branch, before_branch, Some(&default_branch), output_pointer);

            self.builder().build_always_branch(before_branch);
        }
	}

    fn lower_enum_layer(
		&mut self,
		value: LabelValue,
		tree: Candidate,
		before_branch: &BlockRef,
        last_default_branch: Option<&BlockRef>,
		output_pointer: Option<&LabelValue>)
	{
        let basic_blocks = (0..tree.next_candidates.len())
            .map(|_| self.context.function().append_block("matchArm"))
            .collect::<Vec<_>>();

        let mut default_branch_implicit = None;

        let branch_values = basic_blocks
            .iter()
            .zip(tree.next_candidates.iter())
            .filter_map(|(block, (pattern, _))| {
                let value = match &pattern.kind {
                    PatternKind::Literal { value } => match &value.kind {
                        ValueKind::EnumVariant { variant, .. } => variant.name().to_string(),
                        _ => return None
                    }
                    PatternKind::Bind(_) | PatternKind::Wildcard => {
                        default_branch_implicit = Some(block.clone());
                        return None
                    }
                    _ => return None
                };

                Some((value, block.clone()))
            }).collect();

        let default_branch = if let Some(last_default_branch) = last_default_branch {
            last_default_branch.clone()
        } else if let Some(new_default_branch) = default_branch_implicit {
            new_default_branch
        }
        else {
            self.context.function().append_block("defaultArm")
        };

        self
            .builder()
            .build_select_enum(value, branch_values, default_branch.clone());

        if let Some(default_code_block) = tree.default_branch {
            self.builder().position_at_end(&default_branch);

            if let Some((yielded_value, assign_val_ptr)) = self.lower_code_block(&default_code_block).zip(output_pointer) {
                self.builder.build_assign_ptr(assign_val_ptr.clone(), yielded_value);
            };

            self.builder().build_always_branch(before_branch);
        }

        for (basic_block, (_, branch)) in basic_blocks.iter().zip(tree.next_candidates) {
            self.builder().position_at_end(&basic_block);

            self.lower_some_candidate(branch, before_branch, Some(&default_branch), output_pointer);

            self.builder().build_always_branch(before_branch);
        }
	}

    // Move the new stuff over to this case
    fn lower_equality_layer(
		&mut self,
		value: LabelValue,
		candidate: Candidate,
		before_branch: &BlockRef,
        last_default_branch: Option<&BlockRef>,
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
	}


    fn generate_candidates(
        &mut self,
        value: &Value,
		branches: &[MatchBranch]) -> Candidate
    {
        let lowered_value = self.lower_value(value);

        let match_type = match &value.typ.kind {
            TypeKind::Integer { .. } => MatchType::Integer,
            TypeKind::Struct(struct_ref) if struct_ref.integer_repr() => MatchType::Integer,
            TypeKind::Enum(_) => MatchType::Enum,
            TypeKind::Struct(struct_ref) => MatchType::Equality(struct_ref.clone()),
            _ => MatchType::None,
        };

        let mut candidate = Candidate { match_value: Some(lowered_value.clone()),
                                        match_type,
                                        next_candidates: vec![],
                                        default_branch: None };

        let mut tuple_match_values = vec![];

        for branch in branches {
            match &branch.pattern.kind {
                PatternKind::Literal { .. } => {
                    // Get the match type

                    // Add a next candidate to the branch
                    let pattern = branch.pattern.clone();
                    let next_candidate = Candidate { match_value: None,
                                                     match_type: MatchType::None,
                                                     next_candidates: vec![],
                                                     default_branch: Some(branch.code.clone()) };

                    candidate.next_candidates.push((pattern, next_candidate));
                }

                PatternKind::Wildcard => {
                    // Set the default branch
                    candidate.default_branch = Some(branch.code.clone())
                }

                PatternKind::Tuple { items } => {
                    let TypeKind::Tuple(tuple_types) = value.typ.kind() else {
                        panic!()
                    };

                    let mut tuple_candidate = &mut candidate;

                    for (tuple_index, pattern) in items.iter().enumerate() {
                        if tuple_match_values.len() <= tuple_index {
                            tuple_match_values
                                .push(self.builder()
                                          .build_deref_tuple_field(lowered_value.clone(), 
                                                                   tuple_index))
                        }

                        tuple_candidate.match_value = Some(tuple_match_values[tuple_index].clone());

                        let match_type = match tuple_types[tuple_index].kind() {
                            TypeKind::Integer { .. } => MatchType::Integer,
                            TypeKind::Struct(struct_ref) if struct_ref.integer_repr() => MatchType::Integer,
                            TypeKind::Enum(_) => MatchType::Enum,
                            TypeKind::Struct(struct_ref) => MatchType::Equality(struct_ref.clone()),
                            _ => panic!(),
                        };

                        tuple_candidate.match_type = match_type;

                        let tuple_next = Candidate { match_value: None,
                                                     match_type: MatchType::None,
                                                     next_candidates: vec![],
                                                     default_branch: None };

                        if let PatternKind::Bind(name) = &pattern.kind {
                            // Now, we bind name to the current value we are switching on
                            self.context.define_var(name, tuple_candidate.match_value.clone().unwrap());
                        }

                        tuple_candidate.next_candidates.push((pattern.clone(), tuple_next));

                        tuple_candidate = &mut tuple_candidate.next_candidates.last_mut().unwrap().1;
                    }

                    tuple_candidate.default_branch = Some(branch.code.clone())
                }

                PatternKind::Variant { variant, items } => {
                    let pattern = PatternKind::Literal { value: variant.clone() }.with_span(branch.pattern.span);
                    let next_candidate = Candidate { match_value: None,
                                                     match_type: MatchType::None,
                                                     next_candidates: vec![],
                                                     default_branch: None };

                    candidate.next_candidates.push((pattern, next_candidate));

                    let ValueKind::EnumVariant { variant, .. } = &variant.kind else {
                        panic!()
                    };

                    let tuple_types = variant.associated_types();
                    let variant_tuple = self.builder().build_cast_enum_variant(lowered_value.clone(), variant.name());

                    let mut tuple_candidate = &mut candidate.next_candidates.last_mut().unwrap().1;

                    for (tuple_index, pattern) in items.iter().enumerate() {
                        if tuple_match_values.len() <= tuple_index {
                            tuple_match_values
                                .push(self.builder()
                                          .build_deref_tuple_field(variant_tuple.clone(), 
                                                                   tuple_index))
                        }

                        tuple_candidate.match_value = Some(tuple_match_values[tuple_index].clone());

                        let match_type = match tuple_types[tuple_index].kind() {
                            TypeKind::Integer { .. } => MatchType::Integer,
                            TypeKind::Struct(struct_ref) if struct_ref.integer_repr() => MatchType::Integer,
                            TypeKind::Enum(_) => MatchType::Enum,
                            TypeKind::Struct(struct_ref) => MatchType::Equality(struct_ref.clone()),
                            _ => panic!(),
                        };

                        tuple_candidate.match_type = match_type;

                        let tuple_next = Candidate { match_value: None,
                                                     match_type: MatchType::None,
                                                     next_candidates: vec![],
                                                     default_branch: None };

                        if let PatternKind::Bind(name) = &pattern.kind {
                            // Now, we bind name to the current value we are switching on
                            self.context.define_var(name, tuple_candidate.match_value.clone().unwrap());
                        }

                        tuple_candidate.next_candidates.push((pattern.clone(), tuple_next));

                        tuple_candidate = &mut tuple_candidate.next_candidates.last_mut().unwrap().1;
                    }

                    tuple_candidate.default_branch = Some(branch.code.clone())
                }

                PatternKind::Bind(name) => {
                    // Set the default branch
                    candidate.default_branch = Some(branch.code.clone());

                    // Now, we bind name to the current value we are switching on
                    self.context.define_var(name, lowered_value.clone());
                }
            }
        }

        candidate
    }
}

// Integer(Value)
//


// (.north, .north)

// .north ~> .north =>> true
// .east ~> .east =>> true
// .south ~> .south =>> true
// .west ~> .west =>> true
// _ =>> true

// Tree
// Value
// Pattern


//      |--.north--|-.north---
//      |          |------------------|
//      |--.east---|-.east----        |
// -----|          |------------------|
//      |--.south--|-.south---        |-------
//      |          |------------------|
//      |--.west---|-.west----        |
//                 |------------------|


// match value tuple.0
//  against
//    .north
//      match value tuple.1
//       against
//        .north
//         return true
//    .east
//      match value tuple.1
//       against
//        .east
//         return true
//    .south
//      match value tuple.1
//       against
//        .south
//         return true
//    .west
//      match value tuple.1
//       against
//        .south
//         return true

// Match
//   value: LabelValue
//   patterns: Array<Match>
//   default: Option<CodeBlock>

/*

Lower tuple algorithm

Loop through the match arms
Get a collection of the first item of the tuple
Generate branches for them
Generate sub-branches with all branches matching this one
Rinse and repeat

*/

#[derive(Clone)]
enum MatchType {
    Integer,
    Enum,
    Equality(StructRef),
    None
}

pub struct Candidate {
    match_value: Option<LabelValue>,
    match_type: MatchType,
    next_candidates: Vec<(Pattern, Candidate)>,
    default_branch: Option<CodeBlock>,
}

impl Debug for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.next_candidates.len() == 0 && self.default_branch.is_some() {
            return writeln!(f, "{}", format!("{:?}", self.default_branch.as_ref().unwrap()).replace("\n", "\n\t"))
        }

        if let Some(match_value) = &self.match_value {
            writeln!(f, "match {}", match_value)?;
        }

        for (pat, cand) in &self.next_candidates {
            writeln!(f, "\t {pat:?} => {}", format!("{cand:?}").replace("\n", "\n\t"))?;
        }

        if let Some(default_branch) = &self.default_branch {
            writeln!(f, "\t _ => {}", format!("{default_branch:?}").replace("\n", "\n\t"))?;
        }

        Ok(())
    }
}