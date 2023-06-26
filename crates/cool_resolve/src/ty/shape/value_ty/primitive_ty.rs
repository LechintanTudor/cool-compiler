use derive_more::Display;

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
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
pub enum FloatTy {
    #[display(fmt = "f32")]
    F32,

    #[display(fmt = "f64")]
    F64,
}
