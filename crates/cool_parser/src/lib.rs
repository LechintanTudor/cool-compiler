mod assign_pattern;
mod block_elem;
mod error;
mod expr;
mod fn_extern_decl;
mod ident;
mod item;
mod parse_tree;
mod path;
mod pattern;
mod stmt;
mod ty;

pub use self::assign_pattern::*;
pub use self::block_elem::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::fn_extern_decl::*;
pub use self::ident::*;
pub use self::item::*;
pub use self::parse_tree::*;
pub use self::path::*;
pub use self::pattern::*;
pub use self::stmt::*;
pub use self::ty::*;
use cool_lexer::lexer::{LexedSourceFile, TokenStream};
use cool_lexer::tokens::{Token, TokenKind};
use cool_span::Span;
use std::iter::Peekable;

const EOF_TOKEN: Token = Token {
    span: Span::empty(),
    kind: TokenKind::Eof,
};

pub struct Parser<'a> {
    lexed: &'a LexedSourceFile,
    token_stream: Peekable<TokenStream<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexed: &'a LexedSourceFile, token_stream: TokenStream<'a>) -> Self {
        Self {
            lexed,
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

    pub fn bump_expect(&mut self, expected: &'static TokenKind) -> ParseResult<Token> {
        let token = self.bump();

        if &token.kind != expected {
            return self.error(token, std::slice::from_ref(expected));
        }

        Ok(token)
    }

    pub fn peek(&mut self) -> Token {
        self.token_stream.peek().copied().unwrap_or(EOF_TOKEN)
    }

    pub fn error<T>(&self, found: Token, expected: &'static [TokenKind]) -> ParseResult<T> {
        let position = self.lexed.line_offsets.to_source_position(found.span.start);

        Err(ParseError {
            position,
            found,
            expected,
        })
    }

    pub fn peek_error<T>(&mut self, expected: &'static [TokenKind]) -> ParseResult<T> {
        let token = self.peek();
        self.error(token, expected)
    }
}
