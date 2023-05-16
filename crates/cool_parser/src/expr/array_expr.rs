use crate::expr::Expr;
use crate::{LiteralExpr, ParseResult, Parser};
use cool_lexer::tokens::{tk, Token};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ArrayExpr {
    pub span: Span,
    pub elems: Vec<Expr>,
    pub has_trailing_comma: bool,
}

impl Section for ArrayExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct ArrayRepeatExpr {
    pub span: Span,
    pub len: Box<LiteralExpr>,
    pub elem: Box<Expr>,
}

impl Section for ArrayRepeatExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_array_expr(&mut self) -> ParseResult<Expr> {
        let start_token = self.bump_expect(&tk::OPEN_BRACKET)?;

        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACKET) {
            return Ok(ArrayExpr {
                span: start_token.span.to(end_token.span),
                elems: Default::default(),
                has_trailing_comma: false,
            }
            .into());
        }

        let first_elem = self.parse_expr()?;

        let expr: Expr = if self.peek().kind == tk::SEMICOLON {
            match first_elem {
                Expr::Literal(literal_expr) => {
                    self.continue_parse_array_repeat_expr(start_token, literal_expr)?
                        .into()
                }
                _ => return self.peek_error(&[tk::COMMA, tk::CLOSE_BRACKET]),
            }
        } else {
            self.continue_parse_array_expr(start_token, first_elem)?
                .into()
        };

        Ok(expr)
    }

    fn continue_parse_array_expr(
        &mut self,
        start_token: Token,
        expr: Expr,
    ) -> ParseResult<ArrayExpr> {
        let mut elems = vec![expr];

        let (end_token, has_trailing_comma) = loop {
            if self.bump_if_eq(tk::COMMA).is_some() {
                if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACKET) {
                    break (end_token, true);
                } else {
                    elems.push(self.parse_expr()?);
                }
            } else if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACKET) {
                break (end_token, false);
            } else {
                return self.peek_error(&[tk::COMMA, tk::CLOSE_BRACKET]);
            }
        };

        Ok(ArrayExpr {
            span: start_token.span.to(end_token.span),
            elems,
            has_trailing_comma,
        })
    }

    fn continue_parse_array_repeat_expr(
        &mut self,
        start_token: Token,
        literal_expr: LiteralExpr,
    ) -> ParseResult<ArrayRepeatExpr> {
        self.bump_expect(&tk::SEMICOLON)?;
        let elem = self.parse_expr()?;
        let end_token = self.bump_expect(&tk::CLOSE_BRACKET)?;

        Ok(ArrayRepeatExpr {
            span: start_token.span.to(end_token.span),
            len: Box::new(literal_expr),
            elem: Box::new(elem),
        })
    }
}
