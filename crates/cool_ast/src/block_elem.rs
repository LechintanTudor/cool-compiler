use crate::expr::ExprAst;
use crate::stmt::StmtAst;
use crate::Unify;
use cool_resolve::expr_ty::ExprTyUnifier;
use cool_resolve::ty::TyTable;

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}

impl Unify for BlockElemAst {
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable) {
        match self {
            Self::Expr(expr) => expr.unify(unifier, tys),
            Self::Stmt(stmt) => stmt.unify(unifier, tys),
        }
    }
}
