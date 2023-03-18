use crate::symbols::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Radix {
    Two,
    Eight,
    Ten,
    Sixteen,
}

impl fmt::Debug for Radix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = match self {
            Self::Two => 2,
            Self::Eight => 8,
            Self::Ten => 10,
            Self::Sixteen => 16,
        };

        write!(f, "{}", base)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LiteralKind {
    Integer { radix: Radix },
    Bool,
    Char,
    String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub symbol: Symbol,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LiteralKind::String => write!(f, "\"{}\"", self.symbol),
            LiteralKind::Char => write!(f, "'{}'", self.symbol),
            _ => write!(f, "{}", self.symbol),
        }
    }
}
