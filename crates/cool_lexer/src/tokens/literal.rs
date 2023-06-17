use crate::symbols::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IntBase {
    B2,
    B8,
    B10,
    B16,
}

impl IntBase {
    #[inline]
    pub fn as_int(&self) -> u32 {
        match self {
            Self::B2 => 2,
            Self::B8 => 8,
            Self::B10 => 10,
            Self::B16 => 16,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LiteralKind {
    Int { base: IntBase, has_suffix: bool },
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

    /// Returns `true` if and only if the literal is a base 10 number without a suffix.
    #[inline]
    pub fn is_plain_int(&self) -> bool {
        matches!(
            self,
            Self::Int {
                base: IntBase::B10,
                has_suffix: false
            }
        )
    }

    #[inline]
    pub fn is_base_ten_int(&self) -> bool {
        matches!(
            self,
            Self::Int {
                base: IntBase::B10,
                ..
            }
        )
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
