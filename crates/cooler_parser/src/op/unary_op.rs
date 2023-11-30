use crate::{parse_error, ParseResult, Parser};
use cool_lexer::tk;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UnaryOp {
    Minus,
    Not,
    Address { is_mutable: bool },
}

impl UnaryOp {
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Minus => "-",
            Self::Not => "!",
            Self::Address { is_mutable: false } => "&",
            Self::Address { is_mutable: true } => "&mut",
        }
    }
}

impl fmt::Display for UnaryOp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Parser<'_> {
    pub fn parse_unary_op(&mut self) -> ParseResult<UnaryOp> {
        let token = self.bump();

        let op = match token.kind {
            tk::minus => UnaryOp::Minus,
            tk::not => UnaryOp::Not,
            tk::and => {
                let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();
                UnaryOp::Address { is_mutable }
            }
            _ => return parse_error(token, &[tk::minus, tk::not, tk::and]),
        };

        Ok(op)
    }
}
