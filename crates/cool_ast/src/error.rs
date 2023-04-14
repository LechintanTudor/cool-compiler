use cool_lexer::symbols::Symbol;
use cool_resolve::TyId;
use thiserror::Error;

pub type AstResult<T = ()> = Result<T, AstError>;

#[derive(Clone, Error, Debug)]
pub enum AstError {
    #[error(transparent)]
    TyMismatch(#[from] TyMismatch),

    #[error(transparent)]
    TyNotFn(#[from] TyNotFn),

    #[error(transparent)]
    InvalidArgCount(#[from] InvalidArgCount),

    #[error(transparent)]
    TyHintMissing(#[from] TyHintMissing),

    #[error(transparent)]
    FnAbiMismatch(#[from] FnAbiMismatch),
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
pub struct InvalidArgCount {
    pub found: u32,
    pub expected: u32,
}

#[derive(Clone, Error, Debug)]
#[error("missing type hint")]
pub struct TyHintMissing;

#[derive(Clone, Error, Debug)]
#[error("function abi mismatch: found \"{found}\", expected \"{expected}\"")]
pub struct FnAbiMismatch {
    pub found: Symbol,
    pub expected: Symbol,
}
