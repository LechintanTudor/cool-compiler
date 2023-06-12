use crate::{ArrayTy, FnTy, InferTy, ItemTy, PrimitiveTyData, PtrTy, ResolveTy, ValueTy};
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
    pub fn is_number(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_number)
    }

    #[inline]
    pub fn is_int(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_int)
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_float)
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_array)
    }

    #[inline]
    pub fn is_diverge(&self) -> bool {
        matches!(self, Self::Diverge)
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

    #[inline]
    pub fn as_array(&self) -> Option<&ArrayTy> {
        self.as_value().and_then(ValueTy::as_array)
    }

    #[inline]
    pub fn as_fn(&self) -> Option<&FnTy> {
        self.as_value().and_then(ValueTy::as_fn)
    }

    #[inline]
    pub fn as_ptr(&self) -> Option<&PtrTy> {
        self.as_value().and_then(ValueTy::as_ptr)
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
