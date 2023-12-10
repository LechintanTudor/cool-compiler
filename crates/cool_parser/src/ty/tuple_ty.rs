use crate::{parse_error, ParseResult, Parser, TyId};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ParenTy {
    pub span: Span,
    pub inner_ty: TyId,
}

#[derive(Clone, Section, Debug)]
pub struct TupleTy {
    pub span: Span,
    pub elem_tys: SmallVec<TyId, 4>,
    pub has_trailing_comma: bool,
}

impl Parser<'_> {
    pub fn parse_paren_or_tuple_ty(&mut self) -> ParseResult<TyId> {
        let open_paren = self.bump_expect(&tk::open_paren)?;
        let mut elem_tys = SmallVec::new();

        if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
            return Ok(self.add_ty(TupleTy {
                span: open_paren.span.to(close_paren.span),
                elem_tys,
                has_trailing_comma: false,
            }));
        }

        let (close_paren, has_trailing_comma) = loop {
            let elem_ty = self.parse_ty()?;

            if elem_tys.is_empty() {
                if let Some(close_paren) = self.bump_if_eq(tk::close_paren) {
                    return Ok(self.add_ty(ParenTy {
                        span: open_paren.span.to(close_paren.span),
                        inner_ty: elem_ty,
                    }));
                }
            }

            elem_tys.push(elem_ty);
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

        Ok(self.add_ty(TupleTy {
            span: open_paren.span.to(close_paren.span),
            elem_tys,
            has_trailing_comma,
        }))
    }
}
