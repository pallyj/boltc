use std::collections::{HashMap, HashSet};

use crate::{FuncSig, Type, TypeKind, Scope, ScopeKind, Symbol, Visibility, SymbolKind, Metadata, Expr, ExprKind};

macro_rules! sig {
    ( ($($params:expr),*): $ret:expr ) => {
        {
            let mut parameters = Vec::new();

            $(parameters.push($params.clone());)*

            FuncSig::new(parameters, $ret.clone())
        }
    };
}

pub struct Intrinsics {
    types: HashSet<String>,
    funcs: HashMap<String, FuncSig>,
    metadata: Metadata
}

impl Intrinsics {
    pub fn new() -> Intrinsics {
        Intrinsics {
            types: HashSet::new(),
            funcs: HashMap::new(),
            metadata: Metadata::new(),
        }
    }

    pub fn populate(&mut self) {
        self.add_integer_type(8);
        self.add_integer_type(16);
        self.add_integer_type(32);
        self.add_integer_type(64);
        self.add_type("i1".to_string());
    }

    /// Adds an integer type, with arithmatic, binary, comparison, and conversion operations
    pub fn add_integer_type(&mut self, bits: usize) {
        self.add_type(format!("i{bits}"));

        let t = Type::new_anon(TypeKind::Intrinsic(format!("i{bits}")));
        let b = Type::new_anon(TypeKind::Intrinsic("i1".to_string()));

        // Arithmatic
        self.add_func(format!("integer{bits}Add"), sig!((t, t): t));
        self.add_func(format!("integer{bits}Sub"), sig!((t, t): t));
        self.add_func(format!("integer{bits}Mul"), sig!((t, t): t));
        self.add_func(format!("integer{bits}Div"), sig!((t, t): t));
        self.add_func(format!("integer{bits}Rem"), sig!((t, t): t));
        self.add_func(format!("integer{bits}DivSig"), sig!((t, t): t));
        self.add_func(format!("integer{bits}RemSig"), sig!((t, t): t));

        // Binary
        self.add_func(format!("integer{bits}Or"), sig!((t, t): t));
        self.add_func(format!("integer{bits}Xor"), sig!((t, t): t));
        self.add_func(format!("integer{bits}And"), sig!((t, t): t));

        // Binary
        self.add_func(format!("integer{bits}Shl"), sig!((t, t): t));
        self.add_func(format!("integer{bits}Shr"), sig!((t, t): t));
        self.add_func(format!("integer{bits}ShrSig"), sig!((t, t): t));

        // Comparison
        self.add_func(format!("integer{bits}CmpEq"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpNeq"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpLt"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpGt"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpLte"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpGte"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpLtSig"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpGtSig"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpLteSig"), sig!((t, t): b));
        self.add_func(format!("integer{bits}CmpGteSig"), sig!((t, t): b));

        // Unary
        self.add_func(format!("integer{bits}Negate"), sig!((t): t));
        
        // Extension
        let mut b = bits * 2;

        while b <= 64 {
            let o = Type::new_anon(TypeKind::Intrinsic(format!("i{b}")));

            self.add_func(format!("integer{bits}ExtZero{b}"), sig!((t): o));
            self.add_func(format!("integer{bits}ExtSig{b}"), sig!((t): o));

            b *= 2;
        }

        // Truncation
        let mut b = bits / 2;

        while b >= 8 {
            let o = Type::new_anon(TypeKind::Intrinsic(format!("i{b}")));

            self.add_func(format!("integer{bits}Trunc{b}"), sig!((t): o));

            b /= 2;
        }
    }

    pub fn add_func(&mut self, named: String, func: FuncSig) {
        self.funcs.insert(named, func);
    }

    /// Registers an intrinsic type
    pub fn add_type(&mut self, named: String) {
        self.types.insert(named);
    }

    /// Prints all the intrinsic types
    pub fn print(&self) {
        println!("--- Types ---");
        for ty in &self.types {
            println!("{ty}");
        }
        println!();

        println!("--- Functions ---");
        for f in &self.funcs {
            println!("func {}{}", f.0, f.1);
        }
    }
}

impl Scope for Intrinsics {
    fn parent(&self) -> Option<&dyn Scope> {
        None
    }

    fn name(&self) -> &str {
        "intrinsics"
    }

    fn kind(&self) -> ScopeKind {
        ScopeKind::Library
    }

    fn lookup_symbol(&self, name: &String) -> Option<Symbol> {
        if self.types.contains(name) {
            return Some(Symbol::new(SymbolKind::Type(Type::new_anon(TypeKind::Intrinsic(name.clone()))), Visibility::Public))
        }

        if let Some(func) = self.funcs.get(name) {
            let expr_type = Type::new_anon(TypeKind::Func(Box::new(func.clone())));
            let func_value = Expr::new_anon(ExprKind::IntrinsicFunc(name.clone()), expr_type);

            return Some(Symbol::new(SymbolKind::Function(func_value), Visibility::Public));
        }

        return None
    }

    fn define_expr(&self, name: String, value: Expr) {
        todo!()
    }

    fn scoped_type(&self, name: &str) -> Option<TypeKind> {
        None
    }

    fn take_index(&self) -> u64 {
        0
    }
}

/*
add_integer_type (bits: usize)

let t = intrinsics.add_type(format!("i{bits}"));

let arith = sig!( (t, t): t );

integer{bits}Add, arith.clone()
integer{bits}Sub, arith.clone()
integer{bits}SubSig, arith.clone()
integer{bits}Mul, arith.clone()
integer{bits}MulSig, arith.clone()
integer{bits}


integer64_add: (i64, i64): i64
integer64_sub: (i64, i64): i64
integer64_mul: (i64, i64): i64
integer64_div: (i64, i64): i64
integer64_mod: (i64, i64): i64

*/