use std::fmt::Debug;

use crate::WithSource;

use super::Source;

pub trait BoltMessage {
	/// The error code
	fn code(&self) -> String;

	/// The error name
	fn name(&self) -> String;

	/// The error description
	fn description(&self) -> String;

	/// Whether this is an error, or a warning
	fn level(&self) -> MessageLevel;
}

#[derive(PartialEq, Eq)]
pub enum MessageLevel {
	Warning,
	Error,
}

pub enum Try<T, E> {
	Some(T),
	None(E),
	Err(E),
}

impl<T, E> Try<T, WithSource<E>> {
	pub fn discard_error_source(self) -> Try<T, E> {
		match self {
			Try::Some(val) => Try::Some(val),
			Try::None(err) => {
				let (err, _) = err.unwrap();
				Try::None(err)
			}
			Try::Err(err) => {
				let (err, _) = err.unwrap();
				Try::Err(err)
			}
		}
	}
}

impl<T: Debug, E> Debug for Try<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(arg0) => f.debug_tuple("Some").field(arg0).finish(),
            Self::None(arg0) => Ok(()),
            Self::Err(arg0) => Ok(()),
        }
    }
}

/// Unwraps a try
/// 
/// If Some(val), yields val
/// If None(err), returns None(err)
/// If Err(err), returns Err(err) 
#[macro_export]
macro_rules! unwrap {
	($try:expr) => {
		match $try {
			$crate::Try::Some(val) => val,
			$crate::Try::None(e) => return $crate::Try::None(e),
			$crate::Try::Err(e) => return $crate::Try::Err(e), 
		}
	};
}

/// Requires a try to be Some
/// 
/// If Some(val), yields val
/// If None(err), returns Err(err)
/// If Err(err), returns Err(err) 
#[macro_export]
macro_rules! require {
	($try:expr) => {
		match $try {
			$crate::Try::Some(val) => val,
			$crate::Try::None(e) => return $crate::Try::Err(e),
			$crate::Try::Err(e) => return $crate::Try::Err(e), 
		}
	};
}

/// Requires a try to be Some
/// 
/// If Some(val), yields val
/// If None(err), returns Err(err)
/// If Err(err), returns Err(err) 
#[macro_export]
macro_rules! into_result {
	($try:expr) => {
		match $try {
			$crate::Try::Some(val) => Ok(val),
			$crate::Try::None(e) => Err(e),
			$crate::Try::Err(e) => return $crate::Try::Err(e), 
		}
	};
}