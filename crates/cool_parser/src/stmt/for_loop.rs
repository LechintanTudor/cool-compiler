use crate::{BlockExpr, DeclStmt, Expr, ExprOrStmt, ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ForLoop {
    pub span: Span,
    pub decl: Box<DeclStmt>,
    pub cond: Box<Expr>,
    pub after: Box<ExprOrStmt>,
    pub body: Box<BlockExpr>,
}

impl Section for ForLoop {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_for_loop(&mut self) -> ParseResult<ForLoop> {
        let start_token = self.bump_expect(&tk::KW_FOR)?;

        let decl = self.parse_decl_stmt()?;
        self.bump_expect(&tk::SEMICOLON)?;

        let cond = self.parse_expr()?;
        self.bump_expect(&tk::SEMICOLON)?;

        let after = self.parse_bare_expr_or_stmt(true, false)?;
        let body = self.parse_block_expr()?;

        Ok(ForLoop {
            span: start_token.span.to(body.span()),
            decl: Box::new(decl),
            cond: Box::new(cond),
            after: Box::new(after),
            body: Box::new(body),
        })
    }
}
