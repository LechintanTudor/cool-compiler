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

        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
            return Ok(BlockExpr {
                span: open_brace.span.to(close_brace.span),
                stmts: Vec::new(),
                expr: None,
            });
        }

        let mut stmts = Vec::<Stmt>::new();

        let (close_brace, expr) = loop {
            match self.parse_expr_or_stmt()? {
                ExprOrStmt::Expr(expr) => {
                    if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                        break (close_brace, Some(Box::new(expr)));
                    }

                    if self.bump_if_eq(tk::semicolon).is_none()
                        && !is_expr_promotable_to_stmt(&expr)
                    {
                        return self.peek_error(&[tk::close_brace]);
                    }

                    stmts.push(Stmt::Expr(Box::new(expr)));
                }
                ExprOrStmt::Stmt(stmt) => {
                    stmts.push(stmt);

                    if self.bump_if_eq(tk::semicolon).is_none() {
                        let Some(close_brace) = self.bump_if_eq(tk::close_brace) else {
                            return self.peek_error(&[tk::close_brace]);
                        };

                        break (close_brace, None);
                    }

                    if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                        break (close_brace, None);
                    }
                }
            }
        };

        Ok(BlockExpr {
            span: open_brace.span.to(close_brace.span),
            stmts,
            expr,
        })
    }
}

#[must_use]
fn is_expr_promotable_to_stmt(expr: &Expr) -> bool {
    matches!(expr, Expr::Block(_))
}
