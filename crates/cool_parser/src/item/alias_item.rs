use crate::{ParseResult, Parser, Ty};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct AliasItem {
    pub span: Span,
    pub ty: Box<Ty>,
}

impl Parser<'_> {
    pub fn parse_alias_item(&mut self) -> ParseResult<AliasItem> {
        let alias_token = self.bump_expect(&tk::kw_alias)?;
        let ty = self.parse_ty()?;

        Ok(AliasItem {
            span: alias_token.span.to(ty.span()),
            ty: Box::new(ty),
        })
    }
}
