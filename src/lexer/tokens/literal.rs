use crate::symbol::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LiteralKind {
    Integer,
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
            LiteralKind::Integer => write!(f, "{}", self.symbol),
            LiteralKind::String => write!(f, "\"{}\"", self.symbol),
        }
    }
}
