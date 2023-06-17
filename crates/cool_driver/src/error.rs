use crate::ModulePathsError;
use cool_ast::AstError;
use cool_parser::ParseError;
use cool_resolve::{DefineError, ItemPathBuf, ResolveError};
use cool_span::Span;
use derive_more::{Display, Error, From};
use std::fmt;

pub type CompileResult<T> = Result<T, CompileErrorBundle>;

#[derive(Error, Debug)]
pub struct CompileErrorBundle {
    pub errors: Vec<CompileError>,
}

impl From<CompileError> for CompileErrorBundle {
    #[inline]
    fn from(error: CompileError) -> Self {
        Self {
            errors: vec![error],
        }
    }
}

impl fmt::Display for CompileErrorBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "failed to compile package")?;

        for error in self.errors.iter() {
            writeln!(f, "{}", error)?;
        }

        Ok(())
    }
}

#[derive(Clone, Error, From, Display, Debug)]
pub enum CompileError {
    Init(InitError),
    Path(ModulePathsError),
    Parse(ParseError),
    Import(ImportError),
    Resolve(ResolveError),
    Define(DefineError),
    Ast(AstError),
}

impl CompileError {
    pub fn span(&self) -> Option<Span> {
        let span = match self {
            Self::Parse(e) => e.found.span,
            Self::Ast(e) => e.span,
            _ => return None,
        };

        Some(span)
    }
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "failed to initialize compiler: {message}")]
pub struct InitError {
    pub message: String,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "failed to import {import_path:?} in module {module_path:?}")]
pub struct ImportError {
    pub module_path: ItemPathBuf,
    pub import_path: ItemPathBuf,
}
