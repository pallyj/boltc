use std::fmt::Display;

use crate::ty::Type;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SoloIntrinsic {
	INeg, IInv,

	IZext16, IZext32, IZext64,
	ISext16, ISext32, ISext64,

	ITrunc8, ITrunc16, ITrunc32,

	ICnvF16, ICnvF32, ICnvF64,
	ICnvF16Sig, ICnvF32Sig, ICnvF64Sig,

	FNeg,

	FExt32, FExt64,

	FTrunc16, FTrunc32,

	FCnvI,

    AddrCnvPtr,
    PtrCnvAddr,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DuoIntrinsic {
	IAdd, ISub,
	IMul, IDiv, IDivSig,
	IRem, IRemSig,

	IAnd, IOr, IXor,

	IShl, IShr, IShrSig,

	ICmpEq, ICmpNeq,
	ICmpLt, ICmpLte,
	ICmpGt, ICmpGte,
	ICmpLtSig, ICmpLteSig,
	ICmpGtSig, ICmpGteSig,

	FAdd, FSub,
	FMul, FDiv, FRem,

	FCmpEq, FCmpNeq,
	FCmpLt, FCmpLte,
	FCmpGt, FCmpGte,

    AItem, PtrAdd
}

impl SoloIntrinsic {
	pub fn output_type(self, input_type: &Type) -> Type {
		use SoloIntrinsic::*;

		match self {
			 ITrunc8 => Type::int(8),
			 IZext16 | ISext16 | ITrunc16 => Type::int(16),
			 IZext32 | ISext32 | ITrunc32 => Type::int(32),
			 IZext64 | ISext64 | FCnvI => Type::int(64),

			 ICnvF16 | ICnvF16Sig | FTrunc16 => Type::float(16),
			 ICnvF32 | ICnvF32Sig | FExt32 | FTrunc32 => Type::float(32),
			 ICnvF64 | ICnvF64Sig | FExt64 => Type::float(64),

			 _ => input_type.clone(),
		}
	}
}

impl DuoIntrinsic {
	pub fn output_type(self, input_type: &Type) -> Type {
		use DuoIntrinsic::*;

		match self {
			ICmpEq | ICmpNeq |
			ICmpLt | ICmpLte |
			ICmpGt | ICmpGte |
			ICmpLtSig | ICmpLteSig |
			ICmpGtSig | ICmpGteSig |

			FCmpEq | FCmpNeq |
			FCmpLt | FCmpLte |
			FCmpGt | FCmpGte => Type::int(1),

			_ => input_type.clone()
		}
	}
}

impl Display for SoloIntrinsic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoloIntrinsic::INeg => write!(f, "int.neg"),
            SoloIntrinsic::IInv => write!(f, "int.inv"),
            SoloIntrinsic::IZext16 => write!(f, "int.zext.16"),
            SoloIntrinsic::IZext32 => write!(f, "int.zext.32"),
            SoloIntrinsic::IZext64 => write!(f, "int.zext.64"),
            SoloIntrinsic::ISext16 => write!(f, "int.sext.16"),
            SoloIntrinsic::ISext32 => write!(f, "int.sext.32"),
            SoloIntrinsic::ISext64 => write!(f, "int.sext.64"),
            SoloIntrinsic::ITrunc8 => write!(f, "int.trunc.8"),
            SoloIntrinsic::ITrunc16 => write!(f, "int.trunc.16"),
            SoloIntrinsic::ITrunc32 => write!(f, "int.trunc.32"),
            SoloIntrinsic::ICnvF16 => write!(f, "int.fconv.16"),
            SoloIntrinsic::ICnvF32 => write!(f, "int.fconv.32"),
            SoloIntrinsic::ICnvF64 => write!(f, "int.fconv.64"),
            SoloIntrinsic::ICnvF16Sig => write!(f, "int.fconv.sig.16"),
            SoloIntrinsic::ICnvF32Sig => write!(f, "int.fconv.sig.32"),
            SoloIntrinsic::ICnvF64Sig => write!(f, "int.fconv.sig.64"),
            SoloIntrinsic::FNeg => write!(f, "float.neg"),
            SoloIntrinsic::FExt32 => write!(f, "float.ext.32"),
            SoloIntrinsic::FExt64 => write!(f, "float.ext.64"),
            SoloIntrinsic::FTrunc16 => write!(f, "float.trunc.16"),
            SoloIntrinsic::FTrunc32 => write!(f, "float.trunc.32"),
            SoloIntrinsic::FCnvI => write!(f, "float.icnv.64"),
            SoloIntrinsic::AddrCnvPtr => write!(f, "addr.ptrconv"),
            SoloIntrinsic::PtrCnvAddr => write!(f, "ptr.addrconv"),
        }
    }
}

impl Display for DuoIntrinsic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DuoIntrinsic::IAdd => write!(f, "int.add"),
            DuoIntrinsic::ISub => write!(f, "int.sub"),
            DuoIntrinsic::IMul => write!(f, "int.mul"),
            DuoIntrinsic::IDiv => write!(f, "int.div"),
			DuoIntrinsic::IDivSig => write!(f, "int.div.sig"),
            DuoIntrinsic::IRem => write!(f, "int.rem"),
            DuoIntrinsic::IRemSig => write!(f, "int.rem.sig"),
            DuoIntrinsic::IAnd => write!(f, "int.and"),
            DuoIntrinsic::IOr => write!(f, "int.or"),
            DuoIntrinsic::IXor => write!(f, "int.xor"),
            DuoIntrinsic::IShl => write!(f, "int.shl"),
            DuoIntrinsic::IShr => write!(f, "int.shr"),
            DuoIntrinsic::IShrSig => write!(f, "int.shr.sig"),
            DuoIntrinsic::ICmpEq => write!(f, "int.cmp.eq"),
            DuoIntrinsic::ICmpNeq => write!(f, "int.cmp.neq"),
            DuoIntrinsic::ICmpLt => write!(f, "int.cmp.lt"),
            DuoIntrinsic::ICmpLte => write!(f, "int.cmp.lte"),
            DuoIntrinsic::ICmpGt => write!(f, "int.cmp.gt"),
            DuoIntrinsic::ICmpGte => write!(f, "int.cmp.gte"),
            DuoIntrinsic::ICmpLtSig => write!(f, "int.cmp.lt.sig"),
            DuoIntrinsic::ICmpLteSig => write!(f, "int.cmp.lte.sig"),
            DuoIntrinsic::ICmpGtSig => write!(f, "int.cmp.gt.sig"),
            DuoIntrinsic::ICmpGteSig => write!(f, "int.cmp.gte.sig"),
            DuoIntrinsic::FAdd => write!(f, "float.add"),
            DuoIntrinsic::FSub => write!(f, "float.sub"),
            DuoIntrinsic::FMul => write!(f, "float.mul"),
            DuoIntrinsic::FDiv => write!(f, "float.div"),
            DuoIntrinsic::FRem => write!(f, "float.rem"),
            DuoIntrinsic::FCmpEq => write!(f, "float.cmp.eq"),
            DuoIntrinsic::FCmpNeq => write!(f, "float.cmp.neq"),
            DuoIntrinsic::FCmpLt => write!(f, "float.cmp.lt"),
            DuoIntrinsic::FCmpLte => write!(f, "float.cmp.lte"),
            DuoIntrinsic::FCmpGt => write!(f, "float.cmp.gt"),
            DuoIntrinsic::FCmpGte => write!(f, "float.cmp.gte"),
            DuoIntrinsic::AItem => write!(f, "array.item"),
            DuoIntrinsic::PtrAdd => write!(f, "ptr.add"),
        }
    }
}


/*
pub struct StringSlice {
	ptr: constptr UInt8,
	len: UInt,
}

constptr
varptr
*/