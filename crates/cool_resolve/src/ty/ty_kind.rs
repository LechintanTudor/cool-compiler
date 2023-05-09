use crate::{FnAbi, ItemId, TyId};
use cool_collections::SmallVecMap;
use cool_lexer::symbols::Symbol;
use derive_more::From;
use smallvec::SmallVec;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Eq, Hash, From, Debug)]
pub enum TyKind {
    Inferred(InferredTy),
    Unit,
    Int(IntTy),
    Float(FloatTy),
    Bool,
    Char,
    Pointer(PointerTy),
    Tuple(TupleTy),
    Fn(FnTy),
    StructDecl(ItemId),
    Struct(StructTy),
    Module,
}

impl TyKind {
    #[inline]
    pub fn as_fn_ty(&self) -> Option<&FnTy> {
        match self {
            Self::Fn(fn_ty) => Some(fn_ty),
            _ => None,
        }
    }
}

impl Default for TyKind {
    #[inline]
    fn default() -> Self {
        Self::Inferred(InferredTy::Any)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InferredTy {
    Any,
    Number,
    Int,
    Float,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IntTy {
    I8,
    I16,
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FloatTy {
    F32,
    F64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PointerTy {
    pub is_mutable: bool,
    pub pointee: TyId,
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
    pub fields: SmallVecMap<Symbol, TyId, 2>,
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
