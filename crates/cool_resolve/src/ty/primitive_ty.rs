use crate::{BasicTyDef, TyDef};
use derive_more::Display;

#[derive(Clone, Copy, Debug)]
pub struct PrimitiveTyData {
    pub i8_align: u64,
    pub i16_align: u64,
    pub i32_align: u64,
    pub i64_align: u64,
    pub i128_align: u64,
    pub ptr_size: u64,
    pub ptr_align: u64,
    pub f32_align: u64,
    pub f64_align: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
pub enum IntTy {
    #[display(fmt = "i8")]
    I8,

    #[display(fmt = "i16")]
    I16,

    #[display(fmt = "i32")]
    I32,

    #[display(fmt = "i64")]
    I64,

    #[display(fmt = "i128")]
    I128,

    #[display(fmt = "isize")]
    Isize,

    #[display(fmt = "u8")]
    U8,

    #[display(fmt = "u16")]
    U16,

    #[display(fmt = "u32")]
    U32,

    #[display(fmt = "u64")]
    U64,

    #[display(fmt = "u128")]
    U128,

    #[display(fmt = "usize")]
    Usize,
}

impl IntTy {
    #[inline]
    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::I128 | Self::Isize,
        )
    }

    #[inline]
    pub fn is_unsigned(&self) -> bool {
        !self.is_signed()
    }

    pub fn to_ty_def(&self, primitives: &PrimitiveTyData) -> TyDef {
        let (size, align) = match self {
            Self::I8 | Self::U8 => (1, primitives.i8_align),
            Self::I16 | Self::U16 => (2, primitives.i16_align),
            Self::I32 | Self::U32 => (4, primitives.i32_align),
            Self::I64 | Self::U64 => (8, primitives.i64_align),
            Self::I128 | Self::U128 => (16, primitives.i128_align),
            Self::Isize | Self::Usize => (primitives.ptr_size, primitives.ptr_align),
        };

        TyDef::from(BasicTyDef { size, align })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
pub enum FloatTy {
    #[display(fmt = "f32")]
    F32,

    #[display(fmt = "f64")]
    F64,
}

impl FloatTy {
    pub fn to_ty_def(&self, primitives: &PrimitiveTyData) -> TyDef {
        let (size, align) = match self {
            Self::F32 => (4, primitives.f32_align),
            Self::F64 => (8, primitives.f64_align),
        };

        TyDef::from(BasicTyDef { size, align })
    }
}
