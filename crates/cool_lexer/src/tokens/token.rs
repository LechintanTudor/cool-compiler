use crate::tokens::TokenKind;
use cool_span::Span;

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_size() {
        assert!(std::mem::size_of::<Token>() <= 16);
    }
}
