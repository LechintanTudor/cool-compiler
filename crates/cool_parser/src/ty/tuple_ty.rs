use crate::{ParseResult, Parser, Ty};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct TupleTy {
    pub span: Span,
    pub elems: Vec<Ty>,
    pub has_trailing_comma: bool,
}

impl Section for TupleTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_tuple_ty(&mut self) -> ParseResult<TupleTy> {
        let start_token = self.bump_expect(&tk::OPEN_PAREN)?;

        let mut elems = Vec::<Ty>::new();

        let (end_token, has_trailing_comma) = match self.bump_if_eq(tk::CLOSE_PAREN) {
            Some(end_token) => (end_token, false),
            None => {
                loop {
                    elems.push(self.parse_ty()?);

                    let next_token = self.bump();

                    match next_token.kind {
                        tk::COMMA => {
                            if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                                break (end_token, true);
                            }
                        }
                        tk::CLOSE_PAREN => {
                            if elems.len() == 1 {
                                return self.error(next_token, &[tk::COMMA]);
                            }

                            break (next_token, false);
                        }
                        _ => return self.error(next_token, &[tk::COMMA, tk::CLOSE_PAREN]),
                    }
                }
            }
        };

        Ok(TupleTy {
            span: start_token.span.to(end_token.span),
            elems,
            has_trailing_comma,
        })
    }
}
