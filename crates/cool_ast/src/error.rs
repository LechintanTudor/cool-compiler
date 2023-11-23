use cool_resolve::ResolveError;
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

#[derive(Clone, Error, From, Debug, Display)]
pub enum AstError {
    Resolve(ResolveError),
    Literal(LiteralError),
}

#[derive(Clone, Error, Debug, Display)]
pub enum LiteralError {
    ValueTooLarge,
    SuffixUnknown,
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
