use std::{fmt::Display,
          ops::Deref,
          panic,
          sync::{atomic::{AtomicU64, Ordering},
                 Arc, Weak}};

use crate::{typ::Type, value::LabelValue};

#[derive(Clone)]
pub struct ExternFunctionRef {
    func: Arc<ExternFunction>,
}

impl ExternFunctionRef {
    pub fn downgrade(&self) -> ExternFunctionWeakRef { ExternFunctionWeakRef { func: Arc::downgrade(&self.func), } }
}

impl Deref for ExternFunctionRef {
    type Target = ExternFunction;

    fn deref(&self) -> &Self::Target { &self.func }
}

pub struct ExternFunctionWeakRef {
    func: Weak<ExternFunction>,
}

impl ExternFunctionWeakRef {
    pub fn upgrade(&self) -> Option<ExternFunctionRef> { self.func.upgrade().map(|func| ExternFunctionRef { func }) }
}

impl Display for ExternFunctionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.func) }
}

pub struct ExternFunction {
    name: String,

    func_type: Type,

    index: AtomicU64,
}

impl ExternFunction {
    pub fn new(name: &str, func_type: Type) -> ExternFunctionRef {
        let n_pars = match func_type {
            Type::Function { ref pars, return_type: _ } => pars.len(),
            t => panic!("{t} is not a ExternFunction type"),
        };

        let func = ExternFunction { name: name.to_string(),
                                    func_type,
                                    index: AtomicU64::new(n_pars as u64) };

        ExternFunctionRef { func: Arc::new(func) }
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn next_index(&self) -> u64 { self.index.fetch_add(1, Ordering::Relaxed) }

    pub fn typ(&self) -> Type { self.func_type.clone() }

    pub fn arg(&self, n: usize) -> LabelValue {
        match self.func_type {
            Type::Function { ref pars, .. } => LabelValue { label: n as u64,
                                                            typ:   pars[n].clone(), },
            _ => panic!(),
        }
    }
}

impl Display for ExternFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { writeln!(f, "func {name}{type};", name = self.name, type = self.func_type) }
}
