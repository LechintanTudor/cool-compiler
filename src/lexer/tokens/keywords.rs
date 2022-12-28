use std::error::Error;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Keyword {
    Mut,
    True,
    False,
    If,
    Else,
    While,
    Fn,
    Struct,
    Enum,
}

#[derive(Debug)]
pub struct InvalidKeyword;

impl Error for InvalidKeyword {}

impl fmt::Display for InvalidKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid keyword")
    }
}

impl TryFrom<&[u8]> for Keyword {
    type Error = InvalidKeyword;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(match bytes {
            b"mut" => Self::Mut,
            b"true" => Self::True,
            b"false" => Self::False,
            b"if" => Self::If,
            b"else" => Self::Else,
            b"while" => Self::While,
            b"fn" => Self::Fn,
            b"struct" => Self::Struct,
            b"enum" => Self::Enum,
            _ => return Err(InvalidKeyword),
        })
    }
}
