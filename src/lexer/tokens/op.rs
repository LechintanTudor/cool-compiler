use std::error::Error;
use std::fmt;

const OPERATOR_PARTS: &[char] = &[
    '=', '!', '<', '>', '+', '-', '*', '/', '%', '|', '&', '~', '^', ':',
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
    { $($variant:ident => $str:literal,)+ } => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Operator {
            $($variant,)+
        }

        impl Operator {
            pub fn len(&self) -> u32 {
                let len = match self {
                    $(Self::$variant => $str.len(),)+
                };

                len as u32
            }
        }

        impl fmt::Display for Operator {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let display_str = match self {
                    $(Self::$variant => $str,)+
                };

                f.write_str(display_str)
            }
        }

        impl TryFrom<&str> for Operator {
            type Error = InvalidOperator;

            fn try_from(str: &str) -> Result<Self, Self::Error> {
                Ok(match str {
                    $($str => Self::$variant,)+
                    _ => return Err(InvalidOperator),
                })
            }
        }
    };
}

Operator! {
    // Relational
    Eq => "==",
    NotEq => "!=",
    Less => "<",
    LessOrEq => "<=",
    Greater => ">",
    GreaterOrEq => ">=",

    // Arithmetic
    Plus => "+",
    Minus => "-",
    Star => "*",
    Slash => "/",
    Percent => "%",

    // Logical
    LogicalOr => "||",
    LogicalAnd => "&&",

    // Bitwise
    Not => "!",
    Or => "|",
    And => "&",
    Caret => "^",

    // Bitshift
    Shl => "<<",
    Shr => ">>",

    // Assignment
    Assign => "=",

    PlusAssign => "+=",
    MinusAssign => "-=",
    StarAssign => "*=",
    SlashAssign => "/=",
    PercentAssign => "%=",

    LogicalOrAssign => "||=",
    LogicalAndAssign => "&&=",

    OrAssign => "|=",
    AndAssign => "&=",
    XorAssign => "^=",
    ShlAssign => "<<=",
    ShrAssign => ">>=",

    // Access
    Dot => ".",
    Arrow => "->",
    Colon => ":",
}

pub fn is_operator_part(char: char) -> bool {
    OPERATOR_PARTS.contains(&char)
}
