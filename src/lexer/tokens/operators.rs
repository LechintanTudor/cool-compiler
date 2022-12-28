use std::error::Error;
use std::fmt;

const OPERATOR_PARTS: &[u8] = &[
    b'=', b'!', b'<', b'>', b'+', b'-', b'*', b'/', b'%', b'|', b'&', b'~', b'^', b':',
];

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    // Relational
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,

    // Arithmetic
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,

    // Logical
    Not,
    Or,
    And,
    OrAssign,
    AndAssign,

    // Bitwise
    BitwiseNot,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,

    // Bitshift
    BitshitLeft,
    BitshiftRight,

    // Assignment
    Declaration,
    Assignment,
    AdditionAssignment,
    SubtractionAssignment,
    MultiplicationAssignment,
    DivisionAssignment,
    RemainderAssignment,
    BitwiseOrAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitshiftLeftAssign,
    BitshiftRightAssign,
}

#[derive(Debug)]
pub struct InvalidOperator;

impl Error for InvalidOperator {}

impl fmt::Display for InvalidOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid operator")
    }
}

impl TryFrom<&[u8]> for Operator {
    type Error = InvalidOperator;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        Ok(match buffer {
            // Relational
            b"==" => Self::Equal,
            b"!=" => Self::NotEqual,
            b"<" => Self::Less,
            b"<=" => Self::LessOrEqual,
            b">" => Self::Greater,
            b">=" => Self::GreaterOrEqual,

            // Arithmetic
            b"+" => Self::Addition,
            b"-" => Self::Subtraction,
            b"*" => Self::Multiplication,
            b"/" => Self::Division,
            b"%" => Self::Remainder,

            // Logical
            b"!" => Self::Not,
            b"||" => Self::Or,
            b"&&" => Self::And,
            b"||=" => Self::OrAssign,
            b"&&=" => Self::AndAssign,

            // Bitwise
            b"~" => Self::BitwiseNot,
            b"|" => Self::BitwiseOr,
            b"&" => Self::BitwiseAnd,
            b"^" => Self::BitwiseXor,

            // Bitshift
            b"<<" => Self::BitshitLeft,
            b">>" => Self::BitshiftRight,

            // Assignment
            b":" => Self::Declaration,
            b"=" => Self::Assignment,
            b"+=" => Self::AdditionAssignment,
            b"-=" => Self::SubtractionAssignment,
            b"*=" => Self::MultiplicationAssignment,
            b"/=" => Self::DivisionAssignment,
            b"%=" => Self::RemainderAssignment,
            b"|=" => Self::BitwiseOrAssign,
            b"&=" => Self::BitwiseAndAssign,
            b"^=" => Self::BitwiseXorAssign,
            b"<<=" => Self::BitshiftLeftAssign,
            b">>=" => Self::BitshiftRightAssign,

            _ => return Err(InvalidOperator),
        })
    }
}

pub fn is_operator_part(byte: u8) -> bool {
    OPERATOR_PARTS.contains(&byte)
}
