use std::{sync::Arc, ops::Deref, cell::{Cell}};

#[derive(Clone)]
pub struct Case {
	name: String,
	tag: Cell<Option<usize>>
}

impl Case {
	pub fn new(name: String) -> CaseRef {
		CaseRef {
			case_ref: Arc::new(Case { name, tag: Cell::new(None) })
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
        write!(f, "case {} = {}", self.name(), self.tag())
    }
}