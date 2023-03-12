use crate::parse_tree::ParseTree;
use crate::{ParseResult, Parser, UnexpectedToken};
use cool_lexer::symbols::{sym, Symbol};
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Copy, Debug)]
pub struct Ident {
    pub span: Span,
    pub symbol: Symbol,
}

impl ParseTree for Ident {
    #[inline]
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

    pub fn parse_path_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();
        let symbol = match token.kind {
            TokenKind::Ident(symbol) => symbol,
            tk::KW_CRATE => sym::KW_CRATE,
            tk::KW_SUPER => sym::KW_SUPER,
            tk::KW_SELF => sym::KW_SELF,
            _ => Err(UnexpectedToken {
                found: token,
                expected: &[tk::ANY_IDENT, tk::KW_CRATE, tk::KW_SUPER, tk::KW_SELF],
            })?,
        };

        Ok(Ident {
            span: token.span,
            symbol,
        })
    }
}
