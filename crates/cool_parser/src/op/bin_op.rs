use cool_lexer::tokens::{Punctuation, TokenKind};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum BinOpPrecedence {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BinOp {
    Arithmetic(ArithmeticBinOp),
    Relational(RelationalBinOp),
    Bitwise(BitwiseBinOp),
    LogicalBinOp(LogicalBinOp),
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
            Punctuation::Plus => ArithmeticBinOp::Add.into(),
            Punctuation::Minus => ArithmeticBinOp::Sub.into(),
            Punctuation::Star => ArithmeticBinOp::Mul.into(),
            Punctuation::Slash => ArithmeticBinOp::Div.into(),
            Punctuation::Percent => ArithmeticBinOp::Rem.into(),

            // Relational
            Punctuation::EqEq => RelationalBinOp::Eq.into(),
            Punctuation::Ne => RelationalBinOp::Ne.into(),
            Punctuation::Lt => RelationalBinOp::Lt.into(),
            Punctuation::Le => RelationalBinOp::Le.into(),
            Punctuation::Gt => RelationalBinOp::Gt.into(),
            Punctuation::Ge => RelationalBinOp::Ge.into(),

            // Bitwise
            Punctuation::And => BitwiseBinOp::And.into(),
            Punctuation::Or => BitwiseBinOp::Or.into(),
            Punctuation::Caret => BitwiseBinOp::Xor.into(),
            Punctuation::Shl => BitwiseBinOp::Shl.into(),
            Punctuation::Shr => BitwiseBinOp::Shr.into(),

            // Logical
            Punctuation::AndAnd => LogicalBinOp::And.into(),
            Punctuation::OrOr => LogicalBinOp::Or.into(),

            // Other
            _ => return None,
        };

        Some(bin_op)
    }

    pub fn precedence(&self) -> BinOpPrecedence {
        match self {
            Self::Arithmetic(arithmetic_op) => match arithmetic_op {
                ArithmeticBinOp::Mul => BinOpPrecedence::High,
                ArithmeticBinOp::Div => BinOpPrecedence::High,
                ArithmeticBinOp::Rem => BinOpPrecedence::High,
                _ => BinOpPrecedence::Medium,
            },
            Self::Relational(_) => BinOpPrecedence::Low,
            Self::Bitwise(_) => BinOpPrecedence::Medium,
            Self::LogicalBinOp(_) => BinOpPrecedence::Medium,
        }
    }
}

impl From<ArithmeticBinOp> for BinOp {
    #[inline]
    fn from(op: ArithmeticBinOp) -> Self {
        Self::Arithmetic(op)
    }
}

impl From<RelationalBinOp> for BinOp {
    #[inline]
    fn from(op: RelationalBinOp) -> Self {
        Self::Relational(op)
    }
}

impl From<BitwiseBinOp> for BinOp {
    #[inline]
    fn from(op: BitwiseBinOp) -> Self {
        Self::Bitwise(op)
    }
}

impl From<LogicalBinOp> for BinOp {
    #[inline]
    fn from(op: LogicalBinOp) -> Self {
        Self::LogicalBinOp(op)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ArithmeticBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RelationalBinOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BitwiseBinOp {
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LogicalBinOp {
    And,
    Or,
}
