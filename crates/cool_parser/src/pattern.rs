use crate::{Ident, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct Pattern {
    pub span: Span,
    pub is_mutable: bool,
    pub ident: Ident,
}

impl ParseTree for Pattern {
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
