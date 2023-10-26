use crate::{AccessExpr, DerefExpr, Expr, Ident, ParseResult, Parser};
use cool_lexer::{tk, TokenKind};
use cool_span::Section;

impl Parser<'_> {
    pub fn continue_parse_access_or_deref_expr(&mut self, base: Expr) -> ParseResult<Expr> {
        self.bump_expect(&tk::dot)?;
        let token = self.bump();

        let expr = match token.kind {
            TokenKind::Ident(symbol) => {
                AccessExpr {
                    base: Box::new(base),
                    field: Ident {
                        span: token.span,
                        symbol,
                    },
                }
                .into()
            }
            tk::star => {
                DerefExpr {
                    span: base.span().to(token.span),
                    base: Box::new(base),
                }
                .into()
            }
            _ => return self.error(token, &[tk::identifier, tk::star]),
        };

        Ok(expr)
    }
}
