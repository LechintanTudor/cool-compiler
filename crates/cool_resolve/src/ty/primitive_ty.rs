use crate::{AnyTy, ResolveTy, ValueTy};

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IntTy {
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
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

    pub fn to_resolve_ty(&self, primitives: &PrimitiveTyData) -> ResolveTy {
        let (size, align) = match self {
            Self::I8 | Self::U8 => (1, primitives.i8_align),
            Self::I16 | Self::U16 => (2, primitives.i16_align),
            Self::I32 | Self::U32 => (4, primitives.i32_align),
            Self::I64 | Self::U64 => (8, primitives.i64_align),
            Self::I128 | Self::U128 => (16, primitives.i128_align),
            Self::Isize | Self::Usize => (primitives.ptr_size, primitives.ptr_align),
        };

        ResolveTy {
            size,
            align,
            ty: AnyTy::Value(ValueTy::Int(*self)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FloatTy {
    F32,
    F64,
}

impl FloatTy {
    pub fn to_resolve_ty(&self, primitives: &PrimitiveTyData) -> ResolveTy {
        let (size, align) = match self {
            Self::F32 => (4, primitives.f32_align),
            Self::F64 => (8, primitives.f64_align),
        };

        ResolveTy {
            size,
            align,
            ty: AnyTy::Value(ValueTy::Float(*self)),
        }
    }
}
