use cool_parser::AssignOp as ParserAssignOp;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AssignOp {
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Or,
    And,
    Xor,
    Shl,
    Shr,
}

impl AssignOp {
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::Add => "+=",
            Self::Sub => "-=",
            Self::Mul => "*=",
            Self::Div => "/=",
            Self::Rem => "%=",
            Self::Or => "|=",
            Self::And => "&=",
            Self::Xor => "^=",
            Self::Shl => "<<=",
            Self::Shr => ">>=",
        }
    }
}

impl From<ParserAssignOp> for AssignOp {
    #[inline]
    fn from(op: ParserAssignOp) -> Self {
        match op {
            ParserAssignOp::Eq => Self::Eq,
            ParserAssignOp::Add => Self::Add,
            ParserAssignOp::Sub => Self::Sub,
            ParserAssignOp::Mul => Self::Mul,
            ParserAssignOp::Div => Self::Div,
            ParserAssignOp::Rem => Self::Rem,
            ParserAssignOp::Or => Self::Or,
            ParserAssignOp::And => Self::And,
            ParserAssignOp::Xor => Self::Xor,
            ParserAssignOp::Shl => Self::Shl,
            ParserAssignOp::Shr => Self::Shr,
        }
    }
}

impl fmt::Display for AssignOp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
