use std::{sync::Arc, ops::Deref, cell::{Cell, RefCell, Ref, RefMut}};

use errors::Span;

use super::Type;

#[derive(Clone)]
pub struct Case {
	name: String,
	tag: Cell<Option<usize>>,
	associated: RefCell<Vec<Type>>,
	labels: Vec<Option<String>>,
	span: Span,
	meta: String
}

impl Case {
	pub fn new(name: String, associated: Vec<Type>, labels: Vec<Option<String>>, span: Span, meta: String) -> CaseRef {
		CaseRef {
			case_ref: Arc::new(Case { name,
							   tag: Cell::new(None),
							   associated: RefCell::new(associated),
							   labels,
							   span,
							   meta})
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn tag(&self) -> usize {
		self.tag.get().unwrap()
	}

	pub fn has_tag(&self) -> bool {
		self.tag.get().is_some()
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

	pub fn labels(&self) -> &Vec<Option<String>> {
		&self.labels
	}

	pub fn span(&self) -> Span {
		self.span
	}

	pub fn meta(&self) -> String {
		self.meta.clone()
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
			let assoc_ty = self.labels().iter().zip(self.associated_types().iter())
				.map(|(label, ty)| if let Some(label) = label {
					format!("{label}: {ty:?}")
				} else {
					format!("{ty:?}")
				})
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