use crate::TyId;
use derive_more::{Display, Error};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Error, Display, Debug)]
#[display(fmt = "type mismatch: expected: {expected_ty_id:?}, found: {found_ty_id:?}")]
pub struct TyMismatch {
    pub found_ty_id: TyId,
    pub expected_ty_id: TyId,
}
