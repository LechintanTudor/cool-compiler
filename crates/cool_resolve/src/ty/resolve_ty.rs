use crate::ValueTy;

#[derive(Clone, Copy, Debug)]
pub struct PrimitiveTyProps {
    pub i8_align: u64,
    pub i16_align: u64,
    pub i32_align: u64,
    pub i64_align: u64,
    pub i128_align: u64,
    pub f32_align: u64,
    pub f64_align: u64,
    pub ptr_size: u64,
    pub ptr_align: u64,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ResolveTy {
    pub size: u64,
    pub align: u64,
    pub ty: ValueTy,
}

impl ResolveTy {
    #[inline]
    pub fn is_zst(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn is_comparable(&self) -> bool {
        matches!(
            self.ty,
            ValueTy::Int(_) | ValueTy::Float(_) | ValueTy::Bool | ValueTy::Char | ValueTy::Ptr(_),
        )
    }
}

impl Default for ResolveTy {
    #[inline]
    fn default() -> Self {
        Self {
            size: 0,
            align: 1,
            ty: ValueTy::Unit,
        }
    }
}
