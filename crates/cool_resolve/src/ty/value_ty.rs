use crate::{FnAbi, ItemId, TyId};
use cool_lexer::symbols::Symbol;
use derive_more::From;
use paste::paste;
use smallvec::SmallVec;
use std::hash::{Hash, Hasher};

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum IntTy {
    I8,
    I16,
    #[default]
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum FloatTy {
    F32,
    #[default]
    F64,
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
pub struct TupleTy {
    pub elems: SmallVec<[TyId; 6]>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub abi: FnAbi,
    pub params: SmallVec<[TyId; 4]>,
    pub is_variadic: bool,
    pub ret: TyId,
}

#[derive(Clone, Eq, Debug)]
pub struct StructTy {
    pub item_id: ItemId,
    pub fields: Vec<(Symbol, TyId)>,
}

impl StructTy {
    #[inline]
    pub fn empty(item_id: ItemId) -> Self {
        Self {
            item_id,
            fields: Default::default(),
        }
    }
}

impl Hash for StructTy {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.item_id.hash(state);
    }
}

impl PartialEq for StructTy {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id
    }
}
