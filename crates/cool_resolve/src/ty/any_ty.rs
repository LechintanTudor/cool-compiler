use crate::{InferTy, ItemTy, PrimitiveTyData, ResolveTy, ValueTy};
use derive_more::From;

#[derive(Clone, Eq, PartialEq, Hash, From, Debug)]
pub enum AnyTy {
    Infer(InferTy),
    Item(ItemTy),
    Value(ValueTy),
}

impl AnyTy {
    #[inline]
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    pub fn to_resolve_ty(self, primitives: &PrimitiveTyData) -> ResolveTy {
        match self {
            Self::Infer(_) | Self::Item(_) => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: self,
                }
            }
            Self::Value(value_ty) => value_ty.to_resolve_ty(primitives),
        }
    }
}
