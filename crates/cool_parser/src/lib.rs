mod decl;
mod expr;
mod item;
mod stmt;
mod ty;
mod utils;

pub use self::decl::*;
pub use self::expr::*;
pub use self::item::*;
pub use self::stmt::*;
pub use self::ty::*;
pub use self::utils::*;

use cool_lexer::{Token, TokenKind, TokenStream};
use std::slice;

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    tokens: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    #[inline]
    pub fn new(tokens: TokenStream<'a>) -> Self {
        Self { tokens }
    }

    #[inline]
    #[must_use]
    pub fn bump(&mut self) -> Token {
        self.tokens.next_lang()
    }

    #[inline]
    #[must_use]
    pub fn peek(&mut self) -> Token {
        self.tokens.peek_lang()
    }

    #[inline]
    #[must_use]
    pub fn peek_any(&mut self) -> Token {
        self.tokens.peek_any()
    }

    pub fn bump_if_eq(&mut self, expected_token: TokenKind) -> Option<Token> {
        if self.peek().kind != expected_token {
            return None;
        }

        Some(self.bump())
    }

    pub fn bump_expect(&mut self, expected_token: &'static TokenKind) -> ParseResult<Token> {
        let token = self.bump();

        if &token.kind != expected_token {
            return Err(ParseError {
                found: token,
                expected: slice::from_ref(expected_token),
            });
        }

        Ok(token)
    }

    #[inline]
    pub fn error<T>(&self, found: Token, expected: &'static [TokenKind]) -> ParseResult<T> {
        Err(ParseError { found, expected })
    }

    #[inline]
    pub fn peek_error<T>(&mut self, expected: &'static [TokenKind]) -> ParseResult<T> {
        Err(ParseError {
            found: self.peek(),
            expected,
        })
    }
}
