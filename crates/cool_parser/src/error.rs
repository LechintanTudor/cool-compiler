use cool_lexer::tokens::{Token, TokenKind};
use cool_span::Span;
use std::error::Error;
use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Debug)]
pub enum ParseError {
    UnexpectedToken(UnexpectedToken),
}

impl ParseError {
    pub fn span(&self) -> Span {
        match self {
            Self::UnexpectedToken(error) => error.found.span,
        }
    }
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

impl From<UnexpectedToken> for ParseError {
    fn from(error: UnexpectedToken) -> Self {
        Self::UnexpectedToken(error)
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
            "Unexpected token '{}'. Expected one of: {}.",
            self.found.kind,
            TokenKindListDisplayer(self.expected),
        )
    }
}

struct TokenKindListDisplayer<'a>(&'a [TokenKind]);

impl fmt::Display for TokenKindListDisplayer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some((first, others)) = self.0.split_first() else {
            return Ok(())
        };

        write!(f, "'{}'", first)?;

        for other in others {
            write!(f, ", '{}'", other)?;
        }

        Ok(())
    }
}
