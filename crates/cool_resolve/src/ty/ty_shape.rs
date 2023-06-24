use crate::{InferTy, ItemTy, ValueTy};
use derive_more::From;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, From, Debug)]
pub enum TyShape {
    Infer(InferTy),
    Item(ItemTy),
    Value(ValueTy),
    Diverge,
}

impl TyShape {
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
    pub fn is_signed_int(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_signed_int)
    }

    #[inline]
    pub fn is_unsigned_int(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_unsigned_int)
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
}

impl fmt::Display for TyShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Infer(infer_ty) => write!(f, "{infer_ty}"),
            Self::Item(item_ty) => write!(f, "{item_ty}"),
            Self::Value(value_ty) => write!(f, "{value_ty}"),
            Self::Diverge => write!(f, "<diverge>"),
        }
    }
}
