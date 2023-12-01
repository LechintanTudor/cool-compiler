use crate::{ExprId, ExprOrStmt, ParseResult, Parser, Stmt, StmtId};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct BlockExpr {
    pub span: Span,
    pub stmts: SmallVec<BlockExprStmt, 2>,
    pub end_expr: Option<ExprId>,
}

#[derive(Clone, Debug)]
pub struct BlockExprStmt {
    pub stmt: StmtId,
    pub has_semicolon: bool,
}

impl Parser<'_> {
    pub fn parse_block_expr(&mut self) -> ParseResult<ExprId> {
        let open_brace = self.bump_expect(&tk::open_brace)?;
        let mut stmts = SmallVec::new();

        let (close_brace, end_expr) = if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
            (close_brace, None)
        } else {
            loop {
                match self.parse_expr_or_stmt(true)? {
                    ExprOrStmt::Expr(expr) => {
                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, Some(expr));
                        }

                        let has_semicolon = if self.bump_if_eq(tk::semicolon).is_some() {
                            true
                        } else if self[expr].is_promotable_to_stmt() {
                            false
                        } else {
                            return self.peek_error(&[tk::close_brace]);
                        };

                        stmts.push(BlockExprStmt {
                            stmt: self.continue_parse_expr_stmt(expr),
                            has_semicolon,
                        });

                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, None);
                        }
                    }
                    ExprOrStmt::Stmt(stmt) => {
                        let has_semicolon = if self.bump_if_eq(tk::semicolon).is_some() {
                            true
                        } else if !self.stmt_needs_semicolon(stmt) {
                            false
                        } else {
                            return self.peek_error(&[tk::semicolon]);
                        };

                        stmts.push(BlockExprStmt {
                            stmt,
                            has_semicolon,
                        });

                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, None);
                        } else if !has_semicolon {
                            return self.peek_error(&[tk::semicolon, tk::close_brace]);
                        }
                    }
                }
            }
        };

        Ok(self.add_expr(BlockExpr {
            span: open_brace.span.to(close_brace.span),
            stmts,
            end_expr,
        }))
    }

    fn stmt_needs_semicolon(&self, stmt_id: StmtId) -> bool {
        let mut code = ExprOrStmt::Stmt(stmt_id);

        loop {
            match code {
                ExprOrStmt::Expr(expr_id) => {
                    break !self[expr_id].is_promotable_to_stmt();
                }
                ExprOrStmt::Stmt(stmt_id) => {
                    match &self[stmt_id] {
                        Stmt::Defer(defer_stmt) => {
                            code = defer_stmt.code;
                        }
                        _ => break true,
                    }
                }
            }
        }
    }
}
