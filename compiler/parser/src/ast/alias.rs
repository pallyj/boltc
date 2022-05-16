use std::fmt::Debug;

use crate::lexer::SyntaxKind;

use super::typ::Type;

ast!(struct TypeAlias(TypeAlias));


impl TypeAlias {
	pub fn visibility(&self) -> Option<SyntaxKind> {
        self.0
            .children()
            .find(|child| child.kind() == SyntaxKind::Visibility)
            .and_then(|visibility| visibility.first_token())
            .map(|tok| tok.kind())
    }
	
	pub fn name(&self) -> String {
        self.0
            .children()
            .find(|child| child.kind() == SyntaxKind::FuncName)
            .and_then(|func_name| func_name.first_token())
            .map(|name| name.text().to_string())
            .unwrap()
    }

	pub fn aliased_type(&self) -> Type {
		self.0
			.last_child()
			.map(Type::cast)
			.unwrap()
	}
}

impl Debug for TypeAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "typealias {} = {:?}", self.name(), self.aliased_type())
    }
}