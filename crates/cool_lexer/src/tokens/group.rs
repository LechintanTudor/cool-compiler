use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Group {
    Literal,
    StringLiteral,
    Ident,
    Punctuation,
    BinOp,
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Self::Literal => "literal",
            Self::StringLiteral => "string literal",
            Self::Ident => "identifier",
            Self::Punctuation => "punctuation",
            Self::BinOp => "binary operator",
        };

        write!(f, "{}", display_str)
    }
}

pub mod tk {
    use crate::tokens::{Group, TokenKind};

    pub const ANY_LITERAL: TokenKind = TokenKind::Group(Group::Literal);
    pub const ANY_IDENT: TokenKind = TokenKind::Group(Group::Ident);
    pub const STRING_LITERAL: TokenKind = TokenKind::Group(Group::StringLiteral);
    pub const ANY_PUNCTUATION: TokenKind = TokenKind::Group(Group::Punctuation);
    pub const ANY_BIN_OP: TokenKind = TokenKind::Group(Group::BinOp);
}
