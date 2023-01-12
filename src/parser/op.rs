use crate::lexer::Operator;

#[derive(Clone, Debug)]
pub struct InvalidOpConversion;

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Minus,
    Not,
}

impl TryFrom<Operator> for UnaryOp {
    type Error = InvalidOpConversion;

    fn try_from(op: Operator) -> Result<Self, Self::Error> {
        Ok(match op {
            Operator::Minus => Self::Minus,
            Operator::Not => Self::Not,
            _ => return Err(InvalidOpConversion),
        })
    }
}

#[derive(Clone, Debug)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    // Relational
    Eq,
    NotEq,
    Less,
    LessOrEq,
    Greater,
    GreaterOrEq,

    // Logical
    And,
    Or,

    // Bitwise
    BitOr,
    BitAnd,
    Xor,
    Shl,
    Shr,
}

impl TryFrom<Operator> for BinOp {
    type Error = InvalidOpConversion;

    fn try_from(op: Operator) -> Result<Self, Self::Error> {
        Ok(match op {
            // Arithmetic
            Operator::Plus => Self::Add,
            Operator::Minus => Self::Sub,
            Operator::Star => Self::Mul,
            Operator::Slash => Self::Div,
            Operator::Percent => Self::Rem,

            // Relational
            Operator::EqEq => Self::Eq,
            Operator::NotEq => Self::NotEq,
            Operator::Less => Self::Less,
            Operator::LessOrEq => Self::LessOrEq,
            Operator::Greater => Self::Greater,
            Operator::GreaterOrEq => Self::GreaterOrEq,

            // Logical
            Operator::AndAnd => Self::And,
            Operator::OrOr => Self::Or,

            // Bitwise
            Operator::Or => Self::BitOr,
            Operator::And => Self::BitAnd,
            Operator::Caret => Self::Xor,
            Operator::Shl => Self::Shl,
            Operator::Shr => Self::Shr,

            // Error
            _ => return Err(InvalidOpConversion),
        })
    }
}
