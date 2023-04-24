use crate::ModulePathsError;
use cool_ast::AstError;
use cool_parser::ParseError;
use cool_resolve::{DefineError, ItemPathBuf, ResolveError};
use std::fmt;
use std::path::PathBuf;
use thiserror::Error;

pub type CompileResult<T> = Result<T, CompileErrorBundle>;

#[derive(Error, Debug)]
pub struct CompileErrorBundle {
    pub errors: Vec<CompileError>,
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

#[derive(Clone, Error, Debug)]
pub struct CompileError {
    pub path: PathBuf,
    pub kind: CompileErrorKind,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "error in file {}:", self.path.display())?;
        write!(f, " -> {}", self.kind)
    }
}

#[derive(Clone, Error, Debug)]
pub enum CompileErrorKind {
    #[error(transparent)]
    Path(#[from] ModulePathsError),

    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error(transparent)]
    Import(#[from] ImportError),

    #[error(transparent)]
    Resolve(#[from] ResolveError),

    #[error(transparent)]
    Define(#[from] DefineError),

    #[error(transparent)]
    Ast(#[from] AstError),
}

#[derive(Clone, Error, Debug)]
#[error("failed to import {import_path:?} in module {module_path:?}")]
pub struct ImportError {
    pub module_path: ItemPathBuf,
    pub import_path: ItemPathBuf,
}
