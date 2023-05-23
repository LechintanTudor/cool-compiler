use cool_lexer::symbols::Symbol;
use cool_resolve::{FnAbi, ResolveError, TyId, TyMismatch};
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
    AssignToRvalue(#[from] AssignToRvalue),

    #[error(transparent)]
    TyNotComparable(#[from] TyNotComparable),

    #[error(transparent)]
    TyNotPointer(#[from] TyNotPointer),

    #[error(transparent)]
    ExprNotAddressable(#[from] ExprNotAddressable),

    #[error(transparent)]
    ExprNotMutablyAddressable(#[from] ExprNotMutablyAddressable),
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
#[error("tried to assign to an rvalue")]
pub struct AssignToRvalue;

#[derive(Clone, Error, Debug)]
#[error("tried to compare uncomparable types")]
pub struct TyNotComparable;

#[derive(Clone, Error, Debug)]
#[error("tried to dereference non-pointer type")]
pub struct TyNotPointer;

#[derive(Clone, Error, Debug)]
#[error("tried to create a pointer from a non-addressable expression")]
pub struct ExprNotAddressable;

#[derive(Clone, Error, Debug)]
#[error("tried to create a mutable pointer from a non-mutably-addresable expression")]
pub struct ExprNotMutablyAddressable;
