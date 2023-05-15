use cool_lexer::symbols::{sym, Symbol};
use derive_more::{Display, Error};
use std::fmt;

#[derive(Clone, Copy, Error, Display, Debug)]
#[display(fmt = "function has an unknown abi")]
pub struct UnknownAbi;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum FnAbi {
    #[default]
    Cool,
    C,
}

impl fmt::Display for FnAbi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Self::Cool => "Cool",
            Self::C => "C",
        };

        write!(f, "{}", display_str)
    }
}

impl TryFrom<Symbol> for FnAbi {
    type Error = UnknownAbi;

    #[inline]
    fn try_from(symbol: Symbol) -> Result<Self, Self::Error> {
        let abi = match symbol {
            sym::ABI_COOL => Self::Cool,
            sym::ABI_C => Self::C,
            _ => return Err(UnknownAbi),
        };

        Ok(abi)
    }
}
