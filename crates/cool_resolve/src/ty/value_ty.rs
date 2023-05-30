use crate::{FnAbi, ItemId, TyId};
use cool_lexer::symbols::Symbol;
use derive_more::From;
use smallvec::SmallVec;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Eq, Hash, From, Debug)]
pub enum ValueTy {
    Unit,
    Int(IntTy),
    Float(FloatTy),
    Bool,
    Char,
    Fn(FnTy),
    Array(ArrayTy),
    Ptr(PtrTy),
    ManyPtr(ManyPtrTy),
    Slice(SliceTy),
    Tuple(TupleTy),
    Struct(StructTy),
}

impl ValueTy {
    #[inline]
    pub fn as_fn(&self) -> Option<&FnTy> {
        match self {
            Self::Fn(fn_ty) => Some(fn_ty),
            _ => None,
        }
    }

    #[inline]
    pub fn as_array(&self) -> Option<&ArrayTy> {
        match self {
            Self::Array(array_ty) => Some(array_ty),
            _ => None,
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> Option<&PtrTy> {
        match self {
            Self::Ptr(ptr_ty) => Some(ptr_ty),
            _ => None,
        }
    }

    pub fn as_struct(&self) -> Option<&StructTy> {
        match self {
            Self::Struct(struct_ty) => Some(struct_ty),
            _ => None,
        }
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
