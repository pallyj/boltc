use std::{cell::{Ref, RefCell},
          fmt::Display,
          ops::Deref,
          panic,
          sync::{atomic::{AtomicU64, Ordering},
                 Arc, Weak}};

use super::{Block, BlockRef};
use crate::{typ::Type, value::LabelValue};

#[derive(Clone)]
pub struct FunctionRef {
    func: Arc<Function>,
}

impl FunctionRef {
    pub fn append_block(&self, name: &str) -> BlockRef {
        let block_idx = self.blocks().len() as u64;

        let block = Block::new(name.to_string(), block_idx, self);

        self.blocks.borrow_mut().push(block.clone());

        block
    }

    pub fn downgrade(&self) -> FunctionWeakRef { FunctionWeakRef { func: Arc::downgrade(&self.func), } }
}

impl Deref for FunctionRef {
    type Target = Function;

    fn deref(&self) -> &Self::Target { &self.func }
}

pub struct FunctionWeakRef {
    func: Weak<Function>,
}

impl FunctionWeakRef {
    pub fn upgrade(&self) -> Option<FunctionRef> { self.func.upgrade().map(|func| FunctionRef { func }) }
}

impl Display for FunctionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.func) }
}

pub struct Function {
    name: String,

    func_type: Type,

    blocks: RefCell<Vec<BlockRef>>,

    index: AtomicU64,
}

impl Function {
    pub fn new(name: &str, func_type: Type) -> FunctionRef {
        let n_pars = match func_type {
            Type::Function { ref pars, return_type: _ } => pars.len(),
            t => panic!("{t} is not a function type"),
        };

        let func = Function { name: name.to_string(),
                              func_type,
                              blocks: RefCell::new(vec![]),
                              index: AtomicU64::new(n_pars as u64) };

        FunctionRef { func: Arc::new(func) }
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn next_index(&self) -> u64 { self.index.fetch_add(1, Ordering::Relaxed) }

    pub fn typ(&self) -> Type { self.func_type.clone() }

    pub fn blocks(&self) -> Ref<Vec<BlockRef>> { self.blocks.borrow() }

    pub fn arg(&self, n: usize) -> LabelValue {
        match self.func_type {
            Type::Function { ref pars, .. } => LabelValue { label: n as u64,
                                                            typ:   pars[n].clone(), },
            _ => panic!(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "func {name}{type} {{", name = self.name, type = self.func_type)?;

        for block in self.blocks.borrow().iter() {
            write!(f, "{block}")?;
        }

        writeln!(f, "}}")
    }
}
