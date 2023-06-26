use crate::TyId;
use cool_lexer::Symbol;
use derive_more::{Display, Error};
use std::fmt;

#[derive(Clone, Copy, Error, Display, Debug)]
#[display(fmt = "[Type Mismatch]\n -> expected: {expected_ty_id}\n -> found:    {found_ty_id}")]
pub struct TyMismatch {
    pub found_ty_id: TyId,
    pub expected_ty_id: TyId,
}

pub type TyResult<T = ()> = Result<T, TyError>;

#[derive(Clone, Debug)]
pub enum TyErrorKind {
    Unify { expected: TyId },
    CannotBeDefined,
    StructHasInfiniteSize,
    StructHasDuplicatedField { field: Symbol },
    EnumHasInvalidStorage { storage: TyId },
    EnumHasDuplicatedVariant { variant: Symbol },
}

#[derive(Clone, Error, Debug)]
pub struct TyError {
    pub ty_id: TyId,
    pub kind: TyErrorKind,
}

impl fmt::Display for TyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TyErrorKind::Unify { expected } => {
                write!(
                    f,
                    "type mismatch - expected: '{}', found: '{}'",
                    expected, self.ty_id,
                )
            }
            TyErrorKind::CannotBeDefined => {
                write!(f, "type '{}' cannot be defined", self.ty_id)
            }
            TyErrorKind::StructHasInfiniteSize => {
                write!(f, "struct '{}' has infinite size", self.ty_id)
            }
            TyErrorKind::StructHasDuplicatedField { field } => {
                write!(
                    f,
                    "struct '{}' has duplicated field '{}'",
                    self.ty_id, field,
                )
            }
            TyErrorKind::EnumHasInvalidStorage { storage } => {
                write!(
                    f,
                    "enum '{}' has invalid storage of type '{}'",
                    self.ty_id, storage,
                )
            }
            TyErrorKind::EnumHasDuplicatedVariant { variant } => {
                write!(
                    f,
                    "enum '{}' has duplicated variant '{}'",
                    self.ty_id, variant,
                )
            }
        }
    }
}
