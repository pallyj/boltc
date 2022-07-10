use std::fmt::Debug;

use mangle::Path;

use crate::{code::{ExternFunctionRef, FunctionRef},
            scope::{ScopeRef, ScopeRelation, ScopeType},
            typ::{StructRef, TypeKind, EnumRef},
            value::{ConstantRef, GlobalVarRef},
            Symbol, SymbolWrapper, Visibility};

pub struct Library {
    name: String,

    scope: ScopeRef,

    pub path: Path,

    pub functions:        Vec<FunctionRef>,
    pub extern_functions: Vec<ExternFunctionRef>,
    pub structs:          Vec<StructRef>,
    pub enums:            Vec<EnumRef>,
    pub constants:        Vec<ConstantRef>,
    pub globals:          Vec<GlobalVarRef>,

    files: Vec<ScopeRef>,
}

impl Library {
    pub fn new(name: &str) -> Library {
        Library { name:             name.to_string(),
                  path:             Path::new(name),
                  scope:            ScopeRef::new(None, ScopeRelation::None, ScopeType::Library, false, false),
                  functions:        Vec::new(),
                  extern_functions: Vec::new(),
                  structs:          Vec::new(),
                  enums:            Vec::new(),
                  constants:        Vec::new(),
                  globals:          Vec::new(),
                  files:            Vec::new(), }
    }

    pub fn new_file(&mut self) -> ScopeRef {
        let scope = ScopeRef::new(Some(&self.scope),
                                  ScopeRelation::SameLibrary,
                                  ScopeType::File,
                                  false,
                                  false);

        self.files.push(scope.clone());

        scope
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn add_function(&mut self, func: FunctionRef) -> bool {
        // Add the function to the list of functions
        self.functions.push(func.clone());

        // Add the functions symbol, returning another symbol if it exists
        let name = func.borrow().info.name().clone();

        self.scope.add_function(name, func)
    }

    pub fn add_extern_function(&mut self, func: ExternFunctionRef) -> bool {
        // Add the function to the list of functions
        self.extern_functions.push(func.clone());

        // Add the functions symbol, returning another symbol if it exists
        let name = func.borrow().info.name().clone();

        self.scope.add_extern_function(name, func)
    }

    pub fn add_struct(&mut self, r#struct: StructRef) -> Option<SymbolWrapper> {
        // Add the substruct to the list of substructs
        self.structs.push(r#struct.clone());

        // Add the substructs symbol, returning another symbol if it exists
        let visibility = r#struct.visibility();
        let name = r#struct.name();

        let symbol = Symbol::Type(TypeKind::Struct(r#struct));
        self.scope.add_symbol(name, visibility, symbol)
    }

    pub fn add_enum(&mut self, r#enum: EnumRef) -> Option<SymbolWrapper> {
        // Add the substruct to the list of substructs
        self.enums.push(r#enum.clone());

        // Add the substructs symbol, returning another symbol if it exists
        let visibility = r#enum.visibility();
        let name = r#enum.name().to_string();

        let symbol = Symbol::Type(r#enum.get_type());
        self.scope.add_symbol(name, visibility, symbol)
    }

    pub fn add_type(&self, name: String, visibility: Visibility, typ: TypeKind) -> Option<SymbolWrapper> {
        let sym = Symbol::Type(typ);

        self.scope.add_symbol(name, visibility, sym)
    }

    pub fn add_constant(&mut self, var: ConstantRef) -> Option<SymbolWrapper> {
        // Add the function to the list of functions
        self.constants.push(var.clone());

        // Add the functions symbol, returning another symbol if it exists
        let cloned = var.clone();
        let var_ref = cloned.borrow();

        let visibility = var_ref.visibility;
        let name = var_ref.name.clone();

        let symbol = Symbol::Constant(var);

        self.scope.add_symbol(name, visibility, symbol)
    }

    pub fn add_global(
        &mut self,
        global: GlobalVarRef) -> Option<SymbolWrapper>
    {
        self.globals.push(global.clone());

        let visibility = global.visibility();
        let name = global.name();
        let symbol = Symbol::Global(global);

        self.scope.add_symbol(name, visibility, symbol)
    }

    pub fn scope(&self) -> &ScopeRef { &self.scope }

    pub fn path(&self) -> &Path { &self.path }
}

impl Debug for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for global in &self.globals {
            writeln!(f, "{global:?}")?;
        }

        for r#struct in &self.structs {
            writeln!(f, "{struct:?}")?;
        }

        for r#enum in &self.enums {
            writeln!(f, "{enum:?}")?;
        }

        for func in &self.functions {
            writeln!(f, "{func:?}")?;
        }

        for func in &self.extern_functions {
            writeln!(f, "{func:?}")?;
        }

        Ok(())
    }
}
