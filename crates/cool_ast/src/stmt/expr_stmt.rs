use crate::expr::ExprAst;
use crate::{AstGenerator, ResolveAst, SemanticResult, Unify};
use cool_resolve::expr_ty::ExprTyUnifier;
use cool_resolve::ty::{tys, TyId, TyTable};

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

impl ResolveAst for ExprStmtAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: Option<TyId>) -> SemanticResult<TyId> {
        self.expr.resolve(ast, expected_ty)?;
        Ok(tys::UNIT)
    }
}
