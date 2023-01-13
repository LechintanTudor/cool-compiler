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
        let kind_str = match self.kind {
            LiteralKind::Integer => "integer",
            LiteralKind::String => "string",
        };

        write!(f, "<{} {}>", kind_str, self.symbol.0)
    }
}
