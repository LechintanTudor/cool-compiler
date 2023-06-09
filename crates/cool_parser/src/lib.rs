mod abstract_fn;
mod cond_block;
mod decl;
mod error;
mod expr;
mod expr_or_stmt;
mod fn_extern_decl;
mod fn_prototype;
mod ident;
mod item;
mod op;
mod path;
mod pattern;
mod stmt;
mod ty;

pub use self::abstract_fn::*;
pub use self::cond_block::*;
pub use self::decl::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::expr_or_stmt::*;
pub use self::fn_extern_decl::*;
pub use self::fn_prototype::*;
pub use self::ident::*;
pub use self::item::*;
pub use self::op::*;
pub use self::path::*;
pub use self::pattern::*;
pub use self::stmt::*;
pub use self::ty::*;
use cool_lexer::{Token, TokenKind, TokenStream};

pub struct Parser<'a> {
    token_stream: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self { token_stream }
    }

    pub fn bump(&mut self) -> Token {
        self.token_stream.next_lang()
    }

    pub fn peek(&mut self) -> Token {
        self.token_stream.peek_lang()
    }

    pub fn peek_any(&mut self) -> Token {
        self.token_stream.peek_any()
    }

    pub fn bump_if_eq(&mut self, kind: TokenKind) -> Option<Token> {
        if self.peek().kind == kind {
            Some(self.bump())
        } else {
            None
        }
    }

    pub fn bump_expect(&mut self, expected: &'static TokenKind) -> ParseResult<Token> {
        let token = self.bump();

        if &token.kind != expected {
            return self.error(token, std::slice::from_ref(expected));
        }

        Ok(token)
    }

    pub fn error<T>(&self, found: Token, expected: &'static [TokenKind]) -> ParseResult<T> {
        Err(ParseError { found, expected })
    }

    pub fn peek_error<T>(&mut self, expected: &'static [TokenKind]) -> ParseResult<T> {
        let token = self.peek();
        self.error(token, expected)
    }
}
