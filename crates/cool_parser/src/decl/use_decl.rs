use crate::path::IdentPath;
use crate::{Ident, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct UseDecl {
    pub span: Span,
    pub path: IdentPath,
    pub alias: Option<Ident>,
}

impl Section for UseDecl {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_use_decl(&mut self) -> ParseResult<UseDecl> {
        let start_token = self.bump_expect(&tk::KW_USE)?;
        let path = self.parse_import_path()?;

        let (alias, end_span) = if self.bump_if_eq(tk::KW_AS).is_some() {
            let alias = self.parse_ident()?;
            (Some(alias), alias.span())
        } else {
            (None, path.span())
        };

        Ok(UseDecl {
            span: start_token.span.to(end_span),
            path,
            alias,
        })
    }
}
