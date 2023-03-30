use crate::ty::{tys, TyId};
use smallvec::SmallVec;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub enum TyKind {
    #[default]
    Unit,
    Int(IntTy),
    Float(FloatTy),
    Tuple(TupleTy),
    Fn(FnTy),
}

impl TyKind {
    #[inline]
    pub fn is_builtin(&self) -> bool {
        matches!(self, Self::Unit | Self::Int(_) | Self::Float(_))
    }
}

impl From<IntTy> for TyKind {
    #[inline]
    fn from(ty: IntTy) -> Self {
        Self::Int(ty)
    }
}

impl From<FloatTy> for TyKind {
    #[inline]
    fn from(ty: FloatTy) -> Self {
        Self::Float(ty)
    }
}

impl From<TupleTy> for TyKind {
    #[inline]
    fn from(ty: TupleTy) -> Self {
        Self::Tuple(ty)
    }
}

impl From<FnTy> for TyKind {
    #[inline]
    fn from(ty: FnTy) -> Self {
        Self::Fn(ty)
    }
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

impl IntTy {
    pub fn ty_id(&self) -> TyId {
        match self {
            Self::I8 => tys::I8,
            Self::I16 => tys::I16,
            Self::I32 => tys::I32,
            Self::I64 => tys::I64,
            Self::I128 => tys::I128,
            Self::Isize => tys::ISIZE,
            Self::U8 => tys::U8,
            Self::U16 => tys::U16,
            Self::U32 => tys::U32,
            Self::U64 => tys::U64,
            Self::U128 => tys::U128,
            Self::Usize => tys::USIZE,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FloatTy {
    F32,
    F64,
}

impl FloatTy {
    #[inline]
    pub fn ty_id(&self) -> TyId {
        match self {
            Self::F32 => tys::F32,
            Self::F64 => tys::F64,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    pub elems: SmallVec<[TyId; 6]>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub args: SmallVec<[TyId; 4]>,
    pub ret: TyId,
}
