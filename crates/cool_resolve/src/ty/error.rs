use crate::TyId;
use derive_more::{Display, Error};

#[derive(Clone, Copy, Error, Display, Debug)]
#[display(fmt = "type mismatch: expected {expected}, found {found}")]
pub struct TyMismatch {
    pub found: TyId,
    pub expected: TyId,
}
