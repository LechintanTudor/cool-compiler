use crate::parse_tree::ParseTree;
use crate::{ParseResult, Parser, UnexpectedToken};
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Copy, Debug)]
pub struct Ident {
    pub symbol: Symbol,
    pub span: Span,
}

impl ParseTree for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();

        let TokenKind::Ident(symbol) = token.kind else {
            return Err(UnexpectedToken {
                found: token,
                expected: &[tk::ANY_IDENT],
            })?;
        };

        Ok(Ident {
            symbol,
            span: token.span,
        })
    }
}
