use cool_lexer::tokens::{Punctuation, TokenKind};

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
    pub fn from_token_kind(token: TokenKind) -> Option<Self> {
        match token {
            TokenKind::Punctuation(punctuation) => Self::from_punctuation(punctuation),
            _ => None,
        }
    }

    pub fn from_punctuation(punctuation: Punctuation) -> Option<Self> {
        let assign_op = match punctuation {
            Punctuation::Eq => Self::Eq,
            Punctuation::PlusEq => Self::Add,
            Punctuation::MinusEq => Self::Sub,
            Punctuation::StarEq => Self::Mul,
            Punctuation::SlashEq => Self::Div,
            Punctuation::PerecentEq => Self::Rem,
            Punctuation::OrEq => Self::Or,
            Punctuation::AndEq => Self::And,
            Punctuation::CaretEq => Self::Xor,
            Punctuation::ShlEq => Self::Shl,
            Punctuation::ShrEq => Self::Shr,
            _ => return None,
        };

        Some(assign_op)
    }
}
