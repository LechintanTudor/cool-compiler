use crate::expr::ExprAst;

#[derive(Clone, Debug)]
pub struct ExprStmtAst {
    pub expr: ExprAst,
}
