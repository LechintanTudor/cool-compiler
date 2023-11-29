use crate::{ExprId, ExprOrStmt, ParseResult, Parser, StmtId};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Section, Debug)]
pub struct BlockExpr {
    pub span: Span,
    pub stmts: SmallVec<BlockExprStmt, 1>,
    pub end_expr: Option<ExprId>,
}

#[derive(Clone, Section, Debug)]
pub struct BlockExprStmt {
    pub span: Span,
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
                match self.parse_expr_or_stmt()? {
                    ExprOrStmt::Expr(expr) => {
                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, Some(expr));
                        }

                        let (span, has_semicolon) =
                            if let Some(semicolon) = self.bump_if_eq(tk::semicolon) {
                                (self.data.exprs[expr].span().to(semicolon.span), true)
                            } else if self.data.exprs[expr].is_promotable_to_stmt() {
                                (self.data.exprs[expr].span(), false)
                            } else {
                                return self.peek_error(&[tk::close_brace]);
                            };

                        let stmt = self.continue_parse_expr_stmt(expr);

                        stmts.push(BlockExprStmt {
                            span,
                            stmt,
                            has_semicolon,
                        });

                        if let Some(close_brace) = self.bump_if_eq(tk::close_brace) {
                            break (close_brace, None);
                        }
                    }
                    ExprOrStmt::Stmt(stmt) => {
                        let (span, has_semicolon) =
                            if let Some(semicolon) = self.bump_if_eq(tk::semicolon) {
                                (self.data.stmts[stmt].span().to(semicolon.span), true)
                            } else {
                                (self.data.stmts[stmt].span(), false)
                            };

                        stmts.push(BlockExprStmt {
                            span,
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

        Ok(self.data.exprs.push(
            BlockExpr {
                span: open_brace.span.to(close_brace.span),
                stmts,
                end_expr,
            }
            .into(),
        ))
    }
}
