use crate::{AssignOp, Expr, ParseResult, Parser, Stmt};
use cool_derive::Section;
use cool_lexer::{tk, TokenKind};
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum ExprOrStmt {
    Expr(Expr),
    Stmt(Stmt),
}

impl Parser<'_> {
    pub fn parse_expr_or_stmt(&mut self) -> ParseResult<ExprOrStmt> {
        match self.peek().kind {
            tk::kw_break => {
                return self
                    .parse_break_stmt()
                    .map(|stmt| ExprOrStmt::Stmt(stmt.into()));
            }
            tk::kw_continue => {
                return self
                    .parse_continue_stmt()
                    .map(|stmt| ExprOrStmt::Stmt(stmt.into()));
            }
            tk::kw_defer => {
                return self
                    .parse_defer_stmt()
                    .map(|stmt| ExprOrStmt::Stmt(stmt.into()));
            }
            tk::kw_mut => {
                return self
                    .parse_decl_stmt()
                    .map(|stmt| ExprOrStmt::Stmt(stmt.into()));
            }
            tk::kw_return => {
                return self
                    .parse_return_stmt()
                    .map(|stmt| ExprOrStmt::Stmt(stmt.into()));
            }
            _ => (),
        }

        let expr = self.parse_expr()?;

        if let Expr::Ident(ident) = &expr {
            if self.peek().kind == tk::colon {
                return self
                    .continue_parse_decl_stmt((*ident).into())
                    .map(|decl_stmt| ExprOrStmt::Stmt(decl_stmt.into()));
            }
        }

        if let TokenKind::Punct(punct) = self.peek().kind {
            if let Ok(assign_op) = AssignOp::try_from(punct) {
                self.bump();

                return self
                    .continue_parse_assign_stmt(expr, assign_op)
                    .map(|assign_stmt| ExprOrStmt::Stmt(assign_stmt.into()));
            }
        }

        Ok(ExprOrStmt::Expr(expr))
    }
}
