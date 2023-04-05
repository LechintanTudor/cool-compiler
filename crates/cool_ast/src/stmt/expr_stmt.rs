use crate::expr::ExprAst;
use crate::{AstGenerator, ResolveAst, SemanticResult};
use cool_resolve::ty::{tys, TyId};

#[derive(Clone, Debug)]
pub struct ExprStmtAst {
    pub expr: ExprAst,
}

impl ResolveAst for ExprStmtAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: Option<TyId>) -> SemanticResult<TyId> {
        self.expr.resolve(ast, expected_ty)?;
        Ok(tys::UNIT)
    }
}
