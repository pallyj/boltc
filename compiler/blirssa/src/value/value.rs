use std::fmt::Display;

use super::{BinaryIntrinsicFn, LabelValue, UnaryIntrinsicFn};
use crate::{code::{ExternFunctionRef, FunctionRef},
            typ::Type};

pub enum Value {
    IntegerLiteral {
        typ:   Type,
        value: u64,
    },
    FloatLiteral {
        typ:   Type,
        value: f64,
    },
    GlobalString {
        typ:   Type,
        value: String,
    },

    UnaryIntrinsic {
        name:        UnaryIntrinsicFn,
        arg:         LabelValue,
        return_type: Type,
    },

    BinaryIntrinsic {
        name:        BinaryIntrinsicFn,
        left:        LabelValue,
        right:       LabelValue,
        return_type: Type,
    },

    AllocOnStackUndef {
        typ: Type,
    },

    AllocOnStack {
        value: LabelValue,
        typ:   Type,
    },

    Deref {
        pointer: LabelValue,
        typ:     Type,
    },

    /// deref-struct-field "field_name" (struct) : (type)
    /// Dereferences a field of a struct
    /// The value passed to this instruction can be either a pointer to a struct or the struct itself
    DerefStructField {
        r#struct: LabelValue,
        field:    String,
        typ:      Type,
    },

    /// access-struct-field "field_name" (struct) : (type)
    /// Returns a pointer to a field of a struct
    /// The value passed to this instruction must be a pointer to a struct
    AccessStructField {
        r#struct: LabelValue,
        field:    String,
        typ:      Type,
    },

    DerefTupleField {
        tuple: LabelValue,
        field: usize,
        typ:   Type,
    },

    AccessTupleField {
        tuple: LabelValue,
        field: usize,
        typ:   Type,
    },

    Function {
        function: FunctionRef,
    },

    ExternFunction {
        function: ExternFunctionRef,
    },

    BuildFunctionPointer {
        function:  LabelValue,
        func_type: Type,
    },

    Call {
        function: LabelValue,
        args:     Vec<LabelValue>,
        typ:      Type,
    },

    CreateEnumVariant {
        variant: String,
        typ: Type,
    },

    
}

impl Value {
    pub fn typ(&self) -> Type {
        match self {
            Self::IntegerLiteral { typ, .. } => typ,
            Self::FloatLiteral { typ, .. } => typ,
            Self::GlobalString { typ, .. } => typ,

            Self::UnaryIntrinsic { return_type, .. } => return_type,
            Self::BinaryIntrinsic { return_type, .. } => return_type,

            Self::Function { function } => return function.typ(),
            Self::ExternFunction { function } => return function.typ(),
            Self::BuildFunctionPointer { func_type, .. } => return Type::Pointer { pointee: Box::new(func_type.clone()), },
            Self::Call { typ, .. } => typ,

            Self::AllocOnStackUndef { typ, .. } => typ,
            Self::AllocOnStack { typ, .. } => typ,
            Self::Deref { typ, .. } => typ,

            Self::AccessStructField { typ, .. } => typ,
            Self::DerefStructField { typ, .. } => typ,

            Self::AccessTupleField { typ, .. } => typ,
            Self::DerefTupleField { typ, .. } => typ,

            Self::CreateEnumVariant { typ, .. } => typ,
        }.clone()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::IntegerLiteral { typ, value } => write!(f, "integer-literal {value} : {typ}"),
            Value::FloatLiteral { typ, value } => write!(f, "float-literal {value} : {typ}"),
            Value::GlobalString { typ, value } => write!(f, "global-string \"{value}\" : {typ}"),

            Value::UnaryIntrinsic { name, arg, return_type } => write!(f,
                                                                       "intrinsic \"{name}\" ( {arg} ) : {return_type}",
                                                                       name = name.name()),
            Value::BinaryIntrinsic { name,
                                     left,
                                     right,
                                     return_type, } => write!(f,
                                                              "intrinsic \"{name}\" ( {left}, {right} ) : {return_type}",
                                                              name = name.name()),

            Value::AllocOnStackUndef { typ } => write!(f, "alloc-on-stack : {typ}"),
            Value::AllocOnStack { value, typ } => write!(f, "alloc-on-stack {value} : {typ}"),
            Value::Deref { pointer, typ } => write!(f, "deref {pointer} : {typ}"),

            Value::Function { function } => write!(f,
                                                   "function \"{name}\" : {typ}",
                                                   name = function.name(),
                                                   typ = function.typ()),
            Value::ExternFunction { function } => write!(f,
                                                         "function \"{name}\" : {typ}",
                                                         name = function.name(),
                                                         typ = function.typ()),
            Value::BuildFunctionPointer { function, func_type } => write!(f, "build-function-ptr {function} : &{func_type}"),
            Value::Call { function, args, typ } => {
                let args = args.iter()
                               .map(|arg| arg.to_string())
                               .collect::<Vec<_>>()
                               .join(", ");

                write!(f, "call {function} ({args}) : {typ}")
            }

            Value::AccessStructField { r#struct, field, typ } => write!(f, "access-struct-field \"{field}\" {struct} : {typ}"),
            Value::DerefStructField { r#struct, field, typ } => write!(f, "deref-struct-field \"{field}\" {struct} : {typ}"),

            Value::AccessTupleField { tuple, field, typ } => write!(f, "access-tuple-field \"{field}\" {tuple} : {typ}"),
            Value::DerefTupleField { tuple, field, typ } => write!(f, "deref-tuple-field \"{field}\" {tuple} : {typ}"),

            Value::CreateEnumVariant { variant, typ } => write!(f, "create-enum-variant .{variant} : {typ}"),
        }
    }
}
