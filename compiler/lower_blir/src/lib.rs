#![feature(let_else)]
#![feature(drain_filter)]

mod func;
mod smt;
mod struct_;
mod typ;
mod val;
mod enum_;
mod match_;

use std::collections::HashMap;

use blir::{value::Closure, Library};
use blirssa::{code::FunctionRef, value::LabelValue, Builder, Library as SsaLibrary};
use errors::DiagnosticReporter;

pub struct BlirLowerer<'a, 'b> {
    ssa_lib:  SsaLibrary,
    builder:  Builder,
    lib:      Library,
    context:  FunctionLowerContext,
    closures: Vec<(String, Closure)>,
    debugger: &'a mut DiagnosticReporter<'b>
}

impl<'a, 'b> BlirLowerer<'a, 'b> {
    pub fn new(lib: Library, debugger: &'a mut DiagnosticReporter<'b>) -> Self {
        BlirLowerer { ssa_lib: SsaLibrary::new(lib.name()),
                      builder: Builder::new(),
                      lib,
                      context: FunctionLowerContext::new(),
                      closures: Vec::new(),
                      debugger }
    }

    fn builder(&mut self) -> &mut Builder { &mut self.builder }

    fn ssa_library_mut(&mut self) -> &mut SsaLibrary { &mut self.ssa_lib }

    fn ssa_library(&self) -> &SsaLibrary { &self.ssa_lib }

    pub fn lower(&mut self) {
        // Lower struct definitions
        for r#struct in self.lib.structs.clone() {
            self.lower_struct_definition(r#struct);
        }

        // Lower enum definitions
        for r#enum in self.lib.enums.clone() {
            self.lower_enum_definition(r#enum);
        }

        // Lower struct signatures
        for r#struct in self.lib.structs.clone() {
            self.lower_struct_signatures(r#struct);
        }

         // Lower enum signatures
         for r#enum in self.lib.enums.clone() {
            self.lower_enum_signature(r#enum);
        }

        // Lower function signatures
        for func in self.lib.functions.clone() {
            self.lower_func_signature(func);
        }

        // Lower extern function signatures
        for func in self.lib.extern_functions.clone() {
            self.lower_extern_func_signature(func);
        }

        // Lower struct code
        for r#struct in self.lib.structs.clone() {
            self.lower_struct_code(r#struct);
        }

        // Lower enum code
        for r#enum in self.lib.enums.clone() {
            self.lower_enum_code(r#enum);
        }

        // Lower function code
        for func in self.lib.functions.clone() {
            self.lower_func(func);
        }

        for (name, closure) in self.closures.clone() {
            self.lower_closure_code(&name, &closure);
        }
    }

    pub fn finish(self) -> SsaLibrary { self.ssa_lib }
}

pub(crate) struct FunctionLowerContext {
    context_variables: HashMap<String, LabelValue>,
    function:          Option<FunctionRef>,
}

impl FunctionLowerContext {
    pub fn define_var(&mut self, name: &str, val: LabelValue) { self.context_variables.insert(name.to_string(), val); }

    pub fn lookup_var(&self, name: &str) -> Option<&LabelValue> { self.context_variables.get(name) }

    pub fn enter_function(&mut self, function: &FunctionRef) {
        self.context_variables.clear();
        let _ = self.function.insert(function.clone());
    }

    pub fn function(&self) -> &FunctionRef { self.function.as_ref().unwrap() }

    pub fn new() -> FunctionLowerContext {
        FunctionLowerContext { context_variables: HashMap::new(),
                               function:          None, }
    }
}
