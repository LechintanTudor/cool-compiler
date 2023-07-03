mod infer_ty;
mod item_ty;
mod value_ty;

pub use self::infer_ty::*;
pub use self::item_ty::*;
pub use self::value_ty::*;
use derive_more::{Display, From};

#[derive(Clone, Eq, PartialEq, Hash, From, Display, Debug)]
pub enum TyShape {
    Infer(InferTy),
    Item(ItemTy),
    Value(ValueTy),

    #[display(fmt = "<diverge>")]
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
    pub fn is_usize(&self) -> bool {
        self.as_value().is_some_and(ValueTy::is_usize)
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
    pub fn get_value(&self) -> &ValueTy {
        match self {
            Self::Value(ty) => ty,
            _ => panic!("type is not a value type"),
        }
    }
}
