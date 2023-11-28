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

        while self.bump_if_eq(tk::or).is_some() {
            variant_tys.push(self.parse_non_variant_ty()?);
        }

        let start_span = self.data.tys.first().unwrap().span();
        let end_span = self.data.tys.last().unwrap().span();

        Ok(self.data.tys.push(
            VariantTy {
                span: start_span.to(end_span),
                variant_tys,
            }
            .into(),
        ))
    }
}
