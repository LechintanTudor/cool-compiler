use crate::InvalidOp;
use cool_lexer::Punct;
use derive_more::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
pub enum AssignOp {
    #[display("=")]
    Eq,

    #[display("+=")]
    Add,

    #[display("-=")]
    Sub,

    #[display("*=")]
    Mul,

    #[display("/=")]
    Div,

    #[display("%=")]
    Rem,

    #[display("|=")]
    Or,

    #[display("&=")]
    And,

    #[display("^=")]
    Xor,

    #[display("<<=")]
    Shl,

    #[display(">>=")]
    Shr,
}

impl TryFrom<Punct> for AssignOp {
    type Error = InvalidOp;

    fn try_from(punct: Punct) -> Result<Self, Self::Error> {
        let op = match punct {
            Punct::Eq => AssignOp::Eq,
            Punct::PlusEq => AssignOp::Add,
            Punct::MinusEq => AssignOp::Sub,
            Punct::StarEq => AssignOp::Mul,
            Punct::SlashEq => AssignOp::Div,
            Punct::PerecentEq => AssignOp::Rem,
            Punct::OrEq => AssignOp::Or,
            Punct::AndEq => AssignOp::And,
            Punct::CaretEq => AssignOp::Xor,
            Punct::ShlEq => AssignOp::Shl,
            Punct::ShrEq => AssignOp::Shr,
            _ => return Err(InvalidOp),
        };

        Ok(op)
    }
}
