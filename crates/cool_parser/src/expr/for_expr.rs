use crate::{BareBlockElem, BlockExpr, DeclStmt, Expr, ParseResult, Parser};
use cool_lexer::tokens::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ForExpr {
    pub span: Span,
    pub decl: Box<DeclStmt>,
    pub cond: Box<Expr>,
    pub after: Box<BareBlockElem>,
    pub body: Box<BlockExpr>,
}

impl Section for ForExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_for_expr(&mut self) -> ParseResult<ForExpr> {
        let start_token = self.bump_expect(&tk::KW_FOR)?;

        let decl = self.parse_decl_stmt()?;
        self.bump_expect(&tk::SEMICOLON)?;

        let cond = self.parse_expr()?;
        self.bump_expect(&tk::SEMICOLON)?;

        let after = self.parse_bare_block_elem(true, false)?;
        let body = self.parse_block_expr()?;

        Ok(ForExpr {
            span: start_token.span.to(body.span()),
            decl: Box::new(decl),
            cond: Box::new(cond),
            after: Box::new(after),
            body: Box::new(body),
        })
    }
}
