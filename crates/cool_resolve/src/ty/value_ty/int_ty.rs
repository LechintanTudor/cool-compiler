use crate::{PrimitiveTys, ResolveTy, ValueTy};
use std::fmt;

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
    pub fn to_resolve_ty(&self, primitives: &PrimitiveTys) -> ResolveTy {
        let mk_resolve_ty = |size, align| {
            ResolveTy {
                size,
                align,
                ty: ValueTy::Int(*self),
            }
        };

        match self {
            Self::I8 | Self::U8 => mk_resolve_ty(1, primitives.i8_align),
            Self::I16 | Self::U16 => mk_resolve_ty(2, primitives.i16_align),
            Self::I32 | Self::U32 => mk_resolve_ty(4, primitives.i32_align),
            Self::I64 | Self::U64 => mk_resolve_ty(8, primitives.i64_align),
            Self::I128 | Self::U128 => mk_resolve_ty(16, primitives.i128_align),
            Self::Isize | Self::Usize => mk_resolve_ty(primitives.ptr_size, primitives.ptr_align),
        }
    }
}

impl fmt::Display for IntTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::Isize => "isize",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::Usize => "usize",
        };

        f.write_str(display_str)
    }
}
