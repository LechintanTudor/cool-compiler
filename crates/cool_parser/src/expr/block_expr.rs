use crate::{Expr, ExprOrStmt, ParseResult, Parser, Stmt};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct BlockExpr {
    pub span: Span,
    pub stmts: Vec<Stmt>,
    pub expr: Option<Box<Expr>>,
}

impl Section for BlockExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_block_expr(&mut self) -> ParseResult<BlockExpr> {
        let open_brace = self.bump_expect(&tk::OPEN_BRACE)?;
        let mut stmts = Vec::<Stmt>::new();

        let (end_brace, expr) = loop {
            if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
                break (end_token, None);
            }

            match self.parse_bare_expr_or_stmt(true, true)? {
                ExprOrStmt::Expr(expr) => {
                    if let Some(end_token) = self.bump_if_eq(tk::CLOSE_BRACE) {
                        break (end_token, Some(expr));
                    } else {
                        stmts.push(self.continue_parse_stmt(Box::new(expr).into())?);
                    }
                }
                ExprOrStmt::Stmt(stmt) => {
                    stmts.push(self.continue_parse_stmt(stmt)?);
                }
            }
        };

        Ok(BlockExpr {
            span: open_brace.span.to(end_brace.span),
            stmts,
            expr: expr.map(Box::new),
        })
    }
}
