use crate::ty::TyId;
use smallvec::SmallVec;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub enum TyKind {
    #[default]
    Unit,
    Int(IntTy),
    Uint(UintTy),
    Float(FloatTy),
    Tuple(TupleTy),
    Fn(FnTy),
}

impl From<IntTy> for TyKind {
    fn from(ty: IntTy) -> Self {
        Self::Int(ty)
    }
}

impl From<UintTy> for TyKind {
    fn from(ty: UintTy) -> Self {
        Self::Uint(ty)
    }
}

impl From<FloatTy> for TyKind {
    fn from(ty: FloatTy) -> Self {
        Self::Float(ty)
    }
}

impl From<TupleTy> for TyKind {
    fn from(ty: TupleTy) -> Self {
        Self::Tuple(ty)
    }
}

impl From<FnTy> for TyKind {
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
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UintTy {
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    pub elems: SmallVec<[TyId; 6]>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub args: SmallVec<[TyId; 4]>,
    pub ret: TyId,
}
