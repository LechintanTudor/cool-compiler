use crate::{ParseResult, Parser, TyId};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct PtrTy {
    pub span: Span,
    pub pointee_ty: TyId,
    pub is_mutable: bool,
}

impl Parser<'_> {
    pub fn parse_ptr_ty(&mut self) -> ParseResult<TyId> {
        let star_token = self.bump_expect(&tk::star)?;
        let is_mutable = self.bump_if_eq(tk::kw_mut).is_some();

        let pointee_ty = self.parse_non_variant_ty()?;
        let end_span = self[pointee_ty].span();

        Ok(self.add_ty(PtrTy {
            span: star_token.span.to(end_span),
            pointee_ty,
            is_mutable,
        }))
    }
}
