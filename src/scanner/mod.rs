pub mod identifier;
pub mod token;
pub mod token_builder;

use crate::scanner::token::{Separator, Token};
use crate::scanner::token_builder::TokenBuilder;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ScannerError {
    TokenConversionError,
    InvalidTokenCharError { which: u8 },
}

impl Error for ScannerError {}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenConversionError => write!(f, "characters cannot be converted into a token"),
            Self::InvalidTokenCharError { which } => {
                write!(f, "\\{} is not a valid token character", which)
            }
        }
    }
}

pub fn tokenize(source: &[u8]) -> Result<Vec<Token>, ScannerError> {
    let mut tokens = Vec::<Token>::new();
    let mut token_builder = TokenBuilder::default();

    for (i, &new_char) in source.iter().enumerate() {
        if new_char.is_ascii_whitespace() {
            consume_token(&mut tokens, &mut token_builder)?;
        } else if let Ok(separator) = Separator::try_from(new_char) {
            consume_token(&mut tokens, &mut token_builder)?;
            tokens.push(separator.into());
        } else {
            token_builder
                .add_char(i, new_char)
                .map_err(|_| ScannerError::InvalidTokenCharError { which: new_char })?;
        }
    }

    consume_token(&mut tokens, &mut token_builder)?;
    Ok(tokens)
}

fn consume_token(
    tokens: &mut Vec<Token>,
    token_builder: &mut TokenBuilder,
) -> Result<(), ScannerError> {
    let token = token_builder
        .consume()
        .map_err(|_| ScannerError::TokenConversionError)?;

    if let Some(token) = token {
        tokens.push(token);
    }

    Ok(())
}
