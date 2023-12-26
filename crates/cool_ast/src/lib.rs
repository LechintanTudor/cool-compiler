mod error;
mod expr;
mod file;
mod item;
mod op;
mod stmt;
mod ty;
mod utils;

pub use self::error::*;
pub use self::expr::*;
pub use self::file::*;
pub use self::item::*;
pub use self::op::*;
pub use self::stmt::*;
pub use self::ty::*;
pub use self::utils::*;

use cool_collections::define_index_newtype;
use cool_lexer::{Token, TokenKind, TokenStream};
use std::slice;

define_index_newtype!(CrateId);
define_index_newtype!(FileId);

#[derive(Debug)]
pub struct Parser<'a> {
    file: File,
    tokens: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    #[inline]
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self {
            file: File::default(),
            tokens: TokenStream::new(source),
        }
    }

    #[inline]
    pub fn bump(&mut self) -> Token {
        self.tokens.next_lang()
    }

    #[inline]
    pub fn bump_any(&mut self) -> Token {
        self.tokens.next_any()
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

    pub fn bump_any_if_eq(&mut self, expected_token: TokenKind) -> Option<Token> {
        if self.peek_any().kind != expected_token {
            return None;
        }

        Some(self.tokens.next_any())
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

    pub fn peek_error<T>(&mut self, expected: &'static [TokenKind]) -> ParseResult<T> {
        Err(ParseError {
            found: self.peek(),
            expected,
        })
    }

    pub fn peek_any_error<T>(&mut self, expected: &'static [TokenKind]) -> ParseResult<T> {
        Err(ParseError {
            found: self.peek_any(),
            expected,
        })
    }
}
