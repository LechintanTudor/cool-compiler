use cool_resolve::ty::TyId;
use std::error::Error;
use std::fmt;

pub type SemanticResult<T> = Result<T, SemanticError>;

#[derive(Clone, Debug)]
pub enum SemanticError {
    TyMismatch(TyMismatch),
    TyNotFn(TyNotFn),
    InvalidArgCount(InvalidArgCount),
}

impl Error for SemanticError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::TyMismatch(e) => Some(e),
            Self::TyNotFn(e) => Some(e),
            Self::InvalidArgCount(e) => Some(e),
        }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TyMismatch(e) => fmt::Display::fmt(e, f),
            Self::TyNotFn(e) => fmt::Display::fmt(e, f),
            Self::InvalidArgCount(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<TyMismatch> for SemanticError {
    #[inline]
    fn from(error: TyMismatch) -> Self {
        Self::TyMismatch(error)
    }
}

impl From<TyNotFn> for SemanticError {
    #[inline]
    fn from(error: TyNotFn) -> Self {
        Self::TyNotFn(error)
    }
}

impl From<InvalidArgCount> for SemanticError {
    #[inline]
    fn from(error: InvalidArgCount) -> Self {
        Self::InvalidArgCount(error)
    }
}

#[derive(Clone, Debug)]
pub struct TyMismatch {
    pub found_ty: TyId,
    pub expected_ty: Option<TyId>,
}

impl Error for TyMismatch {
    // Empty
}

impl fmt::Display for TyMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to resolve types")
    }
}

#[derive(Clone, Debug)]
pub struct TyNotFn {
    pub found_ty: TyId,
}

impl Error for TyNotFn {
    // Empty
}

impl fmt::Display for TyNotFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type is not a function")
    }
}

#[derive(Clone, Debug)]
pub struct InvalidArgCount {
    pub found: u32,
    pub expected: u32,
}

impl Error for InvalidArgCount {
    // Empty
}

impl fmt::Display for InvalidArgCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid argument count: found {}, expected {}",
            self.found, self.expected
        )
    }
}
