use crate::{Expr, Ident, ParseResult, Parser};
use cool_lexer::{tk, TokenKind};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AccessExpr {
    pub base: Box<Expr>,
    pub ident: Ident,
}

impl Section for AccessExpr {
    #[inline]
    fn span(&self) -> Span {
        self.base.span().to(self.ident.span())
    }
}

#[derive(Clone, Debug)]
pub struct DerefExpr {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl Section for DerefExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn continue_parse_access_expr(&mut self, base: Box<Expr>) -> ParseResult<Expr> {
        self.bump_expect(&tk::DOT)?;
        let next_token = self.bump();

        let expr: Expr = match next_token.kind {
            TokenKind::Ident(symbol) => {
                AccessExpr {
                    base,
                    ident: Ident {
                        span: next_token.span,
                        symbol,
                    },
                }
                .into()
            }
            TokenKind::Literal(literal) => {
                AccessExpr {
                    base,
                    ident: Ident {
                        span: next_token.span,
                        symbol: literal.symbol,
                    },
                }
                .into()
            }
            tk::STAR => {
                DerefExpr {
                    span: base.span().to(next_token.span),
                    expr: base,
                }
                .into()
            }
            _ => self.error(next_token, &[tk::DIAG_IDENT, tk::STAR])?,
        };

        Ok(expr)
    }
}
