#[derive(Clone, Copy)]
pub enum BinaryIntrinsicFn {
    IntegerAdd,
    IntegerSub,
    IntegerMul,
    IntegerDiv,
    IntegerRem,
    IntegerDivSig,
    IntegerRemSig,

    IntegerAnd,
    IntegerXor,
    IntegerOr,

    IntegerShl,
    IntegerShr,
    IntegerShrSig,

    IntegerCmpEq,
    IntegerCmpNeq,
    IntegerCmpLt,
    IntegerCmpLte,
    IntegerCmpGt,
    IntegerCmpGte,
    IntegerCmpLtSig,
    IntegerCmpLteSig,
    IntegerCmpGtSig,
    IntegerCmpGteSig,

    FloatAdd,
    FloatSub,
    FloatMul,
    FloatDiv,
    FloatRem,

    FloatCmpEq,
    FloatCmpNeq,
    FloatCmpLt,
    FloatCmpLte,
    FloatCmpGt,
    FloatCmpGte,
}

impl BinaryIntrinsicFn {
    pub fn output_type(&self) -> IntrinsicFnOutput {
        match self {
            Self::IntegerAdd
            | Self::IntegerSub
            | Self::IntegerMul
            | Self::IntegerDiv
            | Self::IntegerRem
            | Self::IntegerDivSig
            | Self::IntegerRemSig
            | Self::IntegerXor
            | Self::IntegerAnd
            | Self::IntegerOr
            | Self::IntegerShl
            | Self::IntegerShr
            | Self::IntegerShrSig
            | Self::FloatAdd
            | Self::FloatSub
            | Self::FloatMul
            | Self::FloatDiv
            | Self::FloatRem => IntrinsicFnOutput::Same,

            Self::IntegerCmpEq
            | Self::IntegerCmpNeq
            | Self::IntegerCmpLt
            | Self::IntegerCmpLte
            | Self::IntegerCmpGt
            | Self::IntegerCmpGte
            | Self::IntegerCmpLtSig
            | Self::IntegerCmpLteSig
            | Self::IntegerCmpGtSig
            | Self::IntegerCmpGteSig
            | Self::FloatCmpEq
            | Self::FloatCmpNeq
            | Self::FloatCmpLt
            | Self::FloatCmpLte
            | Self::FloatCmpGt
            | Self::FloatCmpGte => IntrinsicFnOutput::Boolean,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::IntegerAdd => "integer.add",
            Self::IntegerSub => "integer.sub",
            Self::IntegerMul => "integer.mul",
            Self::IntegerDiv => "integer.div",
            Self::IntegerRem => "integer.rem",
            Self::IntegerDivSig => "integer.div.sig",
            Self::IntegerRemSig => "integer.rem.sig",
            Self::IntegerXor => "integer.xor",
            Self::IntegerAnd => "integer.and",
            Self::IntegerOr => "integer.or",
            Self::IntegerShl => "integer.shl",
            Self::IntegerShr => "integer.shr",
            Self::IntegerShrSig => "integer.shr.sig",
            Self::FloatAdd => "float.add",
            Self::FloatSub => "float.sub",
            Self::FloatMul => "float.mul",
            Self::FloatDiv => "float.div",
            Self::FloatRem => "float.rem",

            Self::IntegerCmpEq => "integer.cmp.eq",
            Self::IntegerCmpNeq => "integer.cmp.neq",
            Self::IntegerCmpLt => "integer.cmp.lt",
            Self::IntegerCmpLte => "integer.cmp.lte",
            Self::IntegerCmpGt => "integer.cmp.gt",
            Self::IntegerCmpGte => "integer.cmp.gte",
            Self::IntegerCmpLtSig => "integer.cmp.lt.sig",
            Self::IntegerCmpLteSig => "integer.cmp.lte.sig",
            Self::IntegerCmpGtSig => "integer.cmp.gt.sig",
            Self::IntegerCmpGteSig => "integer.cmp.gte.sig",
            Self::FloatCmpEq => "float.cmp.eq",
            Self::FloatCmpNeq => "float.cmp.neq",
            Self::FloatCmpLt => "float.cmp.lt",
            Self::FloatCmpLte => "float.cmp.lte",
            Self::FloatCmpGt => "float.cmp.gt",
            Self::FloatCmpGte => "float.cmp.gte",
        }
    }
}

#[derive(Clone, Copy)]
pub enum UnaryIntrinsicFn {
    IntegerNegate,
    IntegerInvert,

    IntegerExt64,
    IntegerExt32,
    IntegerExt16,
    IntegerExt64Sig,
    IntegerExt32Sig,
    IntegerExt16Sig,

    IntegerTrunc32,
    IntegerTrunc16,
    IntegerTrunc8,

    IntegerToFloat16,
    IntegerToFloat32,
    IntegerToFloat64,
    IntegerToFloat16Sig,
    IntegerToFloat32Sig,
    IntegerToFloat64Sig,

    FloatNegate,

    FloatExt64,
    FloatExt32,

    FloatTrunc32,
    FloatTrunc16,

    FloatToInt,
    FloatToIntSig,
}

impl UnaryIntrinsicFn {
    pub fn output_type(&self) -> IntrinsicFnOutput {
        match self {
            Self::IntegerNegate | Self::IntegerInvert | Self::FloatNegate => IntrinsicFnOutput::Same,

            Self::IntegerExt64 | Self::IntegerExt64Sig => IntrinsicFnOutput::Integer(64),
            Self::IntegerExt32 | Self::IntegerExt32Sig | Self::IntegerTrunc32 => IntrinsicFnOutput::Integer(32),
            Self::IntegerExt16 | Self::IntegerExt16Sig | Self::IntegerTrunc16 => IntrinsicFnOutput::Integer(16),
            Self::IntegerTrunc8 => IntrinsicFnOutput::Integer(8),

            Self::IntegerToFloat16 | Self::IntegerToFloat16Sig => IntrinsicFnOutput::Float(16),
            Self::IntegerToFloat32 | Self::IntegerToFloat32Sig => IntrinsicFnOutput::Float(32),
            Self::IntegerToFloat64 | Self::IntegerToFloat64Sig => IntrinsicFnOutput::Float(64),

            Self::FloatExt64 => IntrinsicFnOutput::Float(64),
            Self::FloatExt32 | Self::FloatTrunc32 => IntrinsicFnOutput::Float(32),
            Self::FloatTrunc16 => IntrinsicFnOutput::Float(16),

            Self::FloatToInt | Self::FloatToIntSig => IntrinsicFnOutput::Integer(64),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::IntegerNegate => "integer.negate",
            Self::IntegerInvert => "integer.invert",
            Self::FloatNegate => "float.negate",

            Self::IntegerExt64 => "integer.ext.64",
            Self::IntegerExt32 => "integer.ext.32",
            Self::IntegerExt16 => "integer.ext.16",

            Self::IntegerExt64Sig => "integer.ext.64.sig",
            Self::IntegerExt32Sig => "integer.ext.32.sig",
            Self::IntegerExt16Sig => "integer.ext.16.sig",

            Self::IntegerTrunc32 => "integer.trunc.32",
            Self::IntegerTrunc16 => "integer.trunc.16",
            Self::IntegerTrunc8 => "integer.trunc.8",

            Self::IntegerToFloat16 => "integer.tofloat.16",
            Self::IntegerToFloat16Sig => "integer.tofloat.16.sig",
            Self::IntegerToFloat32 => "integer.tofloat.32",
            Self::IntegerToFloat32Sig => "integer.tofloat.32.sig",
            Self::IntegerToFloat64 => "integer.tofloat.64",
            Self::IntegerToFloat64Sig => "integer.tofloat.64.sig",

            Self::FloatExt64 => "float.ext.64",
            Self::FloatExt32 => "float.ext.32",
            Self::FloatTrunc32 => "float.trunc.32",
            Self::FloatTrunc16 => "float.trunc.16",

            Self::FloatToInt => "float.toint",
            Self::FloatToIntSig => "float.toint.sig",
        }
    }
}

pub enum IntrinsicFnOutput {
    Boolean,
    Same,
    Integer(u32),
    Float(u32),
}
