mod assign_stmt;
mod decl_stmt;
mod defer_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
pub use self::defer_stmt::*;

use crate::{Expr, ExprOrStmt};
use cool_derive::Section;
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Stmt {
    Assign(AssignStmt),
    Decl(DeclStmt),
    Defer(DeferStmt),
    Expr(Box<Expr>),
}

impl From<ExprOrStmt> for Stmt {
    fn from(value: ExprOrStmt) -> Self {
        match value {
            ExprOrStmt::Expr(expr) => Self::Expr(Box::new(expr)),
            ExprOrStmt::Stmt(stmt) => stmt,
        }
    }
}
