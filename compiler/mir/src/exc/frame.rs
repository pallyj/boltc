use itertools::Itertools;

use crate::{instr::{Local, LocalId}, Project, ty::Type};

use super::{val::Value, var::Var};

pub struct StackFrame {
	locals: Vec<Var>,
}

impl StackFrame {
	///
	/// 
	/// 
	pub fn new(params: &[Type], locals: &[Local], project: &Project) -> Self {
		let vars = params.iter()
						 .chain(locals.into_iter().map(|local| local.ty()))
						 .map(|ty| Var::new(ty.clone(), project))
						 .collect_vec();

		StackFrame { locals: vars }
	}

	///
	/// 
	/// 
	pub fn use_parameters(&mut self, pars: Vec<Value>) {
		for (i, par) in pars.into_iter().enumerate() {
			self.locals[i].set(par);
		}
	}

	///
	/// 
	/// 
	pub fn get_local(&self, local: LocalId) -> &Var {
		&self.locals[local.local_idx()]
	}

	///
	/// 
	/// 
	pub fn get_local_mut(&mut self, local: LocalId) -> &mut Var {
		&mut self.locals[local.local_idx()]
	}
}