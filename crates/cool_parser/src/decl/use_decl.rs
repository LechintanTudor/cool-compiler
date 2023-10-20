use crate::{Ident, IdentPath, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct UseDecl {
    pub span: Span,
    pub path: IdentPath,
    pub alias: Option<Ident>,
}

impl Parser<'_> {
    pub fn parse_use_decl(&mut self) -> ParseResult<UseDecl> {
        let use_token = self.bump_expect(&tk::kw_use)?;
        let path = self.parse_ident_path()?;

        let alias = self
            .bump_if_eq(tk::kw_as)
            .map(|_| self.parse_ident())
            .transpose()?;

        let end_span = alias
            .as_ref()
            .map(|alias| alias.span)
            .unwrap_or(path.span());

        Ok(UseDecl {
            span: use_token.span.to(end_span),
            path,
            alias,
        })
    }
}
