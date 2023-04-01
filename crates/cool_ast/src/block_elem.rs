use crate::expr::ExprAst;
use crate::stmt::StmtAst;
use crate::{AstGenerator, Unify};

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}

impl Unify for BlockElemAst {
    fn unify(&self, gen: &mut AstGenerator) {
        match self {
            Self::Expr(expr) => expr.unify(gen),
            Self::Stmt(stmt) => stmt.unify(gen),
        }
    }
}
