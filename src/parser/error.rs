use crate::lexer::{Token, TokenKind};
use std::error::Error;
use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Debug)]
pub enum ParseError {
    UnexpectedToken(UnexpectedToken),
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::UnexpectedToken(e) => Some(e),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken(e) => fmt::Display::fmt(e, f),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnexpectedToken {
    pub found: Token,
    pub expected: &'static [TokenKind],
}

impl Error for UnexpectedToken {}

impl fmt::Display for UnexpectedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unexpected token '{}' at position {}. Expected one of: {:?}.",
            self.found.kind, self.found.span.start, self.expected
        )
    }
}
