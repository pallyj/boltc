use {
	bolt_ast::Type as AstType
};
use {
	blir::{Type as BlirType, TypeKind as BlirTypeKind},
};
use prelude::*;

pub fn lower_type(ty: WithSource<AstType>) -> Try<BlirType, ()> {
	let (ty, source) = ty.unwrap();

	let kind = match ty {
		AstType::Unit => BlirTypeKind::Unit,

		AstType::Tuple(items) => {
			let mut blir_items = vec![];

			for ast_item in items.into_iter() {
				blir_items.push(require!(lower_type(ast_item)));
			}

			BlirTypeKind::Tuple(blir_items)
		}

		AstType::Named(name) => BlirTypeKind::Named(name),

		//AstType::Collection(unit) => BlirTypeKind::

		_ => BlirTypeKind::Named("Unknown".to_string()),
	};

	Try::Some(BlirType::new(kind, source))
}