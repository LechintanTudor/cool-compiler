use crate::{ParseResult, ParseTree, Parser, Ty};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct PointerTy {
    pub span: Span,
    pub is_mutable: bool,
    pub inner_ty: Box<Ty>,
}

impl ParseTree for PointerTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_pointer_ty(&mut self) -> ParseResult<PointerTy> {
        let start_token = self.bump_expect(&tk::STAR)?;
        let is_mutable = self.bump_if_eq(tk::KW_MUT).is_some();
        let inner_ty = Box::new(self.parse_ty()?);

        Ok(PointerTy {
            span: start_token.span.to(inner_ty.span()),
            is_mutable,
            inner_ty,
        })
    }
}
