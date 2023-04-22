use cool_lexer::symbols::Symbol;
use paste::paste;
use std::error::Error;
use std::fmt;

pub type ResolveResult<T> = Result<T, ResolveError>;

macro_rules! define_resolve_error {
    {
        Symbol {
            $($SymbolError:ident: $symbol_message:literal,)+
        }

        Other {
            $($OtherError:ident: $other_message:literal,)+
        }
    } => {
        paste!{
            #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
            pub enum ResolveErrorKind {
                $([<Symbol $SymbolError>],)+
                $($OtherError,)+
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct ResolveError {
            pub symbol: Symbol,
            pub kind: ResolveErrorKind,
        }

        paste! {
            impl ResolveError {
                $(
                    #[inline]
                    pub fn [<$SymbolError:snake:lower>](symbol: Symbol) -> Self {
                        Self {
                            symbol,
                            kind: ResolveErrorKind::[<Symbol $SymbolError>],
                        }
                    }
                )+

                $(
                    #[inline]
                    pub fn [<$OtherError:snake:lower>](symbol: Symbol) -> Self {
                        Self {
                            symbol,
                            kind: ResolveErrorKind::$OtherError,
                        }
                    }
                )+
            }
        }

        impl Error for ResolveError {
            // Empty
        }

        impl fmt::Display for ResolveError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                paste!{
                    match self.kind {
                        $(
                            ResolveErrorKind::[<Symbol $SymbolError>] => {
                                write!(f, "symbol \"{}\" {}", self.symbol, $symbol_message)
                            }
                        )+
                        $(
                            ResolveErrorKind::$OtherError => {
                                write!(f, "{}", $other_message)
                            }
                        )+
                    }
                }
            }
        }
    };
}

define_resolve_error! {
    Symbol {
        AlreadyDefined: "was already defined",
        NotFound: "could not be found",
        NotPublic: "is not public",
        NotItem: "does not refer to an item",
        NotModule: "does not refer to a module",
        NotTy: "does not refer to a type",
        NotAbi: "does not refer to an abi",
    }

    Other {
        TooManySuperKeywords: "path contains too many super keywords",
    }
}
