use cool_lexer::{Token, TokenKind};
use derive_more::{Display, Error};

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone, Error, Display, Debug)]
#[display(fmt = "unexpected token: {}", "self.found.kind")]
pub struct ParseError {
    pub found: Token,
    pub expected: &'static [TokenKind],
}
