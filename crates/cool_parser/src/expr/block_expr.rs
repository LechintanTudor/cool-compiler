use crate::{Expr, ExprOrStmt, ParseResult, Parser, Stmt};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct BlockExpr {
    pub span: Span,
    pub stmts: Vec<BlockExprStmt>,
    pub expr: Option<Box<Expr>>,
}

#[derive(Clone, Section, Debug)]
pub struct BlockExprStmt {
    pub span: Span,
    pub stmt: Stmt,
    pub has_semicolon: bool,
}

impl Parser<'_> {
    pub fn parse_block_expr(&mut self) -> ParseResult<BlockExpr> {
        let open_brace = self.bump_expect(&tk::open_brace)?;
        let mut stmts = Vec::<BlockExprStmt>::new();

        let (close_brace, expr) = if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
            (close_brace, None)
        } else {
            loop {
                match self.parse_expr_or_stmt()? {
                    ExprOrStmt::Expr(expr) => {
                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, Some(Box::new(expr)));
                        }

                        let (span, has_semicolon) =
                            if let Some(semicolon) = self.bump_if_eq(tk::semicolon) {
                                (expr.span().to(semicolon.span), true)
                            } else if is_expr_promotable_to_stmt(&expr) {
                                (expr.span(), false)
                            } else {
                                return self.peek_error(&[tk::close_brace]);
                            };

                        stmts.push(BlockExprStmt {
                            span,
                            stmt: Stmt::Expr(Box::new(expr)),
                            has_semicolon,
                        });

                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, None);
                        }
                    }
                    ExprOrStmt::Stmt(stmt) => {
                        let (span, has_semicolon) =
                            if let Some(semicolon) = self.bump_if_eq(tk::semicolon) {
                                (stmt.span().to(semicolon.span), true)
                            } else {
                                (stmt.span(), false)
                            };

                        stmts.push(BlockExprStmt {
                            span,
                            stmt,
                            has_semicolon,
                        });

                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, None);
                        } else if !has_semicolon {
                            return self.peek_error(&[tk::close_brace]);
                        }
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

#[inline]
#[must_use]
fn is_expr_promotable_to_stmt(expr: &Expr) -> bool {
    matches!(expr, Expr::Block(_))
}
