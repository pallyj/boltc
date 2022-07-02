use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Visibility {
	Private,
	Fileprivate,
	Internal,
	Public,
}

impl Visibility {
	pub fn compose(from: blir::Visibility) -> Self {
		match from {
			blir::Visibility::Local => Visibility::Public,
			blir::Visibility::Public => Visibility::Public,
			blir::Visibility::Internal => Visibility::Internal,
			blir::Visibility::Fileprivate => Visibility::Fileprivate,
			blir::Visibility::Private => Visibility::Private,
		}
	}
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Private => write!(f, "private"),
            Visibility::Fileprivate => write!(f, "fileprivate"),
            Visibility::Internal => write!(f, "internal"),
            Visibility::Public => write!(f, "public"),
        }
    }
}