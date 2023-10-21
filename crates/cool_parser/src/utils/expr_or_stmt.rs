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
        if self.peek().kind == tk::kw_mut {
            return self
                .parse_decl_stmt()
                .map(|decl_stmt| ExprOrStmt::Stmt(decl_stmt.into()));
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
                return self
                    .continue_parse_assign_stmt(expr, assign_op)
                    .map(|assign_stmt| ExprOrStmt::Stmt(assign_stmt.into()));
            }
        }

        Ok(ExprOrStmt::Expr(expr))
    }
}
