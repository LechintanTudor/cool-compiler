mod assign_stmt;
mod break_stmt;
mod continue_stmt;
mod decl_stmt;
mod defer_stmt;
mod expr_stmt;
mod return_stmt;

pub use self::assign_stmt::*;
pub use self::break_stmt::*;
pub use self::continue_stmt::*;
pub use self::decl_stmt::*;
pub use self::defer_stmt::*;
pub use self::expr_stmt::*;
pub use self::return_stmt::*;

use crate::{AssignOp, Expr, ExprId, ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::tk;
use derive_more::{Debug, From};

define_index_newtype!(StmtId);

#[derive(Clone, Section, From, Debug)]
pub enum Stmt {
    Assign(AssignStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Decl(DeclStmt),
    Defer(DeferStmt),
    Expr(ExprStmt),
    Return(ReturnStmt),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum ExprOrStmt {
    Expr(ExprId),
    Stmt(StmtId),
}

impl Parser<'_> {
    pub fn parse_stmt(&mut self, allow_struct: bool) -> ParseResult<StmtId> {
        let stmt = match self.parse_expr_or_stmt(allow_struct)? {
            ExprOrStmt::Expr(expr) => self.continue_parse_expr_stmt(expr),
            ExprOrStmt::Stmt(stmt) => stmt,
        };

        Ok(stmt)
    }

    pub fn parse_expr_or_stmt(&mut self, allow_struct: bool) -> ParseResult<ExprOrStmt> {
        let code = match self.peek().kind {
            tk::kw_break => self.parse_break_stmt()?.into(),
            tk::kw_continue => self.parse_continue_stmt()?.into(),
            tk::kw_defer => self.parse_defer_stmt()?.into(),
            tk::kw_return => self.parse_return_stmt()?.into(),
            _ => self.parse_expr_full(allow_struct)?.into(),
        };

        if let ExprOrStmt::Expr(expr_id) = code {
            let peeked_token = self.peek().kind;

            if let Expr::Ident(ident) = &self[expr_id] {
                if peeked_token == tk::colon {
                    return self
                        .continue_parse_decl_stmt((*ident).into(), allow_struct)
                        .map(ExprOrStmt::Stmt);
                }
            }

            if AssignOp::try_from(peeked_token).is_ok() {
                return self
                    .continue_parse_assign_stmt(expr_id, allow_struct)
                    .map(ExprOrStmt::Stmt);
            }
        }

        Ok(code)
    }
}
