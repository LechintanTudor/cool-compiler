use crate::symbols::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LiteralKind {
    Int { is_plain: bool },
    Decimal,
    Bool,
    Char,
    Str,
}

impl LiteralKind {
    #[inline]
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int { .. })
    }

    #[inline]
    pub fn is_plain_int(&self) -> bool {
        matches!(self, Self::Int { is_plain: true })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub symbol: Symbol,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LiteralKind::Str => write!(f, "\"{}\"", self.symbol),
            LiteralKind::Char => write!(f, "'{}'", self.symbol),
            _ => write!(f, "{}", self.symbol),
        }
    }
}
