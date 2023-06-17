use cool_lexer::{Token, TokenKind};
use derive_more::Error;
use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Error, Debug)]
pub struct ParseError {
    pub found: Token,
    pub expected: &'static [TokenKind],
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "unexpected token: `{}`", self.found.kind)?;
        write!(f, "expected: {}", ListDisplayer(self.expected))
    }
}

#[derive(Clone, Copy, Debug)]
struct ListDisplayer<'a, T>(pub &'a [T]);

impl<T> fmt::Display for ListDisplayer<'_, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            [] => Ok(()),
            [elem] => write!(f, "`{elem}`"),
            [first, middle @ .., last] => {
                write!(f, "`{first}`")?;

                for elem in middle {
                    write!(f, ", `{elem}`")?;
                }

                write!(f, " or `{last}`")
            }
        }
    }
}
