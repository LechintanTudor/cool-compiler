use crate::expr::ExprAst;
use crate::{AstGenerator, ResolveAst, SemanticResult};
use cool_resolve::ty::TyId;

#[derive(Clone, Debug)]
pub struct ConstItemAst {
    pub expr: ExprAst,
}

impl ResolveAst for ConstItemAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> SemanticResult<TyId> {
        self.expr.resolve(ast, expected_ty)
    }
}
