use crate::{Ident, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
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

        let (span, is_mutable) = match mutable_token {
            Some(token) => (token.span.to(ident.span), true),
            None => (ident.span, false),
        };

        Ok(Pattern {
            span,
            is_mutable,
            ident,
        })
    }
}
