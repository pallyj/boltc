#![feature(let_else)]

mod constraint;
mod infer_walker;

pub use infer_walker::*;

use std::{sync::atomic::{AtomicU64, Ordering}, collections::HashMap, fmt::Display};

use blir::{Type, TypeKind};
use constraint::Constraint;

static INFER_HEAD: AtomicU64 = AtomicU64::new(0);

pub fn type_infer_ctx() -> Type {
    let ctx = INFER_HEAD.fetch_add(1, Ordering::AcqRel);

    Type::new_anon(TypeKind::Infer(ctx))
}

pub struct TypeInferenceCtx {
    constraints: Vec<(u64, Constraint)>,
    solved: HashMap<u64, Type>,
    atoms: HashMap<u64, InferenceAtom>
}

impl TypeInferenceCtx {
    pub fn new() -> Self {
        Self {
            constraints: vec![],
            solved: HashMap::new(),
            atoms: HashMap::new(),
        }
    }

    pub fn add_constraint(&mut self, ctx: u64, constraint: Constraint) {
        if let Constraint::Equality(other) = &constraint {
            self.constraints.push((*other, Constraint::Equality(ctx)));
        }

        self.constraints.push((ctx, constraint))
    }

    fn atom(&mut self, ctx: u64) -> &mut InferenceAtom {
        if !self.atoms.contains_key(&ctx) {
            self.atoms.insert(ctx, InferenceAtom::new());
        }

        self.atoms.get_mut(&ctx).unwrap()
    }

    pub fn collect(&mut self) {
        for i in 0..self.constraints.len() {
            let constraint = self.constraints[i].clone();

            //println!("{}: {}", constraint.0, constraint.1);

            let atom = self.atom(constraint.0);

            match constraint.1 {
                Constraint::SomeInteger => {
                    atom.is_integer = true;
                }
                Constraint::SomeFloat => {
                    atom.is_float = true;
                }
                Constraint::Absolute(ty) => {
                    atom.equality = Some(ty)
                }
                Constraint::Suggestion(sug) => {
                    atom.suggestion = Some(sug)
                }
                Constraint::Equality(eq) => {
                    atom.sameness = Some(eq);
                }
                _ => {}
            }
        }
    }

    pub fn solve(&mut self) {
        self
            .atoms
            .retain(|ctx_num, atom| {
                //println!("{}:\n{}", ctx_num, atom);
    
                if let Some(eq) = &atom.equality {
                    self.solved.insert(*ctx_num, eq.clone());
                    false
                } else if let Some(sug) = &atom.suggestion {
                    if atom.is_integer {
                        // Check if it is an integer
    
                        self.solved.insert(*ctx_num, sug.clone());
                        false
                    } else {
                        true
                    }
                } else {
                    if atom.is_integer {
                        // Set it to the default integer type
    
                        self.solved.insert(*ctx_num, Type::new_anon(TypeKind::Intrinsic("i64".to_string())));
                        false
                    } else {
                        true
                    }
                }
            });

        let mut last_count = usize::MAX;

        while self.atoms.len() < last_count {
            last_count = self.atoms.len();

            self
                .atoms
                .retain(|ctx_num, atom| {

                    //println!("Infer {} still exists", ctx_num);
                    if let Some(other) = atom.sameness {
                        if let Some(other) = self.solved.get(&other) {
                            let other = other.clone();

                            self.solved.insert(*ctx_num, other);

                            return true
                        }
                    }

                    return false
                });
        }
    }

    pub fn get_type(&self, ctx: u64) -> Option<Type> {
        self.solved.get(&ctx).cloned()
    }

    pub fn print(&self) {
        for solved in &self.solved {
            println!("{}: {}", solved.0, solved.1);
        }
    }
}

pub struct InferenceAtom {
    is_integer: bool,
    is_float: bool,
    is_string: bool,
    is_bool: bool,
    is_collection: bool,
    is_record: bool,

    suggestion: Option<Type>,
    equality: Option<Type>,
    sameness: Option<u64>,
}

impl InferenceAtom {
    pub fn new() -> Self {
        InferenceAtom {
            is_integer: false,
            is_float: false,
            is_string: false,
            is_bool: false,
            is_collection: false,
            is_record: false,

            suggestion: None,
            equality: None,
            sameness: None,
        }
    }
}

impl Display for InferenceAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(suggestion) = &self.suggestion {
            writeln!(f, "suggestion: {suggestion}")?;
        }

        if let Some(equals) = &self.equality {
            writeln!(f, "equals: {equals}")?;
        }

        Ok(())
    }
}