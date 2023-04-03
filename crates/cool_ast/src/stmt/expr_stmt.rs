use crate::expr::ExprAst;
use crate::Unify;
use cool_resolve::expr_ty::ExprTyUnifier;
use cool_resolve::ty::TyTable;

#[derive(Clone, Debug)]
pub struct ExprStmtAst {
    pub expr: ExprAst,
}

impl Unify for ExprStmtAst {
    #[inline]
    fn unify(&self, unifier: &mut ExprTyUnifier, tys: &mut TyTable) {
        self.expr.unify(unifier, tys);
    }
}
