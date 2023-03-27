use crate::path::IdentPath;
use crate::{ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct UseDecl {
    pub span: Span,
    pub path: IdentPath,
}

impl ParseTree for UseDecl {
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_use_decl(&mut self) -> ParseResult<UseDecl> {
        let start_token = self.bump_expect(&tk::KW_USE)?;
        let path = self.parse_import_path()?;

        Ok(UseDecl {
            span: start_token.span.to(path.span()),
            path,
        })
    }
}
