use cool_lexer::tokens::{Punctuation, TokenKind};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UnaryOp {
    Minus,
    Not,
    Addr,
}

impl UnaryOp {
    pub fn from_token_kind(token: TokenKind) -> Option<Self> {
        match token {
            TokenKind::Punctuation(punctuation) => Self::from_punctuation(punctuation),
            _ => None,
        }
    }

    pub fn from_punctuation(punctuation: Punctuation) -> Option<Self> {
        let unary_op = match punctuation {
            Punctuation::Minus => Self::Minus,
            Punctuation::Not => Self::Not,
            Punctuation::And => Self::Addr,
            _ => return None,
        };

        Some(unary_op)
    }
}
