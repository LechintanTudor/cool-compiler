mod decl_stmt;
mod expr_stmt;

pub use self::decl_stmt::*;
pub use self::expr_stmt::*;

#[derive(Clone, Debug)]
pub enum StmtAst {
    Decl(DeclStmtAst),
    Expr(ExprStmtAst),
}
