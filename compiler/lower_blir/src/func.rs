use blir::{code::{ExternFunctionRef, FunctionRef},
           typ::TypeKind,
           value::Closure};
use blirssa::value::LabelValue;

use crate::BlirLowerer;

impl BlirLowerer {
    pub(super) fn lower_func_signature(&mut self, func: FunctionRef) {
        let function_type = self.lower_type(&func.take_typ());

        self.ssa_library_mut()
            .add_function(func.borrow().info.link_name(), function_type);
    }

    pub(super) fn lower_extern_func_signature(&mut self, func: ExternFunctionRef) {
        let borrowed = func.borrow();
        let name = &borrowed.info.link_name();
        let function_type = self.lower_type(&func.take_typ());

        self.ssa_library_mut()
            .add_extern_function(name, function_type);
    }

    pub(super) fn lower_func(&mut self, func: FunctionRef) {
        let function = self.ssa_library()
                           .get_function(func.borrow().info.link_name())
                           .cloned()
                           .unwrap();

        self.context.enter_function(&function);

        let mut func_n = 0;
        for param in func.borrow().info.params() {
            if let TypeKind::Void = param.typ.kind() {
                self.context
                    .define_var(&param.bind_name, LabelValue::void());
                continue;
            }
            let arg_value = function.arg(func_n);
            self.context.define_var(&param.bind_name, arg_value);
            func_n += 1;
        }

        let start_block = function.append_block("enter");
        self.builder().position_at_end(&start_block);

        let yield_value = self.lower_code_block(&func.borrow().code);
        if func.borrow().code.typ().kind() != &TypeKind::Divergent {
            self.builder().build_return(yield_value);
        }
    }

    pub(super) fn lower_closure_code(&mut self, name: &str, closure: &Closure) {
        let function = self.ssa_library().get_function(name).cloned().unwrap();

        self.context.enter_function(&function);

        for (func_n, param) in closure.params.iter().enumerate() {
            if let TypeKind::Void = param.typ.kind() {
                // self.context.define_var(&param.name, LabelValue::void());
                // continue;
            }
            let arg_value = function.arg(func_n);
            self.context.define_var(&param.name, arg_value);
        }

        let start_block = function.append_block("enter");
        self.builder().position_at_end(&start_block);

        let yield_value = self.lower_code_block(&closure.code);
        if closure.code.typ().kind() != &TypeKind::Divergent {
            self.builder().build_return(yield_value);
        }
    }
}
