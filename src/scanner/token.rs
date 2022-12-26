use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TokenConversionError;

impl Error for TokenConversionError {}

impl fmt::Display for TokenConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "characters cannot be converted into a token")
    }
}

#[derive(Debug)]
pub enum Token {
    ReservedWord(ReservedWord),
    Separator(Separator),
    Wildcard,
    Identifier { start: usize, end: usize },
    Integer { start: usize, end: usize },
    Float { start: usize, end: usize },
    String { start: usize, end: usize },
}

impl From<ReservedWord> for Token {
    fn from(reserved_word: ReservedWord) -> Self {
        Self::ReservedWord(reserved_word)
    }
}

impl From<Separator> for Token {
    fn from(separator: Separator) -> Self {
        Self::Separator(separator)
    }
}

#[derive(Debug)]
pub enum ReservedWord {
    Mut,
    If,
    Else,
    While,
    For,
}

impl fmt::Display for ReservedWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mut => write!(f, "mut"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::While => write!(f, "while"),
            Self::For => write!(f, "for"),
        }
    }
}

impl TryFrom<&[u8]> for ReservedWord {
    type Error = TokenConversionError;

    fn try_from(chars: &[u8]) -> Result<Self, Self::Error> {
        Ok(match chars {
            b"mut" => Self::Mut,
            b"if" => Self::If,
            b"else" => Self::Else,
            b"while" => Self::While,
            b"for" => Self::For,
            _ => return Err(TokenConversionError),
        })
    }
}

#[derive(Debug)]
pub enum Separator {
    DoubleQuotes,
    Semicolon,
}

impl TryFrom<u8> for Separator {
    type Error = TokenConversionError;

    fn try_from(char: u8) -> Result<Self, Self::Error> {
        Ok(match char {
            b'"' => Self::DoubleQuotes,
            b';' => Self::Semicolon,
            _ => return Err(TokenConversionError),
        })
    }
}
