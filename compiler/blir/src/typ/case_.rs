use std::{sync::Arc, ops::Deref, cell::{Cell, RefCell, Ref, RefMut}};

use super::Type;

#[derive(Clone)]
pub struct Case {
	name: String,
	tag: Cell<Option<usize>>,
	associated: RefCell<Vec<Type>>
}

impl Case {
	pub fn new(name: String, associated: Vec<Type>) -> CaseRef {
		CaseRef {
			case_ref: Arc::new(Case { name,
							   tag: Cell::new(None),
							   associated: RefCell::new(associated) })
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn tag(&self) -> usize {
		self.tag.get().unwrap()
	}

	pub fn set_tag(&self, val: usize) {
		self.tag.set(Some(val));
	}

	pub fn associated_types(&self) -> Ref<Vec<Type>> {
		self.associated.borrow()
	}

	pub fn associated_types_mut(&self) -> RefMut<Vec<Type>> {
		self.associated.borrow_mut()
	}
}

#[derive(Clone)]
pub struct CaseRef {
	case_ref: Arc<Case>
}

impl Deref for CaseRef {
    type Target = Case;

    fn deref(&self) -> &Self::Target {
        &self.case_ref
    }
}

impl std::fmt::Debug for CaseRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.associated_types().len() == 0 {
        	write!(f, "case {} = {}", self.name(), self.tag())
		} else {
			let assoc_ty = self.associated_types()
				.iter()
				.map(|ty| format!("{ty:?}"))
				.collect::<Vec<_>>()
				.join(", ");
			write!(f, "case {}({assoc_ty}) = {}", self.name(), self.tag())
		}
    }
}

impl PartialEq for CaseRef {
    fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.case_ref, &other.case_ref)
    }
}

impl Eq for CaseRef {
    fn assert_receiver_is_total_eq(&self) {}
}