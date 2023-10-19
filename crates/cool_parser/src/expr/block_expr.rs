use crate::{Expr, ExprOrStmt, ParseResult, Parser, Stmt};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct BlockExpr {
    pub span: Span,
    pub stmts: Vec<Stmt>,
    pub expr: Option<Box<Expr>>,
}

impl Parser<'_> {
    pub fn parse_block_expr(&mut self) -> ParseResult<BlockExpr> {
        let open_brace = self.bump_expect(&tk::open_brace)?;
        let mut stmts = Vec::<Stmt>::new();
        let mut last_expr = Option::<Expr>::None;

        let close_brace = loop {
            if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                break close_brace;
            }

            match self.parse_expr_or_stmt()? {
                ExprOrStmt::Expr(expr) => last_expr = Some(expr),
                ExprOrStmt::Stmt(stmt) => stmts.push(stmt),
            }

            if self.bump_if_eq(tk::semicolon).is_some() {
                if let Some(expr) = last_expr.take() {
                    stmts.push(Stmt::Expr(Box::new(expr)));
                }
            }
        };

        Ok(BlockExpr {
            span: open_brace.span.to(close_brace.span),
            stmts,
            expr: last_expr.map(Box::new),
        })
    }
}
