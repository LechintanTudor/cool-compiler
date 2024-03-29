use crate::{ParseResult, Parser};
use cool_lexer::{sym, tk, Symbol, TokenKind};
use cool_span::{Section, Span};

#[derive(Clone, Copy, Debug)]
pub struct Ident {
    pub span: Span,
    pub symbol: Symbol,
}

impl Section for Ident {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();
        let TokenKind::Ident(symbol) = token.kind else {
            return self.error(token, &[tk::DIAG_IDENT]);
        };

        Ok(Ident {
            symbol,
            span: token.span,
        })
    }

    pub fn parse_access_path_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();

        let ident = match token.kind {
            TokenKind::Ident(symbol) => {
                Ident {
                    span: token.span,
                    symbol,
                }
            }
            TokenKind::Literal(literal) => {
                Ident {
                    span: token.span,
                    symbol: literal.symbol,
                }
            }
            _ => return self.error(token, &[tk::DIAG_IDENT]),
        };

        Ok(ident)
    }

    pub fn parse_path_ident(&mut self) -> ParseResult<Ident> {
        let token = self.bump();
        let symbol = match token.kind {
            TokenKind::Ident(symbol) => symbol,
            tk::KW_CRATE => sym::KW_CRATE,
            tk::KW_SUPER => sym::KW_SUPER,
            tk::KW_SELF => sym::KW_SELF,
            _ => {
                self.error(
                    token,
                    &[tk::DIAG_IDENT, tk::KW_CRATE, tk::KW_SUPER, tk::KW_SELF],
                )?
            }
        };

        Ok(Ident {
            span: token.span,
            symbol,
        })
    }
}
