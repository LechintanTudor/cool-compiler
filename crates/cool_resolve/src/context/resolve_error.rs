use cool_lexer::{sym, Symbol};
use derive_more::Error;
use std::fmt;

pub type ResolveResult<T> = Result<T, ResolveError>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResolveErrorKind {
    SymbolAlreadyDefined,
    SymbolNotFound,
    SymbolNotPublic,
    SymbolNotItem,
    SymbolNotModule,
    SymbolNotTy,
    SymbolNotAbi,
    TooManySuperKeywords,
}

impl ResolveErrorKind {
    fn message(&self) -> &'static str {
        match self {
            ResolveErrorKind::SymbolAlreadyDefined => "was already defined",
            ResolveErrorKind::SymbolNotFound => "could not be found",
            ResolveErrorKind::SymbolNotPublic => "is not public",
            ResolveErrorKind::SymbolNotItem => "does not refer to an item",
            ResolveErrorKind::SymbolNotModule => "does not refer to a module",
            ResolveErrorKind::SymbolNotTy => "does not refer to a type",
            ResolveErrorKind::SymbolNotAbi => "does not refer to an abi",
            ResolveErrorKind::TooManySuperKeywords => "path contains too many super keywords",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Error, Debug)]
pub struct ResolveError {
    pub symbol: Symbol,
    pub kind: ResolveErrorKind,
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.symbol {
            sym::EMPTY => write!(f, "{}", self.kind.message()),
            _ => write!(f, "'{}' {}", self.symbol, self.kind.message()),
        }
    }
}
