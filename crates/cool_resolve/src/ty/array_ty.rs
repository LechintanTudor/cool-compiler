use crate::TyId;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ArrayTy {
    pub elem: TyId,
    pub len: u64,
}
