use crate::scanner::token::{ReservedWord, Token, TokenConversionError};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidTokenCharError;

impl Error for InvalidTokenCharError {}

impl fmt::Display for InvalidTokenCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "token builder encountered an invalid char")
    }
}

#[derive(Default)]
pub struct TokenBuilder {
    start_index: usize,
    word: Vec<u8>,
}

impl TokenBuilder {
    pub fn add_char(&mut self, index: usize, new_char: u8) -> Result<(), InvalidTokenCharError> {
        if !is_valid_token_char(new_char) {
            return Err(InvalidTokenCharError);
        }

        if self.word.is_empty() {
            self.start_index = index;
        }

        self.word.push(new_char);
        Ok(())
    }

    pub fn consume(&mut self) -> Result<Option<Token>, TokenConversionError> {
        let token = if self.word.is_empty() {
            None
        } else if let Ok(reserved_word) = ReservedWord::try_from(self.word.as_slice()) {
            Some(Token::ReservedWord(reserved_word))
        } else if is_wildcard(&self.word) {
            Some(Token::Wildcard)
        } else if is_identifier(&self.word) {
            Some(Token::Identifier {
                start: self.start_index,
                end: self.start_index + self.word.len(),
            })
        } else {
            return Err(TokenConversionError);
        };

        self.start_index = 0;
        self.word.clear();
        Ok(token)
    }
}

fn is_valid_token_char(new_char: u8) -> bool {
    new_char == b'_' || new_char.is_ascii_alphanumeric()
}

fn is_wildcard(word: &[u8]) -> bool {
    word == b"_"
}

fn is_identifier(word: &[u8]) -> bool {
    word[0] == b'_' || word[0].is_ascii_alphabetic()
}
