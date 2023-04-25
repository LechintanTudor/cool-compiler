use crate::{ExprAst, StmtAst};

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}
