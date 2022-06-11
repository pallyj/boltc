use crate::{scope::{ScopeRef, ScopeRelation, ScopeType},
            typ::TypeKind,
            value::ValueKind,
            Symbol, Visibility};

#[derive(Debug, Copy, Clone)]
pub enum UnaryIntrinsicFn {
    IntegerNegate,
    IntegerInvert,

    IntegerExtZero16,
    IntegerExtZero32,
    IntegerExtZero64,

    IntegerExtSig16,
    IntegerExtSig32,
    IntegerExtSig64,

    IntegerTrunc8,
    IntegerTrunc16,
    IntegerTrunc32,

    FloatNegate,
    FloatTrunc16,
    FloatTrunc32,
    FloatExt32,
    FloatExt64,

    IntegerFromFloat,
    IntegerFromFloatSig,
    Float16FromInteger,
    Float32FromInteger,
    Float64FromInteger,
    Float16FromIntegerSig,
    Float32FromIntegerSig,
    Float64FromIntegerSig,

    StrSliceLen,

    RawPointerDeref,
    RawPointerRef,
    RawPointerToAddr,
    RawPointerFromAddr,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryIntrinsicFn {
    IntegerAdd,
    IntegerSub,
    IntegerMul,
    IntegerDiv,
    IntegerRem,
    IntegerDivSig,
    IntegerRemSig,

    IntegerOr,
    IntegerXor,
    IntegerAnd,

    IntegerShl,
    IntegerShr,
    IntegerShrSig,

    IntegerCmpEq,
    IntegerCmpNeq,
    IntegerCmpLt,
    IntegerCmpGt,
    IntegerCmpLte,
    IntegerCmpGte,
    IntegerCmpLtSig,
    IntegerCmpGtSig,
    IntegerCmpLteSig,
    IntegerCmpGteSig,

    FloatAdd,
    FloatSub,
    FloatMul,
    FloatDiv,
    FloatRem,

    FloatCmpEq,
    FloatCmpNeq,
    FloatCmpGt,
    FloatCmpGte,
    FloatCmpLt,
    FloatCmpLte,

    RawPointerAdd,
}

pub struct Intrinsics {
    scope: ScopeRef,
}

impl Intrinsics {
    pub fn new() -> Intrinsics { Intrinsics { scope: ScopeRef::new(None, ScopeRelation::None, ScopeType::Library, false, false), } }

    pub fn populate(&self) {
        self.add_integer_type(8);
        self.add_integer_type(16);
        self.add_integer_type(32);
        self.add_integer_type(64);

        self.add_float_type(16);
        self.add_float_type(32);
        self.add_float_type(64);

        self.add_bool_type();
        //self.add_strslice_type();

        self.add_rawptr_type();
    }

    pub fn add_rawptr_type(&self) {
        let hk_rawptr = TypeKind::HKRawPointer;

        self.scope.add_symbol(String::from("RawPointer"),
                              Visibility::Public,
                              Symbol::Type(hk_rawptr.clone()));

        // rawPointerCopy
        // rawPointerDeref
        // rawPointerRef
        // rawPointerAdd
        // rawPointerToAddr
        // rawPointerFromAddr

        let generic_param = TypeKind::GenericParam(String::from("T")).anon();
        let generic_pointer = TypeKind::RawPointer { pointer_type: Box::new(generic_param.clone()) }.anon();
        let address = TypeKind::Integer { bits: 64 }.anon();

        self.add_unary_func_generic(String::from("rawPointerDeref"), UnaryIntrinsicFn::RawPointerDeref, &generic_pointer, &generic_param);
        self.add_unary_func_generic(String::from("rawPointerRef"), UnaryIntrinsicFn::RawPointerRef, &generic_param, &generic_pointer);
        self.add_unary_func_generic(String::from("rawPointerToAddr"), UnaryIntrinsicFn::RawPointerToAddr, &generic_pointer, &address);
        self.add_unary_func_generic(String::from("rawPointerFromAddr"), UnaryIntrinsicFn::RawPointerFromAddr, &address, &generic_pointer);
        
        let func_ty = TypeKind::Function { return_type: Box::new(generic_pointer.clone()),
                                           params:      vec![generic_pointer.clone(), address.clone()],
                                           labels:      vec![None, None], }.anon();

        let func = ValueKind::BinaryIntrinsicFn(BinaryIntrinsicFn::RawPointerAdd).anon(func_ty);
        let genericized_func = func.monomorph_infer(vec![String::from("T")]);

        let sym = Symbol::Value(genericized_func);

        self.scope.add_symbol(String::from("rawPointerAdd"), Visibility::Public, sym);
    }

    pub fn add_strslice_type(&self) {
        let strslice = TypeKind::StrSlice;
        let length = TypeKind::Integer { bits: 64 };

        self.scope.add_symbol("strslice".to_string(),
                              Visibility::Public,
                              Symbol::Type(strslice.clone()));

        // strslice_len
        self.add_unary_func("strslice_len".to_string(), UnaryIntrinsicFn::StrSliceLen, &strslice, &length);
    }

    /// Adds a float type with arithmatic
    pub fn add_float_type(&self, bits: u64) {
        let t = TypeKind::Float { bits };
        let b = TypeKind::Integer { bits: 1 };

        self.scope.add_symbol(format!("f{bits}"),
                              Visibility::Public,
                              Symbol::Type(t.clone()));

        // Arithmatic
        self.add_binary_func(format!("float{bits}Add"),
                             BinaryIntrinsicFn::FloatAdd,
                             &t,
                             &t);
        self.add_binary_func(format!("float{bits}Sub"),
                             BinaryIntrinsicFn::FloatSub,
                             &t,
                             &t);
        self.add_binary_func(format!("float{bits}Mul"),
                             BinaryIntrinsicFn::FloatMul,
                             &t,
                             &t);
        self.add_binary_func(format!("float{bits}Div"),
                             BinaryIntrinsicFn::FloatDiv,
                             &t,
                             &t);
        self.add_binary_func(format!("float{bits}Rem"),
                             BinaryIntrinsicFn::FloatRem,
                             &t,
                             &t);

        // Comparison
        self.add_binary_func(format!("float{bits}CmpEq"),
                             BinaryIntrinsicFn::FloatCmpEq,
                             &t,
                             &b);
        self.add_binary_func(format!("float{bits}CmpNeq"),
                             BinaryIntrinsicFn::FloatCmpNeq,
                             &t,
                             &b);
        self.add_binary_func(format!("float{bits}CmpLt"),
                             BinaryIntrinsicFn::FloatCmpLt,
                             &t,
                             &b);
        self.add_binary_func(format!("float{bits}CmpGt"),
                             BinaryIntrinsicFn::FloatCmpGt,
                             &t,
                             &b);
        self.add_binary_func(format!("float{bits}CmpLte"),
                             BinaryIntrinsicFn::FloatCmpLte,
                             &t,
                             &b);
        self.add_binary_func(format!("float{bits}CmpGte"),
                             BinaryIntrinsicFn::FloatCmpGte,
                             &t,
                             &b);

        // Unary
        self.add_unary_func(format!("float{bits}Negate"),
                            UnaryIntrinsicFn::FloatNegate,
                            &t,
                            &t);

        // Extension
        let mut b = bits * 2;

        while b <= 64 {
            let o = TypeKind::Float { bits: b };

            let u = match b {
                32 => UnaryIntrinsicFn::FloatExt32,
                64 => UnaryIntrinsicFn::FloatExt64,
                _ => return,
            };

            self.add_unary_func(format!("float{bits}Ext{b}"), u, &t, &o);

            b *= 2;
        }

        // Truncation
        let mut b = bits / 2;

        while b >= 16 {
            let o = TypeKind::Float { bits: b };

            let u = match b {
                16 => UnaryIntrinsicFn::FloatTrunc16,
                32 => UnaryIntrinsicFn::FloatTrunc32,
                _ => return,
            };

            self.add_unary_func(format!("float{bits}Trunc{b}"), u, &t, &o);

            b /= 2;
        }

        // To int
        let i = TypeKind::Integer { bits: 64 };

        self.add_unary_func(format!("float{bits}ToInt"),
                            UnaryIntrinsicFn::IntegerFromFloat,
                            &t,
                            &i);
        self.add_unary_func(format!("float{bits}ToIntSig"),
                            UnaryIntrinsicFn::IntegerFromFloatSig,
                            &t,
                            &i);

        let (u_z, u_s) = match bits {
            16 => (UnaryIntrinsicFn::Float16FromInteger, UnaryIntrinsicFn::Float16FromIntegerSig),
            32 => (UnaryIntrinsicFn::Float32FromInteger, UnaryIntrinsicFn::Float32FromIntegerSig),
            64 => (UnaryIntrinsicFn::Float64FromInteger, UnaryIntrinsicFn::Float64FromIntegerSig),
            _ => return,
        };

        self.add_unary_func(format!("float{bits}FromInt"), u_z, &i, &t);
        self.add_unary_func(format!("float{bits}FromIntSig"), u_s, &i, &t);
    }

    /// Adds a boolean type
    pub fn add_bool_type(&self) {
        let b = TypeKind::Integer { bits: 1 };

        self.scope.add_symbol("i1".to_string(),
                              Visibility::Public,
                              Symbol::Type(b.clone()));

        self.add_binary_func("integer1And".to_string(),
                             BinaryIntrinsicFn::IntegerAnd,
                             &b,
                             &b);
        self.add_binary_func("integer1Xor".to_string(),
                             BinaryIntrinsicFn::IntegerXor,
                             &b,
                             &b);
        self.add_binary_func("integer1Or".to_string(),
                             BinaryIntrinsicFn::IntegerOr,
                             &b,
                             &b);

        self.add_binary_func("integer1CmpEq".to_string(),
                             BinaryIntrinsicFn::IntegerCmpEq,
                             &b,
                             &b);
        self.add_binary_func("integer1CmpNeq".to_string(),
                             BinaryIntrinsicFn::IntegerCmpNeq,
                             &b,
                             &b);

        self.add_unary_func("integer1Invert".to_string(),
                            UnaryIntrinsicFn::IntegerInvert,
                            &b,
                            &b);
    }

    /// Adds an integer type, with arithmatic, binary, comparison, and conversion operations
    pub fn add_integer_type(&self, bits: u64) {
        let t = TypeKind::Integer { bits };
        let b = TypeKind::Integer { bits: 1 };

        self.scope.add_symbol(format!("i{bits}"),
                              Visibility::Public,
                              Symbol::Type(t.clone()));

        // Arithmatic
        self.add_binary_func(format!("integer{bits}Add"),
                             BinaryIntrinsicFn::IntegerAdd,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}Sub"),
                             BinaryIntrinsicFn::IntegerSub,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}Mul"),
                             BinaryIntrinsicFn::IntegerMul,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}Div"),
                             BinaryIntrinsicFn::IntegerDiv,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}Rem"),
                             BinaryIntrinsicFn::IntegerRem,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}DivSig"),
                             BinaryIntrinsicFn::IntegerDivSig,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}RemSig"),
                             BinaryIntrinsicFn::IntegerRemSig,
                             &t,
                             &t);

        // Binary
        self.add_binary_func(format!("integer{bits}Or"),
                             BinaryIntrinsicFn::IntegerOr,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}Xor"),
                             BinaryIntrinsicFn::IntegerXor,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}And"),
                             BinaryIntrinsicFn::IntegerAnd,
                             &t,
                             &t);

        // Binary
        self.add_binary_func(format!("integer{bits}Shl"),
                             BinaryIntrinsicFn::IntegerShl,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}Shr"),
                             BinaryIntrinsicFn::IntegerShr,
                             &t,
                             &t);
        self.add_binary_func(format!("integer{bits}ShrSig"),
                             BinaryIntrinsicFn::IntegerShrSig,
                             &t,
                             &t);

        // Comparison
        self.add_binary_func(format!("integer{bits}CmpEq"),
                             BinaryIntrinsicFn::IntegerCmpEq,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpNeq"),
                             BinaryIntrinsicFn::IntegerCmpNeq,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpLt"),
                             BinaryIntrinsicFn::IntegerCmpLt,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpGt"),
                             BinaryIntrinsicFn::IntegerCmpGt,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpLte"),
                             BinaryIntrinsicFn::IntegerCmpLte,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpGte"),
                             BinaryIntrinsicFn::IntegerCmpGte,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpLtSig"),
                             BinaryIntrinsicFn::IntegerCmpLtSig,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpGtSig"),
                             BinaryIntrinsicFn::IntegerCmpGtSig,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpLteSig"),
                             BinaryIntrinsicFn::IntegerCmpLteSig,
                             &t,
                             &b);
        self.add_binary_func(format!("integer{bits}CmpGteSig"),
                             BinaryIntrinsicFn::IntegerCmpGteSig,
                             &t,
                             &b);

        // Unary
        self.add_unary_func(format!("integer{bits}Negate"),
                            UnaryIntrinsicFn::IntegerNegate,
                            &t,
                            &t);
        self.add_unary_func(format!("integer{bits}Invert"),
                            UnaryIntrinsicFn::IntegerInvert,
                            &t,
                            &t);

        // Extension
        let mut b = bits * 2;

        while b <= 64 {
            let o = TypeKind::Integer { bits: b };

            let u_z = match b {
                16 => UnaryIntrinsicFn::IntegerExtZero16,
                32 => UnaryIntrinsicFn::IntegerExtZero32,
                64 => UnaryIntrinsicFn::IntegerExtZero64,
                _ => return,
            };

            let u_s = match b {
                16 => UnaryIntrinsicFn::IntegerExtSig16,
                32 => UnaryIntrinsicFn::IntegerExtSig32,
                64 => UnaryIntrinsicFn::IntegerExtSig64,
                _ => return,
            };

            self.add_unary_func(format!("integer{bits}ExtZero{b}"), u_z, &t, &o);
            self.add_unary_func(format!("integer{bits}ExtSig{b}"), u_s, &t, &o);

            b *= 2;
        }

        // Truncation
        let mut b = bits / 2;

        while b >= 8 {
            let o = TypeKind::Integer { bits: b };

            let u = match b {
                8 => UnaryIntrinsicFn::IntegerTrunc8,
                16 => UnaryIntrinsicFn::IntegerTrunc16,
                32 => UnaryIntrinsicFn::IntegerTrunc32,
                _ => return,
            };

            self.add_unary_func(format!("integer{bits}Trunc{b}"), u, &t, &o);

            b /= 2;
        }
    }

    pub fn add_binary_func(&self, named: String, intrinsic: BinaryIntrinsicFn, param: &TypeKind, return_ty: &TypeKind) {
        let func_ty = TypeKind::Function { return_type: Box::new(return_ty.clone().anon()),
                                           params:      vec![param.clone().anon(), param.clone().anon()],
                                           labels:      vec![], }.anon();

        let func = ValueKind::BinaryIntrinsicFn(intrinsic).anon(func_ty);

        let sym = Symbol::Value(func);

        self.scope.add_symbol(named, Visibility::Public, sym);
    }

    pub fn add_unary_func(&self, named: String, intrinsic: UnaryIntrinsicFn, param: &TypeKind, return_ty: &TypeKind) {
        let func_ty = TypeKind::Function { return_type: Box::new(return_ty.clone().anon()),
                                           params:      vec![param.clone().anon()],
                                           labels:      vec![], }.anon();

        let func = ValueKind::UnaryIntrinsicFn(intrinsic).anon(func_ty);

        let sym = Symbol::Value(func);

        self.scope.add_symbol(named, Visibility::Public, sym);
    }

    pub fn add_unary_func_generic(&self, named: String, intrinsic: UnaryIntrinsicFn, param: &TypeKind, return_ty: &TypeKind) {
        let func_ty = TypeKind::Function { return_type: Box::new(return_ty.clone().anon()),
                                           params:      vec![param.clone().anon()],
                                           labels:      vec![None], }.anon();

        let func = ValueKind::UnaryIntrinsicFn(intrinsic).anon(func_ty);
        let generic_function = func.monomorph_infer(vec![String::from("T")]);

        let sym = Symbol::Value(generic_function);

        self.scope.add_symbol(named, Visibility::Public, sym);
    }

    pub fn scope(&self) -> ScopeRef { self.scope.clone() }
}
