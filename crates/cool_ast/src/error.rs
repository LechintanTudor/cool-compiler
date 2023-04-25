use cool_resolve::{FnAbi, ResolveError, TyId};
use thiserror::Error;

pub type AstResult<T = ()> = Result<T, AstError>;

#[derive(Clone, Error, Debug)]
pub enum AstError {
    #[error(transparent)]
    TyMismatch(#[from] TyMismatch),

    #[error(transparent)]
    TyNotFn(#[from] TyNotFn),

    #[error(transparent)]
    InvalidParamCount(#[from] InvalidParamCount),

    #[error(transparent)]
    TyHintMissing(#[from] TyHintMissing),

    #[error(transparent)]
    FnAbiMismatch(#[from] FnAbiMismatch),

    #[error(transparent)]
    FnVariadicMismatch(#[from] FnVariadicMismatch),

    #[error(transparent)]
    Resolve(#[from] ResolveError),
}

#[derive(Clone, Error, Debug)]
#[error("failed to resolve types")]
pub struct TyMismatch {
    pub found_ty: TyId,
    pub expected_ty: TyId,
}

#[derive(Clone, Error, Debug)]
#[error("type is not a function")]
pub struct TyNotFn {
    pub found_ty: TyId,
}

#[derive(Clone, Error, Debug)]
#[error("invalid argument count: found {found}, expected {expected}")]
pub struct InvalidParamCount {
    pub found: u32,
    pub expected: u32,
}

#[derive(Clone, Error, Debug)]
#[error("missing type hint")]
pub struct TyHintMissing;

#[derive(Clone, Error, Debug)]
#[error("function abi mismatch: found \"{found}\", expected \"{expected}\"")]
pub struct FnAbiMismatch {
    pub found: FnAbi,
    pub expected: FnAbi,
}

#[derive(Clone, Error, Debug)]
#[error("function variadicity mismatch")]
pub struct FnVariadicMismatch {
    pub found: bool,
    pub expected: bool,
}
