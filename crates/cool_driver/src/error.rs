use crate::{ModulePathsError, SourceId};
use cool_ast::{AstError, LiteralError};
use cool_parser::ParseError;
use cool_resolve::{ItemId, ResolveError};
use cool_span::Span;
use derive_more::{Display, Error, From};
use std::io::Error as IoError;
use std::path::PathBuf;

pub type SpannedCompileResult<T = ()> = Result<T, SpannedCompileError>;
pub type CompileResult<T = ()> = Result<T, CompileError>;

#[derive(Error, Debug, Display)]
#[display("{}", self.error)]
pub struct SpannedCompileError {
    pub location: ErrorLocation,

    #[error(source)]
    pub error: CompileError,
}

impl SpannedCompileError {
    pub fn new<L, E>(location: L, error: E) -> Self
    where
        L: Into<ErrorLocation>,
        E: Into<CompileError>,
    {
        Self {
            location: location.into(),
            error: error.into(),
        }
    }
}

#[derive(Clone, From, Debug)]
pub enum ErrorLocation {
    File(PathBuf),
    Source((SourceId, Span)),
}

pub trait WithLocation {
    type Success;

    fn with_location<L>(self, location: L) -> SpannedCompileResult<Self::Success>
    where
        L: Into<ErrorLocation>;
}

impl<T, E> WithLocation for Result<T, E>
where
    E: Into<CompileError>,
{
    type Success = T;

    fn with_location<L>(self, location: L) -> SpannedCompileResult<Self::Success>
    where
        L: Into<ErrorLocation>,
    {
        self.map_err(|error| {
            SpannedCompileError {
                location: location.into(),
                error: error.into(),
            }
        })
    }
}

#[derive(From, Error, Debug, Display)]
pub enum CompileError {
    Path(ModulePathsError),
    Io(IoError),
    Parse(ParseError),
    Resolve(ResolveError),
    Literal(LiteralError),
    Define(DefineError),
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

#[derive(Error, Debug, Display)]
#[display("Item cannot be defined")]
pub struct DefineError {
    pub item_id: ItemId,
}
