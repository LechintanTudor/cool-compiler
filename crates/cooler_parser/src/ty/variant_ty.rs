use crate::{ParseResult, Parser, TyId};
use cool_collections::smallvec::smallvec;
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct VariantTy {
    pub span: Span,
    pub variant_tys: SmallVec<TyId, 4>,
}

impl Parser<'_> {
    pub fn continue_parse_variant_ty(&mut self, first_ty: TyId) -> ParseResult<TyId> {
        debug_assert_eq!(self.peek().kind, tk::or);
        let mut variant_tys = smallvec![first_ty];
        let mut last_ty = first_ty;

        while self.bump_if_eq(tk::or).is_some() {
            last_ty = self.parse_non_variant_ty()?;
            variant_tys.push(last_ty);
        }

        let start_span = self[first_ty].span();
        let end_span = self[last_ty].span();

        Ok(self.add_ty(VariantTy {
            span: start_span.to(end_span),
            variant_tys,
        }))
    }
}
