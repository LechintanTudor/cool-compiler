use crate::expr::ExprAst;
use crate::{AstGenerator, AstResult, ResolveAst};
use cool_resolve::TyId;

#[derive(Clone, Debug)]
pub struct ConstItemAst {
    pub expr: ExprAst,
}

impl ResolveAst for ConstItemAst {
    fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        self.expr.resolve(ast, expected_ty)
    }
}
