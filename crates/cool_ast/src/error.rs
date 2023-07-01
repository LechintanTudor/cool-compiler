use cool_lexer::Symbol;
use cool_resolve::{FnAbi, ResolveError, TyId};
use cool_span::Span;
use derive_more::{Display, Error, From};
use std::fmt;

pub trait AstResultExt {
    fn error<E>(span: Span, error: E) -> Self
    where
        E: Into<AstErrorKind>;

    fn ty_mismatch(span: Span, found_ty_id: TyId, expected_ty_id: TyId) -> Self;

    fn field_not_found(span: Span, ty_id: TyId, field: Symbol) -> Self;
}

pub type AstResult<T = ()> = Result<T, AstError>;

impl<T> AstResultExt for AstResult<T> {
    fn error<E>(span: Span, error: E) -> Self
    where
        E: Into<AstErrorKind>,
    {
        Self::Err(AstError {
            span,
            kind: error.into(),
        })
    }

    fn ty_mismatch(span: Span, found_ty_id: TyId, expected_ty_id: TyId) -> Self {
        Self::Err(AstError::ty_mismatch(span, found_ty_id, expected_ty_id))
    }

    fn field_not_found(span: Span, ty_id: TyId, field: Symbol) -> Self {
        Self::Err(AstError::field_not_found(span, ty_id, field))
    }
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "{}", kind)]
pub struct AstError {
    pub span: Span,
    pub kind: AstErrorKind,
}

impl AstError {
    pub fn new<E>(span: Span, error: E) -> Self
    where
        E: Into<AstErrorKind>,
    {
        Self {
            span,
            kind: error.into(),
        }
    }

    pub fn ty_mismatch(span: Span, found_ty_id: TyId, expected_ty_id: TyId) -> Self {
        Self {
            span,
            kind: AstErrorKind::Ty(TyError {
                ty_id: found_ty_id,
                kind: TyErrorKind::TyMismatch { expected_ty_id },
            }),
        }
    }

    pub fn field_not_found(span: Span, ty_id: TyId, field: Symbol) -> Self {
        Self {
            span,
            kind: AstErrorKind::from(LogicError::FieldNotFound { ty_id, field }),
        }
    }

    #[inline]
    pub fn with_span(self, span: Span) -> Self {
        Self { span, ..self }
    }
}

impl From<AstErrorKind> for AstError {
    #[inline]
    fn from(kind: AstErrorKind) -> Self {
        Self {
            span: Span::empty(),
            kind,
        }
    }
}

#[derive(Clone, From, Error, Display, Debug)]
pub enum AstErrorKind {
    Expr(ExprError),
    Literal(LiteralError),
    Logic(LogicError),
    Resolve(ResolveError),
    Ty(TyError),
    TyDef(TyDefError),
}

#[derive(Clone, Error, Display, Debug)]
pub enum TyDefError {
    #[display(fmt = "unknown function ABI '{abi}'")]
    UnknownAbi { abi: Symbol },

    #[display(fmt = "function ABI mismatch")]
    AbiMismatch { found: FnAbi, expected: FnAbi },

    #[display(fmt = "function parameter count mismatch")]
    ParamCountMismatch { found: u32, expected: u32 },

    #[display(fmt = "function variadicity mismtach")]
    VariadicMismatch { found: bool, expected: bool },

    #[display(fmt = "type hint missing from parameter '{param}'")]
    TyHintMissing { param: Symbol },
}

#[derive(Clone, Debug)]
pub enum TyErrorKind {
    InvalidArgumentCount { found: u32 },
    TyMismatch { expected_ty_id: TyId },
    TyNotCallable,
    TyNotComparable,
    TyNotDereferenceable,
    UnsupportedCast { to_ty_id: TyId },
}

#[derive(Clone, Error, Debug)]
pub struct TyError {
    pub ty_id: TyId,
    pub kind: TyErrorKind,
}

impl fmt::Display for TyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TyErrorKind::InvalidArgumentCount { found } => {
                write!(
                    f,
                    "tried to call function with an invalid number of {} arguments",
                    found,
                )
            }
            TyErrorKind::TyMismatch { expected_ty_id } => {
                write!(f, "expected '{}', found '{}'", expected_ty_id, self.ty_id)
            }
            TyErrorKind::TyNotCallable => {
                write!(f, "expressions of type '{}' are not callable", self.ty_id)
            }
            TyErrorKind::TyNotComparable => {
                write!(f, "expressions of type '{}' are not comparable", self.ty_id)
            }
            TyErrorKind::TyNotDereferenceable => {
                write!(
                    f,
                    "expressions of type '{}' are not dereferenceable",
                    self.ty_id,
                )
            }
            TyErrorKind::UnsupportedCast { to_ty_id } => {
                write!(
                    f,
                    "unsupported cast from '{}' to '{}'",
                    self.ty_id, to_ty_id,
                )
            }
        }
    }
}

#[derive(Clone, Error, Display, Debug)]
pub enum ExprError {
    #[display(fmt = "Expression is not addressable.")]
    NotAddressable,

    #[display(fmt = "expression is not mutably addressable")]
    NotAddressableMutably,

    #[display(fmt = "expression is not assignable")]
    NotAssignable,
}

#[derive(Clone, Debug)]
pub enum LiteralErrorKind {
    UnknownSuffix { suffix: Symbol },
    IntOutOfRange { ty_id: TyId },
}

#[derive(Clone, Error, Debug)]
pub struct LiteralError {
    pub literal: Symbol,
    pub kind: LiteralErrorKind,
}

impl fmt::Display for LiteralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LiteralErrorKind::UnknownSuffix { suffix } => {
                write!(
                    f,
                    "unknown literal suffix '{}' in '{}'",
                    suffix, self.literal,
                )
            }
            LiteralErrorKind::IntOutOfRange { ty_id } => {
                write!(f, "literal {} is out of range of {}", self.literal, ty_id)
            }
        }
    }
}

#[derive(Clone, Error, Display, Debug)]
pub enum LogicError {
    #[display(fmt = "tried to jump from outside a loop")]
    InvalidJump,

    #[display(fmt = "type '{ty_id}' has no field '{field}'")]
    FieldNotFound { ty_id: TyId, field: Symbol },

    #[display(
        fmt = "variant '{variant_ty_id}' is missing in type '{ty_id}' or was already covered"
    )]
    InvalidVariant { ty_id: TyId, variant_ty_id: TyId },

    #[display(fmt = "missing variants for type '{ty_id}'")]
    MissingVariants { ty_id: TyId },
}
