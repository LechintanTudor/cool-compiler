use crate::symbols::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IntBase {
    Two,
    Eight,
    Ten,
    Sixteen,
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
                base: IntBase::Ten,
                has_suffix: false
            }
        )
    }

    #[inline]
    pub fn is_base_ten_int(&self) -> bool {
        matches!(
            self,
            Self::Int {
                base: IntBase::Ten,
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
