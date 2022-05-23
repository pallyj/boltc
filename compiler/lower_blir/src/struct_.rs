use blir::{code::MethodRef,
           typ::{StructRef, TypeKind}};
use blirssa::{typ::StructField, value::LabelValue};

use crate::BlirLowerer;

impl<'a, 'b> BlirLowerer<'a, 'b> {
    pub(super) fn lower_struct_definition(&mut self, r#struct: StructRef) {
        let ssa_lib = self.ssa_library_mut();

        let is_packed = false;
        let is_transparent = r#struct.borrow().is_transparent;

        let link_name = r#struct.borrow().mangle();

        r#struct.borrow_mut().link_name = link_name;

        ssa_lib.add_struct(&r#struct.link_name(), is_transparent, is_packed);

        for substruct in r#struct.borrow().substructs.clone() {
            self.lower_struct_definition(substruct);
        }

        for subenum in r#struct.borrow().subenums.clone() {
            self.lower_enum_definition(subenum);
        }
    }

    pub(super) fn lower_struct_signatures(&mut self, r#struct: StructRef) {
        let borrowed_struct = r#struct.borrow();

        let self_struct = self.ssa_library()
                              .get_struct(&borrowed_struct.link_name)
                              .cloned()
                              .unwrap();

        for substruct in borrowed_struct.substructs.clone() {
            self.lower_struct_signatures(substruct);
        }

        for subenum in borrowed_struct.subenums.clone() {
            self.lower_enum_signature(subenum);
        }

        for var in &borrowed_struct.instance_vars {
            let borrowed_var = var.borrow();

            let ty = self.lower_type(&borrowed_var.typ);
            let field = StructField::new(&borrowed_var.name, ty);

            self_struct.add_field(field);
        }

        for method in &borrowed_struct.methods {
            self.lower_method_signature(method);
        }
    }

    pub(super) fn lower_struct_code(&mut self, r#struct: StructRef) {
        let borrowed_struct = r#struct.borrow();

        for substruct in borrowed_struct.substructs.clone() {
            self.lower_struct_code(substruct);
        }

        for subenum in borrowed_struct.subenums.clone() {
            self.lower_enum_code(subenum);
        }

        for method in &borrowed_struct.methods {
            self.lower_method(method);
        }
    }

    pub(crate) fn lower_method_signature(&mut self, method: &MethodRef) {
        let function_type = self.lower_type(&method.take_typ());

        self.ssa_library_mut()
            .add_function(method.borrow().info.link_name(), function_type);
    }

    pub(super) fn lower_method(&mut self, func: &MethodRef) {
        let method = self.ssa_library()
                         .get_function(func.borrow().info.link_name())
                         .cloned()
                         .unwrap();

        self.context.enter_function(&method);
        if !func.is_static() {
            self.context.define_var("self", method.arg(0));
        }

        let mut func_n = if func.is_static() { 0 } else { 1 };
        for param in func.borrow().info.params() {
            if let TypeKind::Void = param.typ.kind() {
                self.context
                    .define_var(&param.bind_name, LabelValue::void());
                continue;
            }
            let arg_value = method.arg(func_n);
            self.context.define_var(&param.bind_name, arg_value);
            func_n += 1;
        }

        let start_block = method.append_block("enter");
        self.builder().position_at_end(&start_block);

        let yield_value = self.lower_code_block(&func.borrow().code);
        if func.borrow().code.typ().kind() != &TypeKind::Divergent {
            self.builder().build_return(yield_value);
        }
    }
}
