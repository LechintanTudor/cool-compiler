use crate::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LiteralKind {
    Bool,
    Int,
    Char,
    Str,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub value: Symbol,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            LiteralKind::Bool | LiteralKind::Int => write!(f, "{}", self.value),
            LiteralKind::Char => write!(f, "'{}'", self.value),
            LiteralKind::Str => write!(f, "\"{}\"", self.value),
        }
    }
}
