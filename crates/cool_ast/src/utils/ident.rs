use crate::{parse_error, ParseResult, Parser};
use cool_collections::smallvec::smallvec;
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::{tk, Symbol, TokenKind};
use cool_span::{Section, Span};
use derive_more::Constructor;

#[derive(Clone, Copy, Section, Constructor, Debug)]
pub struct Ident {
    pub span: Span,
    pub symbol: Symbol,
}

#[derive(Clone, Debug)]
pub struct IdentPath {
    pub idents: SmallVec<Ident, 2>,
}

impl Section for IdentPath {
    #[inline]
    fn span(&self) -> Span {
        match self.idents.as_slice() {
            [] => Span::EMPTY,
            [ident] => ident.span,
            [first, .., last] => first.span.to(last.span),
        }
    }
}

impl Parser<'_> {
    pub fn parse_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();

        let TokenKind::Ident(symbol) = token.kind else {
            return parse_error(token, &[tk::identifier]);
        };

        Ok(Ident {
            span: token.span,
            symbol,
        })
    }

    pub fn parse_path_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();

        let symbol = match token.kind {
            TokenKind::Ident(symbol) => symbol,
            TokenKind::Keyword(symbol) if symbol.is_path_keyword() => symbol,
            _ => {
                return parse_error(
                    token,
                    &[tk::identifier, tk::kw_crate, tk::kw_super, tk::kw_self],
                );
            }
        };

        Ok(Ident {
            span: token.span,
            symbol,
        })
    }

    pub fn parse_ident_path(&mut self) -> ParseResult<IdentPath> {
        let mut idents = smallvec![self.parse_path_ident()?];

        while self.bump_if_eq(tk::dot).is_some() {
            idents.push(self.parse_path_ident()?);
        }

        Ok(IdentPath { idents })
    }
}
