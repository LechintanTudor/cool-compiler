use crate::{AnyTy, InferTy, ItemTy};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct ResolveTy {
    pub size: u64,
    pub align: u64,
    pub ty: AnyTy,
}

impl From<InferTy> for ResolveTy {
    #[inline]
    fn from(ty: InferTy) -> Self {
        Self {
            size: 0,
            align: 1,
            ty: AnyTy::Infer(ty),
        }
    }
}

impl From<ItemTy> for ResolveTy {
    #[inline]
    fn from(ty: ItemTy) -> Self {
        Self {
            size: 0,
            align: 1,
            ty: AnyTy::Item(ty),
        }
    }
}

impl PartialEq for ResolveTy {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}

impl Eq for ResolveTy {}

impl Hash for ResolveTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.ty.hash(state);
    }
}
