use crate::operators::OperatorFactory;

pub struct Context {
	factory: OperatorFactory,
}

impl Context {
	pub fn new() -> Self {
		let mut factory = OperatorFactory::new();

		factory.register_intrinsics();

		Self {
			factory,
		}
	}

	pub fn factory(&self) -> &OperatorFactory {
		&self.factory
	}

	pub fn factory_mut(&mut self) -> &mut OperatorFactory {
		&mut self.factory
	}
}