use cool_lexer::{Punctuation, TokenKind};
use derive_more::From;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum BinOpPrecedence {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum BinOp {
    Arithmetic(ArithmeticOp),
    Comparison(ComparisonOp),
    Bitwise(BitwiseOp),
    Logical(LogicalOp),
}

impl BinOp {
    pub fn from_token_kind(token: TokenKind) -> Option<Self> {
        match token {
            TokenKind::Punctuation(punctuation) => Self::from_punctuation(punctuation),
            _ => None,
        }
    }

    pub fn from_punctuation(punctuation: Punctuation) -> Option<Self> {
        let bin_op: Self = match punctuation {
            Punctuation::Plus => ArithmeticOp::Add.into(),
            Punctuation::Minus => ArithmeticOp::Sub.into(),
            Punctuation::Star => ArithmeticOp::Mul.into(),
            Punctuation::Slash => ArithmeticOp::Div.into(),
            Punctuation::Percent => ArithmeticOp::Rem.into(),

            // Relational
            Punctuation::EqEq => ComparisonOp::Eq.into(),
            Punctuation::Ne => ComparisonOp::Ne.into(),
            Punctuation::Lt => ComparisonOp::Lt.into(),
            Punctuation::Le => ComparisonOp::Le.into(),
            Punctuation::Gt => ComparisonOp::Gt.into(),
            Punctuation::Ge => ComparisonOp::Ge.into(),

            // Bitwise
            Punctuation::And => BitwiseOp::And.into(),
            Punctuation::Or => BitwiseOp::Or.into(),
            Punctuation::Caret => BitwiseOp::Xor.into(),
            Punctuation::Shl => BitwiseOp::Shl.into(),
            Punctuation::Shr => BitwiseOp::Shr.into(),

            // Logical
            Punctuation::AndAnd => LogicalOp::And.into(),
            Punctuation::OrOr => LogicalOp::Or.into(),

            // Other
            _ => return None,
        };

        Some(bin_op)
    }

    pub fn precedence(&self) -> BinOpPrecedence {
        match self {
            Self::Arithmetic(arithmetic_op) => {
                match arithmetic_op {
                    ArithmeticOp::Mul => BinOpPrecedence::High,
                    ArithmeticOp::Div => BinOpPrecedence::High,
                    ArithmeticOp::Rem => BinOpPrecedence::High,
                    _ => BinOpPrecedence::Medium,
                }
            }
            Self::Comparison(_) => BinOpPrecedence::Low,
            Self::Bitwise(_) => BinOpPrecedence::Medium,
            Self::Logical(_) => BinOpPrecedence::Medium,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BitwiseOp {
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LogicalOp {
    And,
    Or,
}
