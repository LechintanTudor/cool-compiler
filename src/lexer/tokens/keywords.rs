use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct InvalidKeyword;

impl Error for InvalidKeyword {}

impl fmt::Display for InvalidKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid keyword")
    }
}

macro_rules! Keyword {
    { $($variant:ident => $bytes:literal,)+ } => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Keyword {
            $($variant,)+
        }

        impl fmt::Display for Keyword {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let display_bytes: &'static [u8] = match self {
                    $(Self::$variant => $bytes,)+
                };

                let display_str = unsafe {
                    std::str::from_utf8_unchecked(display_bytes)
                };

                f.write_str(display_str)
            }
        }

        impl TryFrom<&[u8]> for Keyword {
            type Error = InvalidKeyword;

            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                Ok(match bytes {
                    $($bytes => Self::$variant,)+
                    _ => return Err(InvalidKeyword),
                })
            }
        }
    };
}

Keyword! {
    Bool => b"bool",
    Else => b"else",
    Enum => b"enum",
    False => b"false",
    Fn => b"fn",
    If => b"if",
    Mut => b"mut",
    Struct => b"struct",
    True => b"true",
    While => b"while",
}
