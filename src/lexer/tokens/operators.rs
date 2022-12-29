use std::error::Error;
use std::fmt;

const OPERATOR_PARTS: &[u8] = &[
    b'=', b'!', b'<', b'>', b'+', b'-', b'*', b'/', b'%', b'|', b'&', b'~', b'^', b':',
];

#[derive(Clone, Debug)]
pub struct InvalidOperator;

impl Error for InvalidOperator {}

impl fmt::Display for InvalidOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid operator")
    }
}

macro_rules! Operator {
    { $($variant:ident => $bytes:literal,)+ } => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Operator {
            $($variant,)+
        }

        impl fmt::Display for Operator {
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

        impl TryFrom<&[u8]> for Operator {
            type Error = InvalidOperator;

            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                Ok(match bytes {
                    $($bytes => Self::$variant,)+
                    _ => return Err(InvalidOperator),
                })
            }
        }
    };
}

Operator! {
    // Relational
    Equal => b"==",
    NotEqual => b"!=",
    Less => b"<",
    LessOrEqual => b"<=",
    Greater => b">",
    GreaterOrEqual => b">=",

    // Arithmetic
    Plus => b"+",
    Minus => b"-",
    Star => b"*",
    Slash => b"/",
    Modulo => b"%",

    // Logical
    LogicalOr => b"||",
    LogicalAnd => b"&&",

    // Bitwise
    Not => b"!",
    Or => b"|",
    And => b"&",
    Caret => b"^",

    // Bitshift
    ShiftLeft => b"<<",
    ShiftRight => b">>",

    // Assignment
    Declaration => b":",
    Assign => b"=",

    PlusAssign => b"+=",
    MinusAssign => b"-=",
    StarAssign => b"*=",
    SlashAssign => b"/=",
    ModuloAssign => b"%=",

    LogicalOrAssign => b"||=",
    LogicalAndAssign => b"&&=",

    OrAssign => b"|=",
    AndAssign => b"&=",
    XorAssign => b"^=",
    ShiftLeftAssign => b"<<=",
    ShiftRightAssign => b">>=",
}

pub fn is_operator_part(byte: u8) -> bool {
    OPERATOR_PARTS.contains(&byte)
}
