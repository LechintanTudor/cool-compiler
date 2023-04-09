use crate::expr::ExprAst;
use crate::{AstGenerator, AstResult, ResolveAst};
use cool_resolve::TyId;

#[derive(Clone, Debug)]
pub struct ConstItemAst {
    pub expr: ExprAst,
}

impl ResolveAst for ConstItemAst {
    #[inline]
    fn resolve_consts(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        self.expr.resolve_consts(ast, expected_ty)
    }

    #[inline]
    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        self.expr.resolve_exprs(ast, expected_ty)
    }
}
