use crate::{ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::{tk, Symbol, TokenKind};
use cool_span::Span;

#[derive(Clone, Copy, Section, Debug)]
pub struct Ident {
    pub span: Span,
    pub symbol: Symbol,
}

impl Parser<'_> {
    pub fn parse_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();

        let TokenKind::Ident(symbol) = token.kind else {
            return self.error(token, &[tk::identifier]);
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
                return self.error(
                    token,
                    &[tk::identifier, tk::kw_crate, tk::kw_super, tk::kw_self],
                )
            }
        };

        Ok(Ident {
            span: token.span,
            symbol,
        })
    }
}
