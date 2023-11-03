use crate::{ItemId, ResolveError, TyId};
use cool_lexer::{sym, Symbol};
use derive_more::From;
use smallvec::SmallVec;

#[derive(Clone, PartialEq, Eq, Hash, From, Debug)]
pub enum TyKind {
    // Undefined types
    Infer(InferTy),
    Item(ItemTy),

    // Defined types
    Unit,
    Int(IntTy),
    Float(FloatTy),
    Ptr(PtrTy),
    ManyPtr(ManyPtrTy),
    Slice(SliceTy),
    Array(ArrayTy),
    Tuple(TupleTy),
    Struct(StructTy),
    Fn(FnTy),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InferTy {
    Any,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemTy {
    Alias,
    Module,
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
pub struct PtrTy {
    pub pointee_ty: TyId,
    pub is_mutable: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ManyPtrTy {
    pub pointee_ty: TyId,
    pub is_mutable: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SliceTy {
    pub elem_ty: TyId,
    pub is_mutable: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ArrayTy {
    pub elem_ty: TyId,
    pub len: u64,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TupleTy {
    pub elem_tys: SmallVec<[TyId; 4]>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StructTy {
    pub item_id: ItemId,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub abi: FnAbi,
    pub param_tys: SmallVec<[TyId; 4]>,
    pub is_variadic: bool,
    pub return_ty: TyId,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FnAbi {
    Cool,
    C,
}

impl TryFrom<Symbol> for FnAbi {
    type Error = ResolveError;

    #[inline]
    fn try_from(abi: Symbol) -> Result<Self, Self::Error> {
        let abi = match abi {
            sym::Cool => FnAbi::Cool,
            sym::C => FnAbi::C,
            _ => return Err(ResolveError::FnAbiIsUnknown { abi }),
        };

        Ok(abi)
    }
}
