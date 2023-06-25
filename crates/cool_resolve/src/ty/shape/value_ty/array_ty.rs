use crate::TyId;
use derive_more::Display;

#[derive(Clone, Eq, PartialEq, Hash, Display, Debug)]
#[display(fmt = "[{len}]{elem}")]
pub struct ArrayTy {
    pub elem: TyId,
    pub len: u64,
}
