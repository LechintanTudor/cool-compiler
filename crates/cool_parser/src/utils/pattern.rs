use crate::{Ident, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub struct Pattern {
    pub span: Span,
    pub is_mutable: bool,
    pub ident: Ident,
}

impl From<Ident> for Pattern {
    #[inline]
    fn from(ident: Ident) -> Self {
        Self {
            span: ident.span,
            is_mutable: false,
            ident,
        }
    }
}

impl Parser<'_> {
    pub fn parse_pattern(&mut self) -> ParseResult<Pattern> {
        let mutable_token = self.bump_if_eq(tk::kw_mut);
        let ident = self.parse_ident()?;
        let start_span = mutable_token.map(|token| token.span).unwrap_or(ident.span);

        Ok(Pattern {
            span: start_span.to(ident.span),
            is_mutable: mutable_token.is_some(),
            ident,
        })
    }
}
