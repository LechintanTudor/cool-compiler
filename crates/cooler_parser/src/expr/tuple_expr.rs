use crate::{parse_error, ExprId, ParseResult, Parser};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ParenExpr {
    pub span: Span,
    pub inner: ExprId,
}

#[derive(Clone, Section, Debug)]
pub struct TupleExpr {
    pub span: Span,
    pub elems: SmallVec<ExprId, 4>,
    pub has_trailing_comma: bool,
}

impl Parser<'_> {
    pub fn parse_paren_or_tuple_expr(&mut self) -> ParseResult<ExprId> {
        let open_paren = self.bump_expect(&tk::open_paren)?;
        let mut elems = SmallVec::new();

        if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
            return Ok(self.add_expr(TupleExpr {
                span: open_paren.span.to(close_paren.span),
                elems,
                has_trailing_comma: false,
            }));
        }

        let (close_paren, has_trailing_comma) = loop {
            let elem = self.parse_expr()?;

            if elems.is_empty() {
                if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                    return Ok(self.add_expr(ParenExpr {
                        span: open_paren.span.to(close_paren.span),
                        inner: elem,
                    }));
                }
            }

            elems.push(elem);
            let token = self.bump();

            match token.kind {
                tk::comma => {
                    if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                        break (close_paren, true);
                    }
                }
                tk::close_paren => break (token, false),
                _ => return parse_error(token, &[tk::comma, tk::close_paren]),
            }
        };

        Ok(self.add_expr(TupleExpr {
            span: open_paren.span.to(close_paren.span),
            elems,
            has_trailing_comma,
        }))
    }
}
