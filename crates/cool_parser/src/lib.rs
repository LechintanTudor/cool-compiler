mod decl;
mod expr;
mod item;
mod module;
mod op;
mod stmt;
mod ty;
mod utils;

pub use self::decl::*;
pub use self::expr::*;
pub use self::item::*;
pub use self::module::*;
pub use self::op::*;
pub use self::stmt::*;
pub use self::ty::*;
pub use self::utils::*;

use cool_lexer::{Token, TokenKind, TokenStream};
use cool_span::Span;
use std::slice;

#[inline]
pub fn parse_module(source: &str) -> ParseResult<Module> {
    Parser::from(source).parse_module()
}

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    tokens: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    #[inline]
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

    pub fn bump_filter<R, F>(&mut self, f: F) -> Option<(Span, R)>
    where
        F: FnOnce(TokenKind) -> Option<R>,
    {
        f(self.peek().kind).map(|result| (self.bump().span, result))
    }

    pub fn error<T>(&self, found: Token, expected: &'static [TokenKind]) -> ParseResult<T> {
        Err(ParseError { found, expected })
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

impl<'a> From<&'a str> for Parser<'a> {
    #[inline]
    fn from(source: &'a str) -> Self {
        Self {
            tokens: TokenStream::new(source),
        }
    }
}

impl<'a> From<TokenStream<'a>> for Parser<'a> {
    #[inline]
    fn from(tokens: TokenStream<'a>) -> Self {
        Self { tokens }
    }
}
