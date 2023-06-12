use crate::{FnAbi, TyId};
use smallvec::SmallVec;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub abi: FnAbi,
    pub params: SmallVec<[TyId; 2]>,
    pub is_variadic: bool,
    pub ret: TyId,
}
