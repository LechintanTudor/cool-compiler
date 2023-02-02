use crate::symbol::Symbol;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LiteralKind {
    Integer { suffix: Option<Symbol> },
    Boolean,
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
            LiteralKind::Integer { suffix: None } => write!(f, "{}", self.symbol),
            LiteralKind::Integer {
                suffix: Some(suffix),
            } => {
                write!(f, "{}{}", self.symbol, suffix)
            }
            LiteralKind::Boolean => write!(f, "{}", self.symbol),
            LiteralKind::String => write!(f, "\"{}\"", self.symbol),
        }
    }
}
