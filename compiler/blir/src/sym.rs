use crate::{code::{ExternFunctionRef, FunctionRef, MethodRef},
            scope::ScopeRelation,
            typ::TypeKind,
            value::{ConstantRef, Value, VarRef},
            Visibility};

#[derive(Debug, Clone)]
pub enum Symbol {
    Value(Value),

    Constant(ConstantRef),

    Function(FunctionRef),
    StaticMethod(MethodRef),
    InstanceMethod(MethodRef),
    ExternFunction(ExternFunctionRef),

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
}
