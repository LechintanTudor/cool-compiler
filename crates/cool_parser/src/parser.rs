use crate::error::{ParseResult, UnexpectedToken};
use cool_lexer::tokens::{Token, TokenKind};
use cool_span::Span;
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

    pub fn bump_kind(&mut self) -> TokenKind {
        self.tokens
            .next()
            .map(|token| token.kind)
            .unwrap_or(TokenKind::Eof)
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

    pub fn bump_split(&mut self) -> (Span, TokenKind) {
        let token = self.bump();
        (token.span, token.kind)
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.peek().copied().unwrap_or(self.eof_token())
    }

    pub fn peek_kind(&mut self) -> TokenKind {
        self.tokens
            .peek()
            .map(|token| token.kind)
            .unwrap_or(TokenKind::Eof)
    }

    pub fn peek_split(&mut self) -> (Span, TokenKind) {
        let token = self.peek();
        (token.span, token.kind)
    }

    fn eof_token(&self) -> Token {
        Token::eof(self.source_len)
    }
}
