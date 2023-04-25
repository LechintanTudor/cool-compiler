mod assign_stmt;
mod decl_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
use crate::ExprAst;

#[derive(Clone, Debug)]
pub enum StmtAst {
    Assign(AssignStmtAst),
    Decl(DeclStmtAst),
    Expr(ExprAst),
}
