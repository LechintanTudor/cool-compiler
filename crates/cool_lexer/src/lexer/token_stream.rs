use crate::lexer::{LineOffsets, Tokenizer};
use crate::tokens::{Token, TokenKind};

pub struct TokenStream<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
    only_lang_tokens: bool,
    emitted_eof: bool,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>, only_lang_tokens: bool) -> Self {
        Self {
            tokenizer,
            only_lang_tokens,
            emitted_eof: false,
        }
    }

    #[inline]
    pub fn line_offsets(&self) -> &LineOffsets {
        self.tokenizer.line_offsets()
    }
}

impl Iterator for TokenStream<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.emitted_eof {
            return None;
        }

        let token = loop {
            let token = self.tokenizer.next_token();

            if token.kind == TokenKind::Eof {
                self.emitted_eof = true;
            }

            if self.only_lang_tokens {
                if token.kind.is_lang_part() {
                    break token;
                }
            } else {
                break token;
            }
        };

        Some(token)
    }
}
