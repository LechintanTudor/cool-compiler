use cool_lexer::symbols::Symbol;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ResolveError {
    pub symbol: Symbol,
    pub kind: ResolveErrorKind,
}

impl Error for ResolveError {}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ResolveErrorKind::SymbolNotFound => {
                write!(f, "symbol \"{}\" was not found", self.symbol)
            }
            ResolveErrorKind::SymbolIsPrivate => {
                write!(f, "symbol \"{}\" is private", self.symbol)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResolveErrorKind {
    SymbolNotFound,
    SymbolIsPrivate,
}
