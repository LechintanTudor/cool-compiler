use crate::ModulePathsError;
use cool_ast::AstError;
use cool_lexer::Symbol;
use cool_parser::ParseError;
use cool_resolve::{ItemId, ItemPathBuf, ResolveError, TyId};
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
    Ast(AstError),
    Define(DefineError),
    Import(ImportError),
    Init(InitError),
    Module(ModuleError),
    Parse(ParseError),
    Resolve(ResolveError),
}

impl CompileError {
    pub fn span(&self) -> Option<Span> {
        match self {
            Self::Ast(e) => Some(e.span),
            Self::Define(e) => e.span,
            Self::Import(e) => Some(e.span),
            Self::Module(e) => e.span,
            Self::Parse(e) => Some(e.found.span),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum DefineErrorKind {
    Item(ItemId),
    Ty(TyId),
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "item {:?} could not be defined", kind)]
pub struct DefineError {
    pub span: Option<Span>,
    pub kind: DefineErrorKind,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "failed to import '{path}'")]
pub struct ImportError {
    pub span: Span,
    pub path: ItemPathBuf,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "failed to initialize compiler: {message}")]
pub struct InitError {
    pub message: String,
}

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "no file found for module '{module_name}'")]
pub struct ModuleError {
    pub span: Option<Span>,
    pub module_name: Symbol,
    pub error: ModulePathsError,
}
