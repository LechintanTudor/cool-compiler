use crate::{InferTy, ItemTy, PrimitiveTyData, ResolveTy, ValueTy};
use derive_more::From;

#[derive(Clone, Eq, PartialEq, Hash, From, Debug)]
pub enum AnyTy {
    Infer(InferTy),
    Item(ItemTy),
    Value(ValueTy),
    Diverge,
}

impl AnyTy {
    #[inline]
    pub fn is_infer(&self) -> bool {
        matches!(self, Self::Infer(_))
    }

    #[inline]
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    #[inline]
    pub fn is_diverge(&self) -> bool {
        matches!(self, Self::Diverge)
    }

    #[inline]
    pub fn is_number(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_number)
    }

    #[inline]
    pub fn is_comparable(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_comparable)
    }

    #[inline]
    pub fn as_value(&self) -> Option<&ValueTy> {
        match self {
            Self::Value(ty) => Some(ty),
            _ => None,
        }
    }

    pub fn to_resolve_ty(self, primitives: &PrimitiveTyData) -> ResolveTy {
        match self {
            Self::Infer(_) | Self::Item(_) | Self::Diverge => {
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
