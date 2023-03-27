use crate::{ParseResult, ParseTree, Parser, Ty};
use cool_lexer::tokens::tk;
use cool_span::Span;

// TODO: Check for trailing comma
#[derive(Clone, Debug)]
pub struct TupleTy {
    pub span: Span,
    pub elems: Vec<Ty>,
}

impl ParseTree for TupleTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_tuple_ty(&mut self) -> ParseResult<TupleTy> {
        let open_paren = self.bump_expect(&tk::OPEN_PAREN)?;
        let mut elems = Vec::<Ty>::new();

        let closed_paren = if self.peek().kind == tk::CLOSE_PAREN {
            self.bump()
        } else {
            loop {
                let element = self.parse_ty()?;
                elems.push(element);

                let next_token = self.bump();

                match next_token.kind {
                    tk::COMMA => {
                        if self.peek().kind == tk::CLOSE_PAREN {
                            break self.bump();
                        }
                    }
                    tk::CLOSE_PAREN => {
                        break next_token;
                    }
                    _ => self.error(next_token, &[tk::COMMA, tk::CLOSE_PAREN])?,
                }
            }
        };

        Ok(TupleTy {
            span: open_paren.span.to(closed_paren.span),
            elems,
        })
    }
}
