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
    { $($variant:ident => $str:literal,)+ } => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Keyword {
            $($variant,)+
        }

        impl fmt::Display for Keyword {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let display_str = match self {
                    $(Self::$variant => $str,)+
                };

                f.write_str(display_str)
            }
        }

        impl TryFrom<&str> for Keyword {
            type Error = InvalidKeyword;

            fn try_from(str: &str) -> Result<Self, Self::Error> {
                Ok(match str {
                    $($str => Self::$variant,)+
                    _ => return Err(InvalidKeyword),
                })
            }
        }
    };
}

Keyword! {
    Else => "else",
    Enum => "enum",
    False => "false",
    Fn => "fn",
    If => "if",
    Mut => "mut",
    Struct => "struct",
    True => "true",
    While => "while",
}
