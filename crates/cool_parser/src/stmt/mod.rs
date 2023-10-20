mod assign_stmt;
mod decl_stmt;
mod expr_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
pub use self::expr_stmt::*;

use cool_derive::Section;
use derive_more::From;

#[derive(Clone, From, Section, Debug)]
pub enum Stmt {
    Assign(AssignStmt),
    Decl(DeclStmt),
    Expr(ExprStmt),
}
