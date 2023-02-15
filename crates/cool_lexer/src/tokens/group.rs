use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Group {
    Literal,
    Ident,
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Self::Literal => "literal",
            Self::Ident => "identifier",
        };

        write!(f, "{}", display_str)
    }
}

pub mod tk {
    use crate::tokens::{Group, TokenKind};

    pub const ANY_LITERAL: TokenKind = TokenKind::Group(Group::Literal);
    pub const ANY_IDENT: TokenKind = TokenKind::Group(Group::Ident);
}
