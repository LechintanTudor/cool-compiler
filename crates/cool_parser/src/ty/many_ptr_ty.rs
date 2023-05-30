use crate::{ParseResult, Parser, Ty};
use cool_lexer::tokens::{tk, Token};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ManyPtrTy {
    pub span: Span,
    pub is_mutable: bool,
    pub pointee: Box<Ty>,
}

impl Section for ManyPtrTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn continue_parse_many_ptr_ty(&mut self, open_bracket: Token) -> ParseResult<ManyPtrTy> {
        debug_assert_eq!(open_bracket.kind, tk::OPEN_BRACKET);
        self.bump_expect(&tk::STAR)?;
        self.bump_expect(&tk::CLOSE_BRACKET)?;

        let is_mutable = self.bump_if_eq(tk::KW_MUT).is_some();
        let pointee = self.parse_ty()?;

        Ok(ManyPtrTy {
            span: open_bracket.span.to(pointee.span()),
            is_mutable,
            pointee: Box::new(pointee),
        })
    }
}
