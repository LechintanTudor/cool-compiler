use crate::expr::ExprKindAst;

#[derive(Clone, Debug)]
pub struct ExprStmtAst {
    pub expr: ExprKindAst,
}
