use std::sync::{Arc};

use prelude::*;
use bolt_ast::{
	Func as AstFunc
};
use blir::{
	FuncDef,
	FuncParam, Type, TypeKind, Scope, MethodDef, ExternFuncDef
};

use crate::{lower_type, lower_code_block};

pub fn lower_func(func: WithSource<AstFunc>, scope: &Arc<dyn Scope>) -> Try<Arc<FuncDef>, ()> {
	let (func, _source) = func.unwrap();

	let name = func.name()
		.clone()
		.unwrap_or("".to_string());

	let mut parameters = vec![];

	for par in func.parameters().clone() {
		let (par, _source) = par.unwrap();

		let label = par.label().clone();
		let bind_name = par.bind_name().clone();

		let typ = require!(lower_type(par.typ().clone()));

		parameters.push(FuncParam::new(label, bind_name, typ));
	}

	let return_type = match func.return_type() {
		Some(return_type) => require!(lower_type(return_type.clone())),
		None => Type::new_anon(TypeKind::Unit),
	};

	let code = require!(lower_code_block(func.code().unwrap().clone()));

	Try::Some(FuncDef::new(name, parameters, return_type, code, scope))
}

pub fn lower_extern_func(func: WithSource<AstFunc>, scope: &Arc<dyn Scope>) -> Try<Arc<ExternFuncDef>, ()> {
	let (func, _source) = func.unwrap();

	let name = func.name()
		.clone()
		.unwrap_or("".to_string());

	let mut parameters = vec![];

	for par in func.parameters().clone() {
		let (par, _source) = par.unwrap();

		let label = par.label().clone();
		let bind_name = par.bind_name().clone();

		let typ = require!(lower_type(par.typ().clone()));

		parameters.push(FuncParam::new(label, bind_name, typ));
	}

	let return_type = match func.return_type() {
		Some(return_type) => require!(lower_type(return_type.clone())),
		None => Type::new_anon(TypeKind::Unit),
	};

	Try::Some(ExternFuncDef::new(name, parameters, return_type, scope))
}

pub fn lower_method(func: WithSource<AstFunc>, scope: &Arc<dyn Scope>) -> Try<Arc<MethodDef>, ()> {
	let (func, _source) = func.unwrap();

	let name = func.name()
		.clone()
		.unwrap_or("".to_string());

	let mut parameters = vec![];

	for par in func.parameters().clone() {
		let (par, _source) = par.unwrap();

		let label = par.label().clone();
		let bind_name = par.bind_name().clone();

		let typ = require!(lower_type(par.typ().clone()));

		parameters.push(FuncParam::new(label, bind_name, typ));
	}

	let return_type = match func.return_type() {
		Some(return_type) => require!(lower_type(return_type.clone())),
		None => Type::new_anon(TypeKind::Unit),
	};

	let code = require!(lower_code_block(func.code().unwrap().clone()));

	Try::Some(MethodDef::new(name, parameters, return_type, code, scope))
}