use crate::expr::ExprAst;
use crate::stmt::StmtAst;

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}
