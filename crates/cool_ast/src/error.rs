use cool_lexer::Symbol;
use cool_resolve::{FnAbi, ResolveError, TyId, TyMismatch};
use derive_more::{Display, Error, From};

pub type AstResult<T = ()> = Result<T, AstError>;

#[derive(Clone, Error, From, Display, Debug)]
pub enum AstError {
    Resolve(ResolveError),
    TyMismatch(TyMismatch),
    TyNotFn(TyNotFn),
    TyHintMissing,
    FnAbiMismatch(FnAbiMismatch),
    FnParamCountMismatch(FnParamCountMismatch),
    FnVariadicMismatch(FnVariadicMismatch),
    LiteralIntOutOfRange(LiteralIntOutOfRange),
    LiteralUnknownSuffix(LiteralUnknownSuffix),

    #[display(fmt = "expected expression and found module")]
    ModuleUsedAsExpr,

    #[display(fmt = "tried to assign to an rvalue expression")]
    AssignToRvalue,

    #[display(fmt = "tried to compare uncomparable types")]
    TyNotComparable,

    #[display(fmt = "tired to use non-pointer expression as a pointer")]
    TyNotPointer,

    #[display(fmt = "tried to create a pointer from a non-addressable expression")]
    ExprNotAddressable,

    #[display(fmt = "tried to create a pointer from a non-mutably-addressable expression")]
    ExprNotMutablyAddressable,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "type is not a function")]
pub struct TyNotFn {
    pub found: TyId,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "invalid argument count: found {found}, expected {expected}")]
pub struct FnParamCountMismatch {
    pub found: u32,
    pub expected: u32,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "function abi mismatch: found \"{found}\", expected \"{expected}\"")]
pub struct FnAbiMismatch {
    pub found: FnAbi,
    pub expected: FnAbi,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "function variadicity mismatch")]
pub struct FnVariadicMismatch {
    pub found: bool,
    pub expected: bool,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "literal value {symbol} is out of range")]
pub struct LiteralIntOutOfRange {
    pub ty_id: TyId,
    pub symbol: Symbol,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "unknown literal suffix: {suffix}")]
pub struct LiteralUnknownSuffix {
    pub suffix: Symbol,
}
