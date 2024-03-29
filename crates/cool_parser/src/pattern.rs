use crate::{Ident, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct Pattern {
    pub span: Span,
    pub is_mutable: bool,
    pub ident: Ident,
}

impl Section for Pattern {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl From<Ident> for Pattern {
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
        let (start_span, is_mutable, ident) = match self.bump_if_eq(tk::KW_MUT) {
            Some(start_token) => {
                let ident = self.parse_ident()?;
                (start_token.span, true, ident)
            }
            None => {
                let ident = self.parse_ident()?;
                (ident.span, false, ident)
            }
        };

        Ok(Pattern {
            span: start_span.to(ident.span),
            is_mutable,
            ident,
        })
    }
}
