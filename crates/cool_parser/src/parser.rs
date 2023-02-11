use crate::error::{ParseResult, UnexpectedToken};
use cool_lexer::tokens::{Token, TokenKind};
use std::iter::Peekable;

pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    tokens: Peekable<T>,
    source_len: u32,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn new(tokens: T, source_len: u32) -> Self {
        Self {
            tokens: tokens.peekable(),
            source_len,
        }
    }

    pub fn bump(&mut self) -> Token {
        self.tokens.next().unwrap_or(self.eof_token())
    }

    pub fn bump_expect(&mut self, expected: &'static [TokenKind]) -> ParseResult<Token> {
        let token = self.bump();

        if !expected.contains(&token.kind) {
            return Err(UnexpectedToken {
                found: token,
                expected,
            })?;
        }

        Ok(token)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.peek().copied().unwrap_or(self.eof_token())
    }

    fn eof_token(&self) -> Token {
        Token::eof(self.source_len)
    }
}
