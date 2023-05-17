use crate::{InferTy, ItemId, TyKind};

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
    pub fn inferred(inferred_ty: InferTy) -> Self {
        Self {
            size: 0,
            align: 0,
            kind: TyKind::Infer(inferred_ty),
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

    #[inline]
    pub fn module() -> Self {
        Self {
            size: 0,
            align: 0,
            kind: TyKind::Module,
        }
    }

    #[inline]
    pub fn ty() -> Self {
        Self {
            size: 0,
            align: 0,
            kind: TyKind::Ty,
        }
    }

    #[inline]
    pub fn is_zst(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn is_comparable(&self) -> bool {
        matches!(
            self.kind,
            TyKind::Int(_) | TyKind::Float(_) | TyKind::Bool | TyKind::Char | TyKind::Pointer(_),
        )
    }
}

impl Default for ResolveTy {
    #[inline]
    fn default() -> Self {
        Self {
            size: 0,
            align: 0,
            kind: TyKind::Unit,
        }
    }
}
