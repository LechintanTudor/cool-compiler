use cool_lexer::symbols::Symbol;
use std::error::Error;
use std::fmt;

pub type ResolveResult<T> = Result<T, ResolveError>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ResolveError {
    pub symbol: Symbol,
    pub kind: ResolveErrorKind,
}

impl ResolveError {
    #[inline]
    pub fn already_defined(symbol: Symbol) -> Self {
        Self {
            symbol,
            kind: ResolveErrorKind::SymbolAlreadyDefined,
        }
    }

    #[inline]
    pub fn not_found(symbol: Symbol) -> Self {
        Self {
            symbol,
            kind: ResolveErrorKind::SymbolNotFound,
        }
    }

    #[inline]
    pub fn private(symbol: Symbol) -> Self {
        Self {
            symbol,
            kind: ResolveErrorKind::SymbolIsPrivate,
        }
    }

    #[inline]
    pub fn not_item(symbol: Symbol) -> Self {
        Self {
            symbol,
            kind: ResolveErrorKind::SymbolNotItem,
        }
    }

    #[inline]
    pub fn not_module(symbol: Symbol) -> Self {
        Self {
            symbol,
            kind: ResolveErrorKind::SymbolNotModule,
        }
    }

    #[inline]
    pub fn not_ty(symbol: Symbol) -> Self {
        Self {
            symbol,
            kind: ResolveErrorKind::SymbolNotTy,
        }
    }
}

impl Error for ResolveError {}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ResolveErrorKind::SymbolAlreadyDefined => {
                write!(f, "symbol \"{}\" is already defined", self.symbol)
            }
            ResolveErrorKind::SymbolNotFound => {
                write!(f, "symbol \"{}\" was not found", self.symbol)
            }
            ResolveErrorKind::SymbolIsPrivate => {
                write!(f, "symbol \"{}\" is private", self.symbol)
            }
            ResolveErrorKind::TooManySuperKeywords => {
                write!(f, "use path contains too many super keywords")
            }
            ResolveErrorKind::SymbolNotItem => {
                write!(f, "symbol \"{}\" does not refer to an item", self.symbol)
            }
            ResolveErrorKind::SymbolNotModule => {
                write!(f, "symbol \"{}\" does not refer to a module", self.symbol)
            }
            ResolveErrorKind::SymbolNotTy => {
                write!(f, "symbol \"{}\" does not refer to a type", self.symbol)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResolveErrorKind {
    SymbolAlreadyDefined,
    SymbolNotFound,
    SymbolIsPrivate,
    TooManySuperKeywords,
    SymbolNotItem,
    SymbolNotModule,
    SymbolNotTy,
}
