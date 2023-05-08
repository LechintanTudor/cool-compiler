use crate::{InferredTy, ItemId, TyKind};

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
    pub kind: TyKind,
}

impl ResolveTy {
    #[inline]
    pub fn inferred(inferred_ty: InferredTy) -> Self {
        Self {
            size: 0,
            align: 0,
            kind: TyKind::Inferred(inferred_ty),
        }
    }

    #[inline]
    pub fn struct_decl(item_id: ItemId) -> Self {
        Self {
            size: 0,
            align: 0,
            kind: TyKind::StructDecl(item_id),
        }
    }

    pub fn module() -> Self {
        Self {
            size: 0,
            align: 0,
            kind: TyKind::Module,
        }
    }
}

impl Default for ResolveTy {
    #[inline]
    fn default() -> Self {
        Self {
            size: 0,
            align: 1,
            kind: TyKind::Unit,
        }
    }
}

impl ResolveTy {
    #[inline]
    pub fn is_zst(&self) -> bool {
        self.size == 0
    }
}
