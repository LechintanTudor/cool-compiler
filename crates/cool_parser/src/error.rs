use cool_lexer::tokens::{Token, TokenKind};
use cool_span::SourcePosition;
use std::error::Error;
use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct ParseError {
    pub position: SourcePosition,
    pub found: Token,
    pub expected: &'static [TokenKind],
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Unexpected token '{}' at line {}, column {}.",
            self.found.kind, self.position.line, self.position.column,
        )?;

        writeln!(
            f,
            " -> Expected: {}.",
            ExpectedTokenDisplayer(self.expected)
        )
    }
}

struct ExpectedTokenDisplayer<'a>(&'a [TokenKind]);

impl fmt::Display for ExpectedTokenDisplayer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some((first, others)) = self.0.split_first() else {
            return Ok(())
        };

        write!(f, "'{}'", first)?;

        let Some((last, others)) = others.split_last() else {
            return Ok(())
        };

        for other in others {
            write!(f, ", '{}'", other)?;
        }

        write!(f, " or '{}'", last)
    }
}
