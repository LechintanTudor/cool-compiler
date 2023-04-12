use crate::ty::TyId;
use cool_lexer::symbols::Symbol;
use smallvec::SmallVec;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TyKind {
    Inferred(InferredTy),
    Unit,
    Int(IntTy),
    Float(FloatTy),
    Bool,
    Char,
    Tuple(TupleTy),
    Fn(FnTy),
    Module,
}

impl Default for TyKind {
    #[inline]
    fn default() -> Self {
        Self::Inferred(InferredTy::Any)
    }
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

impl From<InferredTy> for TyKind {
    #[inline]
    fn from(ty: InferredTy) -> Self {
        Self::Inferred(ty)
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
pub enum InferredTy {
    Any,
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    pub elems: SmallVec<[TyId; 6]>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub abi: Symbol,
    pub params: SmallVec<[TyId; 4]>,
    pub ret: TyId,
}
