use cool_lexer::tokens::{Token, TokenKind};
use std::error::Error;
use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct ParseError {
    pub found: Token,
    pub expected: &'static [TokenKind],
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Unexpected token: {}", self.found.kind)
    }
}
