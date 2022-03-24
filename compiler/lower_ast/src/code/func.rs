use blir::{code::{Function, FunctionRef, FuncParam, Method, MethodRef, ExternFunction, ExternFunctionRef}, Visibility, typ::{TypeKind, Type}, scope::ScopeRef};
use parser::{ast::{func::FuncDef}, lexer::SyntaxKind};

use crate::AstLowerer;

impl AstLowerer {
	pub fn lower_func(&self, func: FuncDef, parent: &ScopeRef) -> FunctionRef {
		let range = func.range();
		let span = self.span(range);

		let visibility = self.lower_visibility(func.visibility());
		let name = func.name();
		let params = func.parameters()
			.iter()
			.map(|param| FuncParam { label: None, bind_name: param.label(), typ: self.lower_type(param.typ()) })
			.collect();
		let return_type = func.return_type()
			.map(|rt| self.lower_type(rt))
			.unwrap_or(TypeKind::Void.anon());
		let code = self.lower_code_block(func.code().unwrap());
		
		Function::new(visibility, name, params, return_type, code, span, parent)
	}

	pub fn lower_extern_func(&self, func: FuncDef) -> ExternFunctionRef {
		let range = func.range();
		let span = self.span(range);

		let visibility = self.lower_visibility(func.visibility());
		let name = func.name();
		let params = func.parameters()
			.iter()
			.map(|param| FuncParam { label: None, bind_name: param.label(), typ: self.lower_type(param.typ()) })
			.collect();
		let return_type = func.return_type()
			.map(|rt| self.lower_type(rt))
			.unwrap_or(TypeKind::Void.anon());
		
		ExternFunction::new(visibility, name, params, return_type, span)
	}

	pub fn lower_method(&self, func: FuncDef, reciever: Type, parent: &ScopeRef) -> MethodRef {
		let range = func.range();
		let span = self.span(range);

		let visibility = self.lower_visibility(func.visibility());
		let is_static = func.is_static();
		let name = func.name();
		let params = func.parameters()
			.iter()
			.map(|param| FuncParam { label: None, bind_name: param.label(), typ: self.lower_type(param.typ()) })
			.collect();
		let return_type = func.return_type()
			.map(|rt| self.lower_type(rt))
			.unwrap_or(TypeKind::Void.anon());
		let code = self.lower_code_block(func.code().unwrap());
		
		Method::new(reciever, is_static, visibility, name, params, return_type, code, span, parent)
	}

	pub fn lower_visibility(&self, visibility: Option<SyntaxKind>) -> Visibility {
		match visibility {
			Some(SyntaxKind::PublicKw) => Visibility::Public,
			Some(SyntaxKind::InternalKw) => Visibility::Internal,
			Some(SyntaxKind::FilePrivateKw) => Visibility::Fileprivate,
			Some(SyntaxKind::PrivateKw) => Visibility::Private,

			None => Visibility::Internal,

			_ => panic!(),
		}
	}
}