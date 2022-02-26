use std::{collections::HashSet, sync::Arc};

use crate::{SourceFile, WithSource, GenericToken, BoltMessage};

pub trait GenericLexer: Sized {
	type Token: GenericToken;
	type Error: BoltMessage;

	fn into(self) -> (Arc<SourceFile>, Vec<WithSource<Self::Token>>, HashSet<usize>);

	fn errors(&self) -> Option<Vec<Self::Error>>;
}