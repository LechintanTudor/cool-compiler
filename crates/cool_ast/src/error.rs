use cool_lexer::{Token, TokenKind};
use cool_span::{Section, Span};
use derive_more::{Constructor, Error};
use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Copy, Constructor, Error, Debug)]
pub struct ParseError {
    pub found: Token,
    pub expected: &'static [TokenKind],
}

impl Section for ParseError {
    #[inline]
    fn span(&self) -> Span {
        self.found.span
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Unexpected token: {}", self.found.kind)?;
        write!(f, "Expected: {}", TokenKindDisplayer(self.expected))
    }
}

#[derive(Clone, Copy, Debug)]
struct TokenKindDisplayer(&'static [TokenKind]);

impl fmt::Display for TokenKindDisplayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some((first, others)) = self.0.split_first() else {
            return Ok(());
        };

        write!(f, "{first}")?;

        let Some((last, others)) = others.split_last() else {
            return Ok(());
        };

        for other in others {
            write!(f, ", {other}")?;
        }

        write!(f, " or {last}")
    }
}

pub fn parse_error<T>(found: Token, expected: &'static [TokenKind]) -> ParseResult<T> {
    Err(ParseError::new(found, expected))
}
