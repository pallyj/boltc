pub mod struct_;

use std::ops::Add;

use inkwell::{types::{IntType, FloatType, BasicTypeEnum, BasicType, PointerType, ArrayType, FunctionType, StructType, BasicMetadataTypeEnum}, AddressSpace};
use itertools::Itertools;
use mir::ty::{StructId, EnumId};

use crate::MirLowerer;

impl MirLowerer
{
	///
	/// Lowers a mir type into an LLVM type to be used as a raw variable
	/// 
	pub fn lower_ty(&self, ty: mir::ty::Type) -> BasicTypeEnum
	{
		use mir::ty::TypeKind::*;

		match ty.into_kind()
		{
			Integer { bits } => self.lower_int_ty(bits).into(),
			Float { bits } => self.lower_float_ty(bits).into(),

			Pointer(element) => self.lower_pointer_ty(*element).into(),
			Array { item, count } => self.lower_array_ty(*item, count).into(),
			Function { parameters, return_type } => self.lower_func_ty(*return_type, parameters).ptr_type(AddressSpace::Global).into(),
			Tuple(items) => self.lower_tuple_ty(items).into(),

			Struct { id } => self.lower_struct_ty(id).into(),
			Enum { id } => self.lower_enum_ty(id).into(),
		}
	}

	fn lower_int_ty(&self, bits: u32) -> IntType
	{
		match bits
		{
			64 => self.context.i64_type(),
			32 => self.context.i32_type(),
			16 => self.context.i16_type(),
			8 => self.context.i8_type(),

			_ => unreachable!(),
		}
		
	}

	fn lower_float_ty(&self, bits: u32) -> FloatType
	{
		match bits
		{
			64 => self.context.f64_type(),
			32 => self.context.f32_type(),
			16 => self.context.f16_type(),

			_ => unreachable!()
		}
	}

	///
	/// Lowers a mir type into an LLVM type to be used as a variable
	/// 
	pub fn lower_pointer_ty(&self, element: mir::ty::Type) -> PointerType
	{
		let address_space = AddressSpace::Global;

		match element.into_kind()
		{
			mir::ty::TypeKind::Integer { bits } => self.lower_int_ty(bits).ptr_type(address_space),
			mir::ty::TypeKind::Float { bits } => self.lower_float_ty(bits).ptr_type(address_space),

			mir::ty::TypeKind::Pointer(element) => self.lower_pointer_ty(*element).ptr_type(address_space),
			mir::ty::TypeKind::Array { item, count } => self.lower_array_ty(*item, count).ptr_type(address_space),
			mir::ty::TypeKind::Function { parameters, return_type } => self.lower_func_ty(*return_type, parameters).ptr_type(address_space),

			mir::ty::TypeKind::Tuple(items) => self.lower_tuple_ty(items).ptr_type(address_space),

			mir::ty::TypeKind::Struct { id } => self.lower_struct_ty(id).ptr_type(address_space),
			mir::ty::TypeKind::Enum { id } => self.lower_enum_ty(id).ptr_type(address_space),
		}
	}

	fn lower_array_ty(&self, element: mir::ty::Type, count: usize) -> ArrayType
	{
		let Ok(count) = u32::try_from(count) else
		{
			panic!("array is too big");
		};

		match element.into_kind() {
			mir::ty::TypeKind::Integer { bits } => self.lower_int_ty(bits).array_type(count),
			mir::ty::TypeKind::Float { bits } => self.lower_float_ty(bits).array_type(count),

			mir::ty::TypeKind::Pointer(element) => self.lower_pointer_ty(*element).array_type(count),
			mir::ty::TypeKind::Array { item, count: inner_count } => self.lower_array_ty(*item, inner_count ).array_type(count),
			mir::ty::TypeKind::Function { parameters, return_type } => {
				self.lower_func_ty(*return_type, parameters).ptr_type(AddressSpace::Global).array_type(count)
			}

			mir::ty::TypeKind::Tuple(items) => self.lower_tuple_ty(items).array_type(count),

			mir::ty::TypeKind::Struct { id } => self.lower_struct_ty(id).array_type(count),
			mir::ty::TypeKind::Enum { id } => self.lower_enum_ty(id).array_type(count),
		}
	}

	///
	/// Lowers a mir type into an LLVM type to be used for function calls
	/// 
	pub fn lower_func_ty(&self, return_type: mir::ty::Type, params: Vec<mir::ty::Type>) -> FunctionType
	{
		let return_type_lowered = self.lower_ty(return_type);

		let params_lowered: Vec<BasicMetadataTypeEnum> =
			params.into_iter()
				  .map(|param| self.lower_ty(param))
				  .map(|param| param.as_basic_type_enum().into())
				  .collect_vec();

		return_type_lowered.fn_type(&params_lowered, false)
	}

	fn lower_tuple_ty(&self, items: Vec<mir::ty::Type>) -> StructType
	{
		let items_lowered = items.into_iter()
								 .map(|item| self.lower_ty(item))
								 .collect_vec();

		self.context.struct_type(&items_lowered, false)
	}

	fn lower_struct_ty(&self, id: StructId) -> BasicTypeEnum {
		let structure = self.project.get_struct(id).unwrap();

		if structure.is_transparent() {
			let field_type = structure.fields().first().unwrap();

			self.lower_ty(field_type.clone())
		} else {
			self.module.get_struct_type(structure.name()).expect("struct doesn't exist").into()
		}
	}

	fn lower_enum_ty(&self, id: EnumId) -> StructType {
		self.context.opaque_struct_type(&format!("_e{id:?}"))
	}
}

