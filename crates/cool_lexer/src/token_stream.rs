use crate::{Lexer, Token};

#[derive(Clone, Debug)]
pub struct TokenStream<'a> {
    lexer: Lexer<'a>,
    peeked: Option<Token>,
}

impl<'a> TokenStream<'a> {
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source),
            peeked: None,
        }
    }

    #[inline]
    pub fn next_lang(&mut self) -> Token {
        if let Some(token) = self.peeked.take() {
            if token.kind.is_lang_part() {
                return token;
            }
        }

        loop {
            let token = self.lexer.next_token();

            if token.kind.is_lang_part() {
                return token;
            }
        }
    }

    #[inline]
    pub fn next_any(&mut self) -> Token {
        if let Some(token) = self.peeked.take() {
            return token;
        }

        self.lexer.next_token()
    }

    #[inline]
    #[must_use]
    pub fn peek_lang(&mut self) -> Token {
        if let Some(token) = self.peeked {
            if token.kind.is_lang_part() {
                return token;
            }
        }

        loop {
            let token = self.lexer.next_token();

            if token.kind.is_lang_part() {
                self.peeked = Some(token);
                return token;
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn peek_any(&mut self) -> Token {
        if let Some(token) = self.peeked {
            return token;
        }

        let token = self.lexer.next_token();
        self.peeked = Some(token);
        token
    }
}
