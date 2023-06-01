use crate::{ParseResult, Parser, Ty};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct AliasItem {
    pub span: Span,
    pub ty: Ty,
}

impl Section for AliasItem {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_alias_item(&mut self) -> ParseResult<AliasItem> {
        let start_token = self.bump_expect(&tk::KW_ALIAS)?;
        let ty = self.parse_ty()?;

        Ok(AliasItem {
            span: start_token.span.to(ty.span()),
            ty,
        })
    }
}
