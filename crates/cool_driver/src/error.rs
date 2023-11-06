use crate::ModulePathsError;
use cool_ast::{AstError, LiteralError};
use cool_parser::ParseError;
use cool_resolve::ResolveError;
use derive_more::{Display, Error, From};
use std::io::Error as IoError;

pub type CompileResult<T> = Result<T, CompileError>;

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
