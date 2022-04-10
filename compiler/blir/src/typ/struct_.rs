use std::{cell::{Ref, RefCell, RefMut},
          fmt::Debug,
          ops::Deref,
          sync::Arc};

use mangle::{Path, MangledStruct};

use super::{Type, TypeKind};
use crate::{attributes::Attributes,
            code::MethodRef,
            scope::{ScopeRef, ScopeRelation, ScopeType},
            value::{ConstantRef, VarRef},
            Symbol, SymbolWrapper, Visibility};

pub struct Struct {
    inner: RefCell<StructInner>,
}

#[allow(dead_code)]
pub struct StructInner {
    pub attributes: Attributes,
    pub visibility: Visibility,

    pub name: String,

    pub link_name: String,

    scope: ScopeRef,

    pub substructs:    Vec<StructRef>,
    pub methods:       Vec<MethodRef>,
    pub instance_vars: Vec<VarRef>,
    pub constants:     Vec<ConstantRef>,

    pub is_transparent: bool,

    path: Path,
}

impl StructInner {
    pub fn scope(&self) -> &ScopeRef { &self.scope }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn mangle(&self) -> String {
        MangledStruct(&self.path).to_string()
    }
}

impl Struct {
    pub fn new(attributes: Attributes, visibility: Visibility, name: String, parent: &ScopeRef, parent_path: Path) -> StructRef {
        let r#struct = StructInner { attributes,
                                     visibility,
                                     link_name: name.clone(),
                                     scope: ScopeRef::new(Some(parent),
                                                          ScopeRelation::SameFile,
                                                          ScopeType::Container,
                                                          false,
                                                          false),
                                     substructs: Vec::new(),
                                     methods: Vec::new(),
                                     instance_vars: Vec::new(),
                                     path: parent_path.append(&name),
                                     name,
                                     constants: Vec::new(),
                                     is_transparent: false };

        let struct_ref = StructRef { r#struct: Arc::new(Struct { inner: RefCell::new(r#struct), }), };

        struct_ref.add_type("Self".to_string(),
                            Visibility::Private,
                            TypeKind::Struct(struct_ref.clone()));

        struct_ref
    }

    pub fn add_substruct(&self, substruct: StructRef) -> Option<SymbolWrapper> {
        // Add the substruct to the list of substructs
        self.inner.borrow_mut().substructs.push(substruct.clone());

        // Add the substructs symbol, returning another symbol if it exists
        let visibility = substruct.visibility();
        let name = substruct.name();

        let symbol = Symbol::Type(TypeKind::Struct(substruct));

        self.borrow().scope.add_symbol(name, visibility, symbol)
    }

    pub fn add_method(&self, method: MethodRef) -> Option<SymbolWrapper> {
        // Add the function to the list of functions
        self.inner.borrow_mut().methods.push(method.clone());

        // Add the functions symbol, returning another symbol if it exists
        let visibility = method.visibility();
        let name = method.name();
        let is_static = method.is_static();

        if is_static {
            let symbol = Symbol::StaticMethod(method);

            self.borrow().scope.add_symbol(name, visibility, symbol)
        } else {
            let symbol = Symbol::InstanceMethod(method);

            self.borrow()
                .scope
                .add_instance_symbol(name, visibility, symbol)
        }
    }

    pub fn add_var(&self, var: VarRef) -> Option<SymbolWrapper> {
        // Add the function to the list of functions
        self.inner.borrow_mut().instance_vars.push(var.clone());

        // Add the functions symbol, returning another symbol if it exists
        let cloned = var.clone();
        let var_ref = cloned.borrow();

        let visibility = var_ref.visibility;
        let name = var_ref.name.clone();

        let symbol = Symbol::InstanceVariable(var);

        self.borrow()
            .scope
            .add_instance_symbol(name, visibility, symbol)
    }

    pub fn add_constant(&self, var: ConstantRef) -> Option<SymbolWrapper> {
        // Add the function to the list of functions
        self.inner.borrow_mut().constants.push(var.clone());

        // Add the functions symbol, returning another symbol if it exists
        let cloned = var.clone();
        let var_ref = cloned.borrow();

        let visibility = var_ref.visibility;
        let name = var_ref.name.clone();

        let symbol = Symbol::Constant(var);

        self.borrow().scope.add_symbol(name, visibility, symbol)
    }

    pub fn add_type(&self, name: String, visibility: Visibility, typ: TypeKind) -> Option<SymbolWrapper> {
        let sym = Symbol::Type(typ);

        self.borrow().scope.add_symbol(name, visibility, sym)
    }

    pub fn borrow(&self) -> Ref<StructInner> { self.inner.borrow() }

    pub fn borrow_mut(&self) -> RefMut<StructInner> { self.inner.borrow_mut() }

    pub fn visibility(&self) -> Visibility { self.inner.borrow().visibility }

    pub fn name(&self) -> String { unsafe { &*self.inner.as_ptr() }.name.clone() }

    pub fn link_name(&self) -> String { unsafe { &*self.inner.as_ptr() }.link_name.clone() }

    pub fn lookup_static_item(&self, name: &str) -> Option<Symbol> {
        unsafe { &*self.inner.as_ptr() }.scope()
                                        .lookup_static_member(name)
                                        .map(|sym| sym.resolve())
    }

    pub fn lookup_instance_item(&self, name: &str, scope: &ScopeRef) -> Option<Symbol> {
        // TODO: Maybe flip this?
        let rel = self.inner.borrow().scope.relation_to(scope);

        unsafe { &*self.inner.as_ptr() }.scope()
                                        .lookup_instance_member(name)
                                        .and_then(|sym| sym.filter(rel))
                                        .map(|sym| sym.resolve())
    }
}

#[derive(Clone)]
pub struct StructRef {
    r#struct: Arc<Struct>,
}

impl StructRef {
    pub fn params(&self) -> Vec<Type> {
        let struct_ptr = unsafe { &*self.inner.as_ptr() };

        struct_ptr.instance_vars
                  .iter()
                  .map(|var| var.borrow().typ.clone())
                  .collect()
    }

    pub fn integer_repr(&self) -> bool {
        let struct_ptr = unsafe { &*self.inner.as_ptr() };

        let vars = &struct_ptr.instance_vars;

        if vars.len() != 1 {
            return false;
        }

        match vars[0].borrow().typ.kind() {
            TypeKind::Integer { bits } => *bits > 1,
            _ => false,
        }
    }

    pub fn float_repr(&self) -> bool {
        let struct_ptr = unsafe { &*self.inner.as_ptr() };

        let vars = &struct_ptr.instance_vars;

        if vars.len() != 1 {
            return false;
        }

        match vars[0].borrow().typ.kind() {
            TypeKind::Float { .. } => true,
            _ => false,
        }
    }

    pub fn bool_repr(&self) -> bool {
        let struct_ptr = unsafe { &*self.inner.as_ptr() };

        let vars = &struct_ptr.instance_vars;

        if vars.len() != 1 {
            return false;
        }

        match vars[0].borrow().typ.kind() {
            TypeKind::Integer { bits } => *bits == 1,
            _ => false,
        }
    }
}

impl Deref for StructRef {
    type Target = Struct;

    fn deref(&self) -> &Self::Target { self.r#struct.deref() }
}

impl PartialEq for StructRef {
    fn eq(&self, other: &Self) -> bool { Arc::ptr_eq(&self.r#struct, &other.r#struct) }
}

impl Eq for StructRef {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Debug for StructRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} struct {} {{", self.visibility(), self.name())?;

        for var in self.borrow().instance_vars.iter() {
            writeln!(f, "\t{}", format!("{var:?}").replace("\n", "\t"))?;
        }

        for substruct in self.borrow().substructs.iter() {
            writeln!(f, "\t{}", format!("{substruct:?}").replace("\n", "\t"))?;
        }

        for func in self.borrow().methods.iter() {
            writeln!(f, "\t{}", format!("{func:?}").replace("\n", "\n\t"))?;
        }

        write!(f, "}}")
    }
}
