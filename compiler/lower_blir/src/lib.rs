mod typ;
mod func;
mod struct_;
mod smt;
mod val;

use std::collections::HashMap;

use blir::Library;
use blirssa::{Library as SsaLibrary, Builder, value::LabelValue, code::FunctionRef};

pub struct BlirLowerer {
    ssa_lib: SsaLibrary,
    builder: Builder,
    lib: Library,
    context: FunctionLowerContext,
}

impl BlirLowerer {
    pub fn new(lib: Library) -> BlirLowerer {
        BlirLowerer {
            ssa_lib: SsaLibrary::new(lib.name()),
            builder: Builder::new(),
            lib,
            context: FunctionLowerContext::new(),
        }
    }

    fn builder(&mut self) -> &mut Builder {
        &mut self.builder
    }

    fn ssa_library_mut(&mut self) -> &mut SsaLibrary {
        &mut self.ssa_lib
    }

    fn ssa_library(&self) -> &SsaLibrary {
        &self.ssa_lib
    }

    pub fn lower(&mut self) {
        // Lower struct definitions
        for r#struct in self.lib.structs.clone() {
            self.lower_struct_definition(r#struct);
        }
        
        // Lower struct signatures
        for r#struct in self.lib.structs.clone() {
            self.lower_struct_signatures(r#struct);
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

        // Lower function code
        for func in self.lib.functions.clone() {
            self.lower_func(func);
        }
    }

    pub fn finish(self) -> SsaLibrary {
        self.ssa_lib
    }
}

pub (crate) struct FunctionLowerContext {
    context_variables: HashMap<String, LabelValue>,
    function: Option<FunctionRef>,
}

impl FunctionLowerContext {
    pub fn define_var(&mut self, name: &str, val: LabelValue) {
        self.context_variables.insert(name.to_string(), val);
    }

    pub fn lookup_var(&self, name: &str) -> Option<&LabelValue> {
        self.context_variables.get(name)
    }

    pub fn enter_function(&mut self, function: &FunctionRef) {
        self.context_variables.clear();
        let _ = self.function.insert(function.clone());
    }

    pub fn function(&self) -> &FunctionRef {
        self.function
            .as_ref()
            .unwrap()
    }

    pub fn new() -> FunctionLowerContext {
        FunctionLowerContext {
            context_variables: HashMap::new(),
            function: None
        }
    }
}