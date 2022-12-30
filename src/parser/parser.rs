use crate::lexer::{IdentifierTable, LiteralTable, Token};
use crate::parser::ast::RootAst;
use std::iter::Peekable;
use std::slice::Iter as SliceIter;

pub struct Parser<'a> {
    tokens: Peekable<SliceIter<'a, Token>>,
    identifier_table: &'a IdentifierTable,
    literal_table: &'a LiteralTable,
}

impl<'a> Parser<'a> {
    pub fn new(
        tokens: &'a [Token],
        identifier_table: &'a IdentifierTable,
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

    pub fn next(&mut self) -> Token {
        self.tokens.next().copied().unwrap_or(Token::Eof)
    }

    pub fn next_and<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(Token) -> bool,
    {
        f(self.next())
    }

    pub fn peek(&mut self) -> Token {
        self.tokens
            .peek()
            .map(|&&token| token)
            .unwrap_or(Token::Eof)
    }

    pub fn consume(&mut self) {
        self.next();
    }

    pub fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(Token) -> bool,
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
        T: Into<Token>,
    {
        if self.peek() == token.into() {
            self.consume();
            true
        } else {
            false
        }
    }

    pub fn consume_if_eof(&mut self) -> bool {
        if self.peek().is(Token::Eof) {
            self.consume();
            true
        } else {
            false
        }
    }
}
