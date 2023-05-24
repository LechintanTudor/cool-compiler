use crate::{BareBlockElem, ParseResult, Parser};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct DeferStmt {
    pub span: Span,
    pub elem: Box<BareBlockElem>,
}

impl Section for DeferStmt {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_defer_stmt(&mut self) -> ParseResult<DeferStmt> {
        let start_token = self.bump_expect(&tk::KW_DEFER)?;
        let elem = self.parse_bare_block_elem(false, true)?;

        Ok(DeferStmt {
            span: start_token.span.to(elem.span()),
            elem: Box::new(elem),
        })
    }
}
