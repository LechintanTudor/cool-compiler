use crate::lexer::{IdentTable, LiteralTable, TokenKind};
use crate::parser::RootAst;
use std::iter::Peekable;
use std::slice::Iter as SliceIter;

pub struct Parser<'a> {
    tokens: Peekable<SliceIter<'a, TokenKind>>,
    identifier_table: &'a IdentTable,
    literal_table: &'a LiteralTable,
}

impl<'a> Parser<'a> {
    pub fn new(
        tokens: &'a [TokenKind],
        identifier_table: &'a IdentTable,
        literal_table: &'a LiteralTable,
    ) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            identifier_table,
            literal_table,
        }
    }

    pub fn parse(&mut self) -> anyhow::Result<RootAst> {
        self.parse_root()
    }

    pub fn next(&mut self) -> TokenKind {
        self.tokens.next().copied().unwrap_or(TokenKind::Eof)
    }

    pub fn next_and<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(TokenKind) -> bool,
    {
        f(self.next())
    }

    pub fn peek(&mut self) -> TokenKind {
        self.tokens
            .peek()
            .map(|&&token| token)
            .unwrap_or(TokenKind::Eof)
    }

    pub fn peek_eq<T>(&mut self, token: T) -> bool
    where
        T: Into<TokenKind>,
    {
        self.peek() == token.into()
    }

    pub fn consume(&mut self) {
        self.next();
    }

    pub fn consume_ident(&mut self) -> Option<u32> {
        let index = self.peek().as_ident_index();

        if index.is_some() {
            self.consume();
        }

        index
    }

    pub fn consume_lit(&mut self) -> Option<u32> {
        let index = self.peek().as_lit_index();

        if index.is_some() {
            self.consume();
        }

        index
    }

    pub fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(TokenKind) -> bool,
    {
        if f(self.peek()) {
            self.consume();
            true
        } else {
            false
        }
    }

    pub fn consume_if_eq<T>(&mut self, token: T) -> bool
    where
        T: Into<TokenKind>,
    {
        if self.peek() == token.into() {
            self.consume();
            true
        } else {
            false
        }
    }

    pub fn consume_if_eof(&mut self) -> bool {
        if self.peek().is(TokenKind::Eof) {
            self.consume();
            true
        } else {
            false
        }
    }
}
