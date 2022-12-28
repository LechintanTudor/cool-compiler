use std::error::Error;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Separator {
    Semicolon,
    OpenParanthesis,
    ClosedParanthesis,
    OpenBracket,
    ClosedBracket,
    OpenBrace,
    ClosedBrace,
}

#[derive(Debug)]
pub struct InvalidSeparator;

impl Error for InvalidSeparator {}

impl fmt::Display for InvalidSeparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid separator")
    }
}

impl TryFrom<u8> for Separator {
    type Error = InvalidSeparator;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            b';' => Self::Semicolon,
            b'(' => Self::OpenParanthesis,
            b')' => Self::ClosedParanthesis,
            b'[' => Self::OpenBracket,
            b']' => Self::ClosedBracket,
            b'{' => Self::OpenBrace,
            b'}' => Self::ClosedBrace,
            _ => return Err(InvalidSeparator),
        })
    }
}
