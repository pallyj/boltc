use errors::Span;

use crate::{code::{ExternFunctionRef, FunctionInfo, FunctionRef, MethodRef},
            scope::ScopeRelation,
            typ::{Type, TypeKind},
            value::{ConstantRef, Value, VarRef},
            Visibility};

#[derive(Debug, Clone)]
pub enum Symbol {
    Value(Value),

    Constant(ConstantRef),

    Function(Monomorphizer),

    InstanceVariable(VarRef),

    Type(TypeKind),
}

#[derive(Clone)]
pub struct SymbolWrapper {
    sym: Symbol,
    vis: Visibility,
}

impl SymbolWrapper {
    pub fn new(sym: Symbol, vis: Visibility) -> Self { Self { sym, vis } }

    pub fn filter(self, relation: ScopeRelation) -> Option<SymbolWrapper> {
        if relation.can_access(self.vis) {
            Some(self)
        } else {
            None
        }
    }

    pub fn resolve(self) -> Symbol { self.sym }

    pub fn visibility(&self) -> Visibility { self.vis }

    pub fn symbol(&mut self) -> &mut Symbol { &mut self.sym }
}

#[derive(Debug, Clone)]
pub enum SomeFunction {
    Function(FunctionRef),
    StaticMethod(MethodRef),
    InstanceMethod(MethodRef),
    ExternFunction(ExternFunctionRef),
}

impl SomeFunction {
    pub fn visibility(&self) -> Visibility {
        match self {
            Self::Function(function) => function.borrow().visibility,
            Self::ExternFunction(function) => function.borrow().visibility,
            Self::InstanceMethod(method) => method.borrow().visibility,
            Self::StaticMethod(method) => method.borrow().visibility,
        }
    }

    pub fn info(&self) -> &FunctionInfo {
        match self {
            Self::Function(function) => function.info(),
            Self::ExternFunction(function) => function.info(),
            Self::InstanceMethod(method) => method.info(),
            Self::StaticMethod(method) => method.info(),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Function(function) => function.borrow().span.clone(),
            Self::ExternFunction(function) => function.borrow().span.clone(),
            Self::InstanceMethod(method) => method.borrow().span.clone(),
            Self::StaticMethod(method) => method.borrow().span.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monomorphizer {
    functions: Vec<SomeFunction>,
}

impl Monomorphizer {
    pub fn new() -> Monomorphizer { Monomorphizer { functions: vec![] } }

    pub fn add_function(&mut self, function: FunctionRef) { self.functions.push(SomeFunction::Function(function)) }

    pub fn add_method(&mut self, function: MethodRef) {
        let some_function = if function.is_static() {
            SomeFunction::StaticMethod(function)
        } else {
            SomeFunction::InstanceMethod(function)
        };

        self.functions.push(some_function)
    }

    pub fn add_extern_function(&mut self, function: ExternFunctionRef) { self.functions.push(SomeFunction::ExternFunction(function)) }

    // fn params(info: &FunctionInfo) -> (Vec<Option<String>>, Vec<Type>) {
    // info.params()
    // .clone()
    // .into_iter()
    // .map(|param| (param.label, param.typ))
    // .unzip()
    // }

    pub fn combine(&mut self, mut other: Monomorphizer) { self.functions.append(&mut other.functions) }

    pub fn filter_visibility(&mut self, relationship: ScopeRelation) {
        self.functions
            .retain(|sig| visibility_matches(sig, relationship))
    }

    pub fn filter_labels(&mut self, labels: &Vec<Option<String>>) {
        self.functions
            .retain(|sig| labels_match(&sig.info(), labels));
    }

    pub fn filter_types(&mut self, types: &Vec<Type>) { self.functions.retain(|sig| types_match(&sig.info(), types)); }

    pub fn degrees(&self) -> usize { self.functions.len() }

    pub fn open_possibilities(&self) -> &Vec<SomeFunction> { &self.functions }

    pub fn resolve(&self) -> Option<SomeFunction> {
        if self.degrees() == 1 {
            Some(self.functions[0].clone())
        } else {
            None
        }
    }
}

fn labels_match(sig: &FunctionInfo, labels: &Vec<Option<String>>) -> bool {
    sig.params()
       .iter()
       .zip(labels)
       .all(|(sig_label, label)| &sig_label.label == label)
}

fn types_match(sig: &FunctionInfo, types: &Vec<Type>) -> bool {
    if sig.params().len() != types.len() {
        return false;
    }

    sig.params()
       .iter()
       .zip(types)
       .all(|(sig, types)| is_assignable_from(&sig.typ, types))
}

fn is_assignable_from(ty1: &Type, ty2: &Type) -> bool {
    if ty2.kind() == &TypeKind::Divergent {
        return true;
    }

    match (ty1.kind(), ty2.kind()) {
        (TypeKind::Function { return_type: return_type1,
                              params: params1,
                              .. },
         TypeKind::Function { return_type: return_type2,
                              params: params2,
                              .. }) => {
            if !is_assignable_from(return_type1, return_type2) {
                return false;
            }

            if params1.iter()
                      .zip(params2)
                      .any(|(ty1, ty2)| !is_assignable_from(ty1, ty2))
            {
                return false;
            }

            true
        }
        (_, TypeKind::Infer { .. }) => true,
        (_, TypeKind::Divergent) => true,
        (t1, t2) if t1 == t2 => true,
        _ => false,
    }
}

fn visibility_matches(func: &SomeFunction, relationship: ScopeRelation) -> bool { relationship.can_access(func.visibility()) }
