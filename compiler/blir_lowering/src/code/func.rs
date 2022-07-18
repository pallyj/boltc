use blir::{code::{FunctionRef, MethodRef, FuncParam, ExternFunctionRef}, value::Closure, typ::TypeKind};
use errors::Span;
use itertools::Itertools;
use mir::{instr::Terminator, val::Place};

use crate::BlirLowerer;

impl<'a> BlirLowerer<'a> {
	///
	/// Adds the signature of a function to a project
	/// 
	pub fn lower_function_signature(&mut self, func: &FunctionRef) {
		let borrowed_func = func.borrow();

		let func_name = borrowed_func.info.link_name();
		
		let parameters = borrowed_func.info.params()
										   .iter()
										   .map(|it| {
												let ty = self.lower_ty(&it.typ);

												if it.is_shared {
													ty.shared_pointer()
												}
												else { ty }
											})
											.collect_vec();
		let return_type = self.lower_ty(borrowed_func.info.return_type());

		let is_entry_point = func.info().is_entry_point();

		self.builder.add_function(func_name, parameters, return_type, is_entry_point);
	}

	/// 
	/// Adds the signature of a method to a project
	/// 
	pub fn lower_method_signature(&mut self, method: &MethodRef) {
		let method_type = self.lower_ty(&method.take_typ());
		let borrowed_method = method.borrow();
		let method_name = borrowed_method.info.link_name();

		let mir::ty::TypeKind::Function { mut parameters, return_type } = method_type.into_kind() else {
			panic!("compiler error: function had non-function return type")
		};

		let parameter_iter = (!method.is_static()).then(|| method.is_mutating())
			.into_iter()
			.chain(borrowed_method.info.params().iter().map(|FuncParam { is_shared, .. }| *is_shared));

		for (parameter_type, is_shared) in parameters.iter_mut().zip(parameter_iter) {
			if is_shared {
				*parameter_type = std::mem::take(parameter_type).shared_pointer();
			}
		}

		self.builder.add_function(method_name, parameters.clone(), (*return_type).clone(), false);
	}

	///
	/// Adds the signature of an extern function to a project
	/// 
	pub fn lower_extern_function(&mut self, func: &ExternFunctionRef) {
		let borrowed_func = func.borrow();

		let func_name = borrowed_func.info.link_name();
		let parameters = borrowed_func.info.params()
										   .iter()
										   .map(|it| {
												let ty = self.lower_ty(&it.typ);

												if it.is_shared { ty.shared_pointer() }
												else { ty }
											})
											.collect_vec();
		let return_type = self.lower_ty(borrowed_func.info.return_type());

		self.builder.add_extern_function(func_name, parameters, return_type);
	}

	///
	/// Lower function code
	/// 
	pub (crate) fn lower_function_code(
		&mut self,
		func: &FunctionRef)
	{
		let borrowed_function = func.borrow();

		// Position our builder on the equivalent mir function
		let function_id = self.builder.get_function_id(borrowed_function.info.link_name());
		self.builder.position_on_func(function_id);

		self.function_ctx.clear();

		// Get the function so we can use its type
		let function = self.builder.get_function_by_id(function_id);

		// Add parameters to the local context
		for (i, (parameter, param_type)) in borrowed_function.info.params().iter().zip(function.params()).enumerate() {
			let place = Place::function_param(i, param_type.clone(), Self::span_of(parameter.typ.span())); // Use the span of the name

			if parameter.is_shared {
				self.function_ctx.insert(parameter.bind_name.clone(), place.copy(Span::empty()).deref(Span::empty()));
			} else {
				self.function_ctx.insert(parameter.bind_name.clone(), place);
			}
		}

		// Push a block onto the function and position the builder on it
		let start_block = self.builder.append_block();
		self.builder.position_at_end(start_block);

		// Lower the code
        let yield_value = self.lower_code_block(&borrowed_function.code);

		// Check if the function has an explicit return
		// If it does, it would cause a segfault to return here
        if !borrowed_function.code.escapes() {
			// If we have a yielded value, return it
			if let Some(return_val) = yield_value {
				self.builder.build_terminator(Terminator::returns(return_val))
			} else {
				self.builder.build_terminator(Terminator::return_void())
			}
        }
    }

	pub (crate) fn lower_method_code(
		&mut self,
		method: &MethodRef)
	{
		let borrowed_method = method.borrow();

		// Position our builder on its equivalent mir code
		let function_id = self.builder.get_function_id(borrowed_method.info.link_name());
		self.builder.position_on_func(function_id);

		// Enter a new function
		self.function_ctx.clear();

		// Get the function so we can use its type
		let function = self.builder.get_function_by_id(function_id);

		// Optionally add self parameter to function
		let params = (!borrowed_method.is_static).then_some(("self", borrowed_method.is_mutating))
			.into_iter()
			.chain(borrowed_method.info.params().iter().map(|FuncParam { bind_name, is_shared, .. }| (bind_name.as_str(), *is_shared)));

		// Add parameters to function
		for (i, ((bind_name, is_shared), param_type)) in params.zip(function.params()).enumerate() {
			let param = Place::function_param(i, param_type.clone(), Span::empty());

			if is_shared {
				self.function_ctx.insert(bind_name.to_string(), param.copy(Span::empty()).deref(Span::empty()));
			} else {
				self.function_ctx.insert(bind_name.to_string(), param);
			}
		}

		// Push a block onto the function and position the builder on it
		let start_block = self.builder.append_block();
		self.builder.position_at_end(start_block);

		// Lower the code
        let yield_value = self.lower_code_block(&borrowed_method.code);

		// Check if the function has an explicit return
		// If it does, it would cause a segfault to return here
        if !borrowed_method.code.escapes() {
			// If we have a yielded value, return it
			if let Some(return_val) = yield_value {
				self.builder.build_terminator(Terminator::returns(return_val))
			} else {
				self.builder.build_terminator(Terminator::return_void())
			}
        }
	}

	pub(crate) fn lower_closure_code(&mut self, name: &str, closure: &Closure) {
        let function_id = self.builder.get_function_id(name);

        self.builder.position_on_func(function_id);

		// Enter a new function
		self.function_ctx.clear();

        for (i, param) in closure.params.iter().enumerate() {
            if let TypeKind::Void = param.typ.kind() {
                // self.context.define_var(&param.name, LabelValue::void());
                // continue;
            }
			let param_ty = self.lower_ty(&param.typ);
            let arg_value = Place::function_param(i, param_ty, Self::span_of(param.typ.span()));
			self.function_ctx.insert(param.name.clone(), arg_value);
        }

        // Push a block onto the function and position the builder on it
		let start_block = self.builder.append_block();
		self.builder.position_at_end(start_block);

		// Lower the code
        let yield_value = self.lower_code_block(&closure.code);

		// Check if the function has an explicit return
		// If it does, it would cause a segfault to return here
        if !closure.code.escapes() {
			// If we have a yielded value, return it
			if let Some(return_val) = yield_value {
				self.builder.build_terminator(Terminator::returns(return_val))
			} else {
				self.builder.build_terminator(Terminator::return_void())
			}
        }
	}
}