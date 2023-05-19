use crate::{ParseResult, Parser, Ty};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct PtrTy {
    pub span: Span,
    pub is_mutable: bool,
    pub pointee: Box<Ty>,
}

impl Section for PtrTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_ptr_ty(&mut self) -> ParseResult<PtrTy> {
        let start_token = self.bump_expect(&tk::STAR)?;
        let is_mutable = self.bump_if_eq(tk::KW_MUT).is_some();
        let pointee = Box::new(self.parse_ty()?);

        Ok(PtrTy {
            span: start_token.span.to(pointee.span()),
            is_mutable,
            pointee,
        })
    }
}
