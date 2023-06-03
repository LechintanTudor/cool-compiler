mod aggregate_ty;
mod float_ty;
mod int_ty;

pub use self::aggregate_ty::*;
pub use self::float_ty::*;
pub use self::int_ty::*;
use crate::{FnAbi, ItemId, TyId};
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
        Array,
        Aggregate,
    },
}

impl ValueTy {
    #[inline]
    pub fn is_subscriptable(&self) -> bool {
        matches!(
            self,
            Self::ManyPtr(_)
                | Self::Array(_)
                | Self::Aggregate(AggregateTy {
                    kind: AggregateKind::Slice,
                    ..
                }),
        )
    }

    #[inline]
    pub fn as_struct(&self) -> Option<&AggregateTy> {
        self.as_aggregate()
            .filter(|aggregate| matches!(aggregate.kind, AggregateKind::Struct(_)))
    }

    #[inline]
    pub fn as_struct_parts(&self) -> Option<(ItemId, &[Field])> {
        let aggregate = self.as_aggregate()?;

        match aggregate.kind {
            AggregateKind::Struct(item_id) => Some((item_id, &aggregate.fields)),
            _ => None,
        }
    }

    #[inline]
    pub fn as_tuple(&self) -> Option<&AggregateTy> {
        self.as_aggregate()
            .filter(|aggregate| matches!(aggregate.kind, AggregateKind::Tuple))
    }

    #[inline]
    pub fn as_slice(&self) -> Option<&AggregateTy> {
        self.as_aggregate()
            .filter(|aggregate| matches!(aggregate.kind, AggregateKind::Slice))
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub abi: FnAbi,
    pub params: SmallVec<[TyId; 4]>,
    pub is_variadic: bool,
    pub ret: TyId,
}
