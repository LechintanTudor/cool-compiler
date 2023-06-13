use crate::expr::Expr;
use crate::{ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ParenExpr {
    pub span: Span,
    pub inner: Box<Expr>,
}

impl Section for ParenExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct TupleExpr {
    pub span: Span,
    pub elems: Vec<Expr>,
    pub has_trailing_comma: bool,
}

impl Section for TupleExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_tuple_expr(&mut self) -> ParseResult<Expr> {
        let open_paren = self.bump_expect(&tk::OPEN_PAREN)?;

        if let Some(close_paren) = self.bump_if_eq(tk::CLOSE_PAREN) {
            return Ok(Expr::Tuple(TupleExpr {
                span: open_paren.span.to(close_paren.span),
                elems: vec![],
                has_trailing_comma: false,
            }));
        }

        let first_elem = self.parse_expr()?;

        if let Some(close_paren) = self.bump_if_eq(tk::CLOSE_PAREN) {
            return Ok(Expr::Paren(ParenExpr {
                span: open_paren.span.to(close_paren.span),
                inner: Box::new(first_elem),
            }));
        }

        let mut elems = vec![first_elem];

        let (close_paren, has_trailing_comma) = loop {
            self.bump_expect(&tk::COMMA)?;

            if let Some(close_paren) = self.bump_if_eq(tk::CLOSE_PAREN) {
                break (close_paren, true);
            }

            elems.push(self.parse_expr()?);

            if let Some(close_paren) = self.bump_if_eq(tk::CLOSE_PAREN) {
                break (close_paren, false);
            }
        };

        Ok(Expr::Tuple(TupleExpr {
            span: open_paren.span.to(close_paren.span),
            elems,
            has_trailing_comma,
        }))
    }
}
