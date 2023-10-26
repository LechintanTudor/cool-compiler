use crate::{Expr, ParenExpr, ParseResult, Parser, TupleExpr};
use cool_lexer::tk;

impl Parser<'_> {
    pub fn parse_paren_or_tuple_expr(&mut self) -> ParseResult<Expr> {
        let open_paren = self.bump_expect(&tk::open_paren)?;
        let mut elems = Vec::<Expr>::new();

        let (close_paren, has_trailing_comma) =
            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                (close_paren, false)
            } else {
                loop {
                    elems.push(self.parse_expr()?);
                    let token = self.bump();

                    match token.kind {
                        tk::comma => {
                            if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                                break (close_paren, true);
                            }
                        }
                        tk::close_paren => break (token, false),
                        _ => return self.error(token, &[tk::comma, tk::close_paren]),
                    }
                }
            };

        let expr = if elems.len() == 1 && !has_trailing_comma {
            ParenExpr {
                span: open_paren.span.to(close_paren.span),
                expr: Box::new(elems.pop().unwrap()),
            }
            .into()
        } else {
            TupleExpr {
                span: open_paren.span.to(close_paren.span),
                elems,
                has_trailing_comma,
            }
            .into()
        };

        Ok(expr)
    }
}
