use crate::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LiteralKind {
    Int,
    Char,
    Str,
}

#[derive(Clone, Copy, Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub value: Symbol,
}

impl fmt::Display for Literal {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LiteralKind::Int => write!(f, "{}", self.value),
            LiteralKind::Char => write!(f, "'{}'", self.value),
            LiteralKind::Str => write!(f, "\"{}\"", self.value),
        }
    }
}
