mod aggregate_ty;
mod float_ty;
mod int_ty;

pub use self::aggregate_ty::*;
pub use self::float_ty::*;
pub use self::int_ty::*;
use crate::{FnAbi, TyId};
use derive_more::From;
use paste::paste;
use smallvec::SmallVec;
use std::hash::Hash;

macro_rules! define_value_ty {
    { Simple { $($SimpleTy:ident,)* }, Wrapped { $($WrappedTy:ident,)* }, } => {
        paste! {
            #[derive(Clone, PartialEq, Eq, Hash, From, Debug)]
            pub enum ValueTy {
                $($SimpleTy,)*
                $($WrappedTy([<$WrappedTy Ty>]),)*
            }

            impl ValueTy {
                $(
                    #[inline]
                    pub fn [<as_ $WrappedTy:snake:lower>](&self) -> Option<&[<$WrappedTy Ty>]> {
                        match self {
                            Self::$WrappedTy(ty) => Some(ty),
                            _ => None,
                        }
                    }
                )*
            }
        }
    };
}

define_value_ty! {
    Simple {
        Unit,
        Bool,
        Char,
        Range,
    },
    Wrapped {
        Int,
        Float,
        Fn,
        Ptr,
        ManyPtr,
        Slice,
        Array,
        Tuple,
        Struct,
    },
}

impl ValueTy {
    #[inline]
    pub fn is_subscriptable(&self) -> bool {
        matches!(self, Self::ManyPtr(_) | Self::Slice(_) | Self::Array(_))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ArrayTy {
    pub len: u64,
    pub elem: TyId,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PtrTy {
    pub is_mutable: bool,
    pub pointee: TyId,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ManyPtrTy {
    pub is_mutable: bool,
    pub pointee: TyId,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SliceTy {
    pub is_mutable: bool,
    pub elem: TyId,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub abi: FnAbi,
    pub params: SmallVec<[TyId; 4]>,
    pub is_variadic: bool,
    pub ret: TyId,
}
