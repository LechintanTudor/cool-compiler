mod block_elem;
mod error;
mod expr;
mod ident;
mod item;
mod parse_tree;
mod path;
mod pattern;
mod stmt;
mod ty;

pub use self::block_elem::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::ident::*;
pub use self::item::*;
pub use self::parse_tree::*;
pub use self::path::*;
pub use self::pattern::*;
pub use self::stmt::*;
pub use self::ty::*;
use cool_lexer::lexer::TokenStream;
use cool_lexer::tokens::{Token, TokenKind};
use cool_span::Span;
use std::iter::Peekable;

const EOF_TOKEN: Token = Token {
    span: Span::empty(),
    kind: TokenKind::Eof,
};

pub struct Parser<'a> {
    token_stream: Peekable<TokenStream<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self {
            token_stream: token_stream.peekable(),
        }
    }

    pub fn bump(&mut self) -> Token {
        self.token_stream.next().unwrap_or(EOF_TOKEN)
    }

    pub fn bump_if_eq(&mut self, kind: TokenKind) -> Option<Token> {
        if self.peek().kind == kind {
            Some(self.bump())
        } else {
            None
        }
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
        self.token_stream.peek().copied().unwrap_or(EOF_TOKEN)
    }
}
