mod break_stmt;
mod continue_stmt;
mod expr_stmt;
mod return_stmt;

pub use self::break_stmt::*;
pub use self::continue_stmt::*;
pub use self::expr_stmt::*;
pub use self::return_stmt::*;

use crate::{ExprId, ParseResult, Parser};
use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_lexer::tk;
use derive_more::{Debug, From};

define_index_newtype!(StmtId);

#[derive(Clone, Section, From, Debug)]
pub enum Stmt {
    Break(BreakStmt),
    Continue(ContinueStmt),
    Expr(ExprStmt),
    Return(ReturnStmt),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
pub enum ExprOrStmt {
    Expr(ExprId),
    Stmt(StmtId),
}

impl Parser<'_> {
    pub fn parse_expr_or_stmt(&mut self) -> ParseResult<ExprOrStmt> {
        let expr_or_stmt = match self.peek().kind {
            tk::kw_break => self.parse_break_stmt()?.into(),
            tk::kw_continue => self.parse_continue_stmt()?.into(),
            tk::kw_return => self.parse_return_stmt()?.into(),
            _ => self.parse_expr()?.into(),
        };

        Ok(expr_or_stmt)
    }
}
