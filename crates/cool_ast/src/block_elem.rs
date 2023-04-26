use crate::{ExprAst, StmtAst};

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}

impl From<ExprAst> for BlockElemAst {
    #[inline]
    fn from(expr: ExprAst) -> Self {
        Self::Expr(expr)
    }
}

impl From<StmtAst> for BlockElemAst {
    #[inline]
    fn from(stmt: StmtAst) -> Self {
        Self::Stmt(stmt)
    }
}
