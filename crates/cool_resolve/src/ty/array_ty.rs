use crate::TyId;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ArrayTy {
    pub elem: TyId,
    pub len: u64,
}

impl fmt::Display for ArrayTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]{}", self.len, self.elem)
    }
}
