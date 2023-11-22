use crate::{ModulePathsError, SourceId};
use cool_ast::{AstError, LiteralError};
use cool_parser::ParseError;
use cool_resolve::ResolveError;
use cool_span::Span;
use derive_more::{Display, Error, From};
use std::io::Error as IoError;
use std::path::PathBuf;

pub type CompileResult<T = ()> = Result<T, CompileError>;
pub type SpannedCompileResult<T = ()> = Result<T, SpannedCompileError>;

#[derive(Error, Debug, Display)]
#[display("{}", self.error)]
pub struct SpannedCompileError {
    pub location: ErrorLocation,

    #[error(source)]
    pub error: CompileError,
}

#[derive(Clone, From, Debug)]
pub enum ErrorLocation {
    File(PathBuf),
    Source((SourceId, Span)),
}

pub trait WithLocation {
    fn with_location<L>(self, location: L) -> SpannedCompileError
    where
        L: Into<ErrorLocation>;
}

impl<E> WithLocation for E
where
    E: Into<CompileError>,
{
    fn with_location<L>(self, location: L) -> SpannedCompileError
    where
        L: Into<ErrorLocation>,
    {
        SpannedCompileError {
            location: location.into(),
            error: self.into(),
        }
    }
}

#[derive(From, Error, Debug, Display)]
pub enum CompileError {
    Path(ModulePathsError),
    Io(IoError),
    Parse(ParseError),
    Resolve(ResolveError),
    Literal(LiteralError),
}

impl From<AstError> for CompileError {
    #[inline]
    fn from(error: AstError) -> Self {
        match error {
            AstError::Resolve(e) => Self::Resolve(e),
            AstError::Literal(e) => Self::Literal(e),
        }
    }
}
