use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct InvalidSeparator;

impl Error for InvalidSeparator {}

impl fmt::Display for InvalidSeparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid separator")
    }
}

macro_rules! Separator {
    { $($variant:ident => $byte:literal,)+ } => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Separator {
            $($variant,)+
        }
        
        impl fmt::Display for Separator {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let display_bytes = &[match self {
                    $(Self::$variant => $byte,)+
                }];

                let display_str = unsafe {
                    std::str::from_utf8_unchecked(display_bytes)
                };

                f.write_str(display_str)
            }
        }
        
        impl TryFrom<u8> for Separator {
            type Error = InvalidSeparator;

            fn try_from(byte: u8) -> Result<Self, Self::Error> {
                Ok(match byte {
                    $($byte => Self::$variant,)+
                    _ => return Err(InvalidSeparator),
                })
            }
        }
    };
}


Separator! {
    Semicolon => b';',
    OpenParanthesis => b'(',
    ClosedParanthesis => b')',
    OpenBracket => b'[',
    ClosedBracket => b']',
    OpenBrace => b'{',
    ClosedBrace => b'}',
}
