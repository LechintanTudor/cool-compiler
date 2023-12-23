use crate::{parse_error, ExprId, ParseResult, Parser};
use cool_collections::smallvec::smallvec;
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ArrayExpr {
    pub span: Span,
    pub elems: SmallVec<ExprId, 4>,
    pub has_trailing_comma: bool,
}

#[derive(Clone, Section, Debug)]
pub struct ArrayRepeatExpr {
    pub span: Span,
    pub len: ExprId,
    pub elem: ExprId,
}

impl Parser<'_> {
    pub fn parse_array_or_array_repeat_expr(&mut self) -> ParseResult<ExprId> {
        let open_bracket = self.bump_expect(&tk::open_bracket)?;

        if let Some(close_bracket) = self.bump_if_eq(tk::close_bracket) {
            return Ok(self.add_expr(ArrayExpr {
                span: open_bracket.span.to(close_bracket.span),
                elems: smallvec![],
                has_trailing_comma: false,
            }));
        };

        let first_elem = self.parse_expr()?;
        let next_token = self.bump();

        match next_token.kind {
            tk::close_bracket => {
                return Ok(self.add_expr(ArrayExpr {
                    span: open_bracket.span.to(next_token.span),
                    elems: smallvec![first_elem],
                    has_trailing_comma: false,
                }));
            }
            tk::semicolon => {
                let elem = self.parse_expr()?;
                let close_bracket = self.bump_expect(&tk::close_bracket)?;

                return Ok(self.add_expr(ArrayRepeatExpr {
                    span: open_bracket.span.to(close_bracket.span),
                    len: first_elem,
                    elem,
                }));
            }
            tk::comma => (),
            _ => return parse_error(next_token, &[tk::close_bracket, tk::semicolon, tk::comma]),
        }

        let mut elems = smallvec![first_elem];

        let (close_bracket, has_trailing_comma) =
            if let Some(close_bracket) = self.bump_if_eq(tk::close_bracket) {
                (close_bracket, true)
            } else {
                loop {
                    elems.push(self.parse_expr()?);
                    let next_token = self.bump();

                    match next_token.kind {
                        tk::close_bracket => {
                            break (next_token, false);
                        }
                        tk::comma => {
                            if let Some(close_bracket) = self.bump_if_eq(tk::close_bracket) {
                                break (close_bracket, true);
                            }
                        }
                        _ => return parse_error(next_token, &[tk::close_bracket, tk::comma]),
                    }
                }
            };

        Ok(self.add_expr(ArrayExpr {
            span: open_bracket.span.to(close_bracket.span),
            elems,
            has_trailing_comma,
        }))
    }
}
