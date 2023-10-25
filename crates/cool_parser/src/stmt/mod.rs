mod assign_stmt;
mod break_stmt;
mod continue_stmt;
mod decl_stmt;
mod defer_stmt;
mod return_stmt;

pub use self::assign_stmt::*;
pub use self::break_stmt::*;
pub use self::continue_stmt::*;
pub use self::decl_stmt::*;
pub use self::defer_stmt::*;
pub use self::return_stmt::*;

use crate::{Expr, ExprOrStmt};
use cool_derive::Section;
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Stmt {
    Assign(AssignStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    Decl(DeclStmt),
    Defer(DeferStmt),
    Expr(Box<Expr>),
    Return(ReturnStmt),
}

impl From<ExprOrStmt> for Stmt {
    #[inline]
    fn from(value: ExprOrStmt) -> Self {
        match value {
            ExprOrStmt::Expr(expr) => Self::Expr(Box::new(expr)),
            ExprOrStmt::Stmt(stmt) => stmt,
        }
    }
}
