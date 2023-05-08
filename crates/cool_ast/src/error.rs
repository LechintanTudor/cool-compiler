use cool_lexer::symbols::Symbol;
use cool_resolve::{FnAbi, ResolveError, TyId};
use thiserror::Error;

pub type AstResult<T = ()> = Result<T, AstError>;

#[derive(Clone, Error, Debug)]
pub enum AstError {
    #[error(transparent)]
    Resolve(#[from] ResolveError),

    #[error(transparent)]
    TyMismatch(#[from] TyMismatch),

    #[error(transparent)]
    TyNotFn(#[from] TyNotFn),

    #[error(transparent)]
    TyHintMissing(#[from] TyHintMissing),

    #[error(transparent)]
    FnAbiMismatch(#[from] FnAbiMismatch),

    #[error(transparent)]
    FnParamCountMismatch(#[from] FnParamCountMismatch),

    #[error(transparent)]
    FnVariadicMismatch(#[from] FnVariadicMismatch),

    #[error(transparent)]
    LiteralIntOutOfRange(#[from] LiteralIntOutOfRange),

    #[error(transparent)]
    LiteralUnknownSuffix(#[from] LiteralUnknownSuffix),

    #[error(transparent)]
    ModuleUsedAsExpr(#[from] ModuleUsedAsExpr),

    #[error(transparent)]
    MissingElseBlock(#[from] MissingElseBlock),

    #[error(transparent)]
    AssignToRvalue(#[from] AssignToRvalue),
}

#[derive(Clone, Error, Debug)]
#[error("failed to resolve types")]
pub struct TyMismatch {
    pub found: TyId,
    pub expected: TyId,
}

#[derive(Clone, Error, Debug)]
#[error("type is not a function")]
pub struct TyNotFn {
    pub found: TyId,
}

#[derive(Clone, Error, Debug)]
#[error("invalid argument count: found {found}, expected {expected}")]
pub struct FnParamCountMismatch {
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

#[derive(Clone, Error, Debug)]
#[error("literal value {symbol} is out of range")]
pub struct LiteralIntOutOfRange {
    pub ty_id: TyId,
    pub symbol: Symbol,
}

#[derive(Clone, Error, Debug)]
#[error("unknown literal suffix: {suffix}")]
pub struct LiteralUnknownSuffix {
    pub suffix: Symbol,
}

#[derive(Clone, Error, Debug)]
#[error("expression evaluates to a module")]
pub struct ModuleUsedAsExpr;

#[derive(Clone, Error, Debug)]
#[error("missing else block in non-unit expression")]
pub struct MissingElseBlock;

#[derive(Clone, Error, Debug)]
#[error("tried to assign to an rvalue")]
pub struct AssignToRvalue;
