use crate::lexer::Tokenizer;
use crate::tokens::Token;

pub struct TokenStream<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
    peeked: Option<Token>,
}

impl<'a> TokenStream<'a> {
    #[inline]
    pub(crate) fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
        Self {
            tokenizer,
            peeked: None,
        }
    }

    pub fn next_lang(&mut self) -> Token {
        if let Some(token) = self.peeked.take() {
            if token.kind.is_lang_part() {
                return token;
            }
        }

        loop {
            let token = self.tokenizer.next_token();

            if token.kind.is_lang_part() {
                return token;
            }
        }
    }

    pub fn next_any(&mut self) -> Token {
        if let Some(token) = self.peeked.take() {
            return token;
        }

        self.tokenizer.next_token()
    }

    pub fn peek_lang(&mut self) -> Token {
        if let Some(token) = self.peeked {
            if token.kind.is_lang_part() {
                return token;
            }
        }

        loop {
            let token = self.tokenizer.next_token();

            if token.kind.is_lang_part() {
                self.peeked = Some(token);
                return token;
            }
        }
    }

    pub fn peek_any(&mut self) -> Token {
        if let Some(token) = self.peeked {
            return token;
        }

        let token = self.tokenizer.next_token();
        self.peeked = Some(token);
        token
    }
}
