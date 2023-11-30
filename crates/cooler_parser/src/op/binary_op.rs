use crate::op::define_op;

define_op! {
    BinaryOp {
        // Arithmetic
        Add => "+" from Plus,
        Sub => "-" from Minus,
        Mul => "*" from Star,
        Div => "/" from Slash,
        Rem => "%" from Percent,

        // Relational
        Eq => "==" from EqEq,
        Ne => "!=" from Ne,
        Lt => "<" from Lt,
        Le => "<=" from Le,
        Gt => ">" from Gt,
        Ge => ">=" from Ge,

        // Bitwise
        And => "&" from And,
        Or => "|" from Or,
        Xor => "^" from Caret,
        Shl => "<<" from Shl,
        Shr => ">>" from Shr,

        LogicalAnd => "&&" from AndAnd,
        LogicalOr => "||" from OrOr,
    }
}

impl BinaryOp {
    #[must_use]
    pub const fn precedence(&self) -> u32 {
        match self {
            Self::Add => 2,
            Self::Sub => 2,
            Self::Mul => 3,
            Self::Div => 3,
            Self::Rem => 3,

            Self::Eq => 1,
            Self::Ne => 1,
            Self::Lt => 1,
            Self::Le => 1,
            Self::Gt => 1,
            Self::Ge => 1,

            Self::And => 2,
            Self::Or => 2,
            Self::Xor => 2,
            Self::Shl => 2,
            Self::Shr => 2,

            Self::LogicalAnd => 0,
            Self::LogicalOr => 0,
        }
    }
}
