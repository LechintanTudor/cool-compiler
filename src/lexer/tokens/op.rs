use std::error::Error;
use std::fmt;

const OPERATOR_PARTS: &[char] = &[
    '=', '!', '<', '>', '+', '-', '*', '/', '%', '|', '&', '~', '^',
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
    Equal => "==",
    NotEqual => "!=",
    Less => "<",
    LessOrEqual => "<=",
    Greater => ">",
    GreaterOrEqual => ">=",

    // Arithmetic
    Plus => "+",
    Minus => "-",
    Star => "*",
    Slash => "/",
    Modulo => "%",

    // Logical
    LogicalOr => "||",
    LogicalAnd => "&&",

    // Bitwise
    Not => "!",
    Or => "|",
    And => "&",
    Caret => "^",

    // Bitshift
    ShiftLeft => "<<",
    ShiftRight => ">>",

    // Assignment
    Assign => "=",

    PlusAssign => "+=",
    MinusAssign => "-=",
    StarAssign => "*=",
    SlashAssign => "/=",
    ModuloAssign => "%=",

    LogicalOrAssign => "||=",
    LogicalAndAssign => "&&=",

    OrAssign => "|=",
    AndAssign => "&=",
    XorAssign => "^=",
    ShiftLeftAssign => "<<=",
    ShiftRightAssign => ">>=",
}

pub fn is_op_part(char: char) -> bool {
    OPERATOR_PARTS.contains(&char)
}
