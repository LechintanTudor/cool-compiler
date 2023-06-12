use crate::TyId;
use derive_more::{Display, Error};

#[derive(Clone, Copy, Error, Display, Debug)]
#[display(fmt = "[Type Mismatch]\n -> expected: {expected_ty_id}\n -> found:    {found_ty_id}")]
pub struct TyMismatch {
    pub found_ty_id: TyId,
    pub expected_ty_id: TyId,
}
