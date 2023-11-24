use cool_resolve::{ExprId, ResolveError};
use cool_span::Span;
use derive_more::{Display, Error, From};

pub type SpannedAstResult<T = ()> = Result<T, SpannedAstError>;
pub type AstResult<T = ()> = Result<T, AstError>;
pub type LiteralResult<T = ()> = Result<T, LiteralError>;

#[derive(Clone, Error, Debug, Display)]
#[display("{}", self.error)]
pub struct SpannedAstError {
    pub span: Span,

    #[error(source)]
    pub error: AstError,
}

impl SpannedAstError {
    pub fn new<E>(span: Span, error: E) -> Self
    where
        E: Into<AstError>,
    {
        Self {
            span,
            error: error.into(),
        }
    }
}

#[derive(Clone, Error, From, Debug, Display)]
pub enum AstError {
    Resolve(ResolveError),
    Literal(LiteralError),
    Semantic(SemanticError),
}

#[derive(Clone, Error, Debug, Display)]
pub enum LiteralError {
    ValueTooLarge,
    SuffixUnknown,
}

#[derive(Clone, Error, Debug, Display)]
pub enum SemanticError {
    #[display("Expression is not assignable")]
    ExprNotAssignable { expr_id: ExprId },
}

pub trait WithSpan {
    type Success;

    fn with_span(self, span: Span) -> SpannedAstResult<Self::Success>;
}

impl<T, E> WithSpan for Result<T, E>
where
    E: Into<AstError>,
{
    type Success = T;

    fn with_span(self, span: Span) -> SpannedAstResult<Self::Success> {
        self.map_err(|error| {
            SpannedAstError {
                span,
                error: error.into(),
            }
        })
    }
}
