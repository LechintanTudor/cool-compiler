use crate::{BlockExpr, DeclStmt, Expr, ParseResult, Parser, Stmt};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ForExpr {
    pub span: Span,
    pub decl: Box<DeclStmt>,
    pub cond: Box<Expr>,
    pub post: Box<Stmt>,
    pub body: Box<BlockExpr>,
}

impl Parser<'_> {
    pub fn parse_for_expr(&mut self) -> ParseResult<ForExpr> {
        let for_token = self.bump_expect(&tk::kw_for)?;

        let decl = self.parse_decl_stmt()?;
        self.bump_expect(&tk::semicolon)?;

        let cond = self.parse_expr()?;
        self.bump_expect(&tk::semicolon)?;

        let post: Stmt = self.parse_expr_or_stmt()?.into();
        let body = self.parse_block_expr()?;

        Ok(ForExpr {
            span: for_token.span.to(body.span),
            decl: Box::new(decl),
            cond: Box::new(cond),
            post: Box::new(post),
            body: Box::new(body),
        })
    }
}
